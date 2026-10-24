use cortical_neuron::CorticalNeuron;

/// A single weight change produced by STDP
pub struct StdpEvent {
    pub pre_id: usize,
    pub post_id: usize,
    pub old_weight: u16,
    pub new_weight: u16,
    pub delta_t: usize,
    pub is_ltp: bool,  // true = LTP (strengthened), false = LTD (weakened)
}

/// DA modulation for three-factor learning rule.
/// Uses PHASIC reward signal (N10 ENKEPHALIN_TO_VTA fires = actual reward).
/// Tonic VTA firing does NOT gate plasticity.
///
/// If reward fired recently → full strength, decaying linearly to 0.
/// If reward never fired or fired long ago → 0 (no learning without phasic DA).
fn da_modulate(strength: u16, tic: usize, vta_phasic_last: usize, window: usize) -> u16 {
    if vta_phasic_last == 0 {
        return 0;  // no phasic DA ever → no DA-gated learning
    }
    if tic < vta_phasic_last { return 0; }
    let da_dt = tic - vta_phasic_last;
    if da_dt > window { return 0; }  // phasic DA decayed → no learning
    // Full strength at dt=0, decaying linearly to 0 at dt=window
    let factor = (window - da_dt) as u32 * 256 / window as u32; // 256..0
    let result = (strength as u32 * factor / 256) as u16;
    result
}

/// Serotonin plasticity modulation.
/// 5-HT sets the global "learning rate" — it doesn't choose WHICH synapses
/// change (that's DA's job), it controls HOW MUCH they can change.
///
/// Hierarchy: final_strength = STDP_timing × DA_reward × 5HT_regime
///
/// 5-HT1A >> 5-HT2A → suppressed plasticity (exploitation mode, "don't change")
/// 5-HT1A ≈  5-HT2A → normal plasticity
/// 5-HT2A >> 5-HT1A → enhanced plasticity (exploration mode, "rewrite allowed")
///
/// Returns a factor in 0..512 range (256 = neutral = ×1.0)
fn ht_plasticity_factor(ht1a_cached: u16, ht2a_cached: u16) -> u32 {
    // 256 = neutral. 2A enhances (adds), 1A suppresses (subtracts).
    // Clamp to 0..512 to prevent extreme swings.
    let factor = (256u32 + ht2a_cached as u32).saturating_sub(ht1a_cached as u32);
    factor.min(512)
}

/// Phase 9: Spike-Timing Dependent Plasticity (STDP)
///
/// Runs AFTER all neuromodulators are applied (phase_8).
/// Modifies synapse weights based on relative spike timing:
///
///   pre fires BEFORE post (causal, Δt > 0) → LTP (weight ↑)
///   pre fires AFTER  post (anti-causal)     → LTD (weight ↓)
///
/// Effect magnitude decays linearly with |Δt| within the STDP window.
///
/// Three modulation layers (hierarchical):
///   1. STDP timing: base strength from spike coincidence
///   2. DA gating: phasic reward selects WHICH synapses change
///   3. 5-HT regime: serotonin sets HOW MUCH change is allowed
///
/// Only neurons with stdp_enabled=true have their input weights modified.
/// last_fired_tic is updated for ALL neurons (needed as source info).
pub fn phase_9(mesh: &mut Vec<CorticalNeuron>, tic: usize, vta_last_fire: usize) -> Vec<StdpEvent> {
    let mut events: Vec<StdpEvent> = Vec::new();
    let n = mesh.len();

    // ── Snapshot current state (avoid borrow conflicts) ──
    let fired_this_tic: Vec<bool> = mesh.iter().map(|n| n.fired_this_tic).collect();
    let last_fired: Vec<usize> = mesh.iter().map(|n| n.last_fired_tic).collect();

    // ── LTP: for each post-synaptic neuron that fired this tick ──
    // Check its sources — if a source fired recently (before this tick),
    // that source helped cause this firing → strengthen the synapse.
    for post_id in 0..n {
        if !fired_this_tic[post_id] { continue; }
        if !mesh[post_id].stdp_enabled { continue; }

        let window = mesh[post_id].stdp_window as usize;
        let ltp_rate = mesh[post_id].ltp_rate as u32;
        let weight_max = mesh[post_id].weight_max;
        let da_gated = mesh[post_id].stdp_da_gated;
        let ht_mod = mesh[post_id].stdp_ht_modulated;
        let ht_factor = if ht_mod {
            ht_plasticity_factor(mesh[post_id].ht1a_cached, mesh[post_id].ht2a_cached)
        } else {
            256 // neutral
        };

        for src_idx in 0..mesh[post_id].sources.len() {
            let pre_id = mesh[post_id].sources[src_idx].from;
            let pre_last = last_fired[pre_id];

            // pre must have fired before this tick and within the window
            if pre_last == 0 || pre_last >= tic { continue; }
            let dt = tic - pre_last;
            if dt > window { continue; }

            // Linear decay: closer in time → stronger effect
            // strength = ltp_rate × (window − dt) / window
            let mut strength = (ltp_rate * (window - dt) as u32 / window as u32) as u16;
            if strength == 0 { strength = 1; } // at least +1 within window

            // Layer 2: DA gating — phasic reward selects which synapses learn
            if da_gated {
                strength = da_modulate(strength, tic, vta_last_fire, window);
            }

            // Layer 3: 5-HT regime — serotonin scales learning rate globally
            // 5-HT1A high → suppressed (factor < 256)
            // 5-HT2A high → enhanced  (factor > 256)
            if ht_factor != 256 && strength > 0 {
                strength = (strength as u32 * ht_factor / 256) as u16;
            }

            if strength > 0 {
                let w = &mut mesh[post_id].sources[src_idx].weight;
                let old = *w;
                *w = (*w + strength).min(weight_max);
                if *w != old {
                    events.push(StdpEvent {
                        pre_id, post_id, old_weight: old, new_weight: *w,
                        delta_t: dt, is_ltp: true,
                    });
                }
            }
        }
    }

    // ── LTD: for each pre-synaptic neuron that fired this tick ──
    // Check its targets — if a target fired recently BEFORE this tick
    // (post before pre = anti-causal), weaken the synapse.
    let targets_snapshot: Vec<Vec<usize>> = mesh.iter()
        .map(|n| n.targets.clone())
        .collect();

    for pre_id in 0..n {
        if !fired_this_tic[pre_id] { continue; }

        for &post_id in &targets_snapshot[pre_id] {
            if post_id >= n { continue; }
            if !mesh[post_id].stdp_enabled { continue; }
            // Skip if post also fired this tick (already handled by LTP)
            if fired_this_tic[post_id] { continue; }

            let post_last = last_fired[post_id];
            let window = mesh[post_id].stdp_window as usize;
            let ltd_rate = mesh[post_id].ltd_rate as u32;
            let weight_min = mesh[post_id].weight_min;
            let da_gated = mesh[post_id].stdp_da_gated;
            let ht_mod = mesh[post_id].stdp_ht_modulated;
            let ht_factor = if ht_mod {
                ht_plasticity_factor(mesh[post_id].ht1a_cached, mesh[post_id].ht2a_cached)
            } else {
                256
            };

            // post must have fired before this tick and within the window
            if post_last == 0 || post_last >= tic { continue; }
            let dt = tic - post_last;
            if dt > window { continue; }

            // Linear decay
            let mut strength = (ltd_rate * (window - dt) as u32 / window as u32) as u16;
            if strength == 0 { strength = 1; }

            // Layer 2: DA gating
            if da_gated {
                strength = da_modulate(strength, tic, vta_last_fire, window);
            }

            // Layer 3: 5-HT regime
            if ht_factor != 256 && strength > 0 {
                strength = (strength as u32 * ht_factor / 256) as u16;
            }

            if strength > 0 {
                if let Some(src) = mesh[post_id].sources.iter_mut().find(|s| s.from == pre_id) {
                    let old = src.weight;
                    src.weight = src.weight.saturating_sub(strength).max(weight_min);
                    if src.weight != old {
                        events.push(StdpEvent {
                            pre_id, post_id, old_weight: old, new_weight: src.weight,
                            delta_t: dt, is_ltp: false,
                        });
                    }
                }
            }
        }
    }

    // ── Update last_fired_tic for all neurons that fired ──
    for id in 0..n {
        if fired_this_tic[id] {
            mesh[id].last_fired_tic = tic;
        }
    }

    events
}