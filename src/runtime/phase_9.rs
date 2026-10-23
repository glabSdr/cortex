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
/// Returns modulated strength based on how recently VTA fired.
/// If VTA fired this tick or 1 tick ago → full strength.
/// Decays linearly to 0 over `window` ticks.
/// If VTA never fired (vta_last_fire=0) → strength unchanged (baseline learning).
/// DA modulation for three-factor learning rule.
/// Uses PHASIC VTA signal (VTA_INPUT fires = reward-predicting stimulus).
/// Tonic VTA firing does NOT gate plasticity.
///
/// If VTA_INPUT fired recently → full strength, decaying linearly to 0.
/// If VTA_INPUT never fired or fired long ago → 0 (no learning without phasic DA).
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

/// Phase 9: Spike-Timing Dependent Plasticity (STDP)
///
/// Runs AFTER all neuromodulators are applied (phase_8).
/// Modifies synapse weights based on relative spike timing:
///
///   pre fires BEFORE post (causal, Δt > 0) → LTP (weight ↑)
///   pre fires AFTER  post (anti-causal)     → LTD (weight ↓)
///
/// Effect magnitude decays linearly with |Δt| within the STDP window.
/// Optionally gated by dopamine (three-factor learning rule):
///   STDP_effect × (dopamine / 255)
///
/// Only neurons with stdp_enabled=true have their input weights modified.
/// last_fired_tic is updated for ALL neurons (needed as source info).
///
/// `vta_last_fire`: tick when VTA last fired (0 = never). Used for DA gating:
/// phasic DA opens a plasticity window. Learning is strongest right after
/// VTA fire and decays over the neuron's stdp_window.
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

            // Three-factor DA modulation: phasic DA gates plasticity
            // VTA fired recently → full learning; fades with distance
            if da_gated {
                strength = da_modulate(strength, tic, vta_last_fire, window);
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
    // We need targets list, which we clone to avoid borrow issues.
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

            // post must have fired before this tick and within the window
            if post_last == 0 || post_last >= tic { continue; }
            let dt = tic - post_last;
            if dt > window { continue; }

            // Linear decay
            let mut strength = (ltd_rate * (window - dt) as u32 / window as u32) as u16;
            if strength == 0 { strength = 1; }

            // Three-factor DA modulation
            if da_gated {
                strength = da_modulate(strength, tic, vta_last_fire, window);
            }

            if strength > 0 {
                // Find the source entry for this pre→post synapse
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