//Yes, each modulator affects specific gates, but each in its own way.
// Norepinephrine → glut_gate and gaba_gate (both)
//
// NE lowers both input thresholds. The neuron starts "hearing" more — both glutamate and GABA. Seems counterproductive: why hear more inhibition? But the effect is contrast enhancement. Without NE: strong signal = 100, noise = 30, gate = 40. The signal passes (100 > 40), noise is cut off (30 < 40). With NE: gate drops to 15. The signal passes (100 > 15), noise also passes (30 > 15). But the difference between 100 and 30 is now visible at full scale, not clipped at the level of 40. The neuron gets a more precise picture of the input distribution. Strong stays strong, weak is seen as weak, not clipped to zero.
//
// NE → glut_gate -= amount
// NE → gaba_gate -= amount
// // "hearing everything louder"
//

//
// Acetylcholine → glut_gate (selectively)
//
// ACh lowers glut_gate not on all neurons, but only on those serving the current focus of attention. For the rest it changes nothing, or even raises it. This is a spatial filter: "these inputs matter, the rest don't."
//
// ACh → glut_gate -= amount (on targets)
// ACh → glut_gate += amount (on non-targets)  // optional
// // "hearing only this"
//
// Serotonin → out_gate (globally, in both directions)
//
// Serotonin is more complex than the rest because through different receptors it does the opposite. But simplified to one effect: serotonin raises out_gate globally. The currently active process is already above threshold — it keeps running. New stimuli that used to be slightly above threshold are now below it — they can't interrupt the current process.
//
// 5-HT (1A) → out_gate += amount  // "don't get distracted, hold the current process"
// 5-HT (2A) → out_gate -= amount  // "break the current pattern" (psychedelics)
//
// Opioid peptides — two separate synaptic channels:
//
// Enkephalin (μ-ligand): local interneurons release into synapse
//   μ on VTA GABA interneuron → GABA↓ → DA disinhibition (DA↑)
//   μ on LC → out_gate↑ → NE↓ (analgesia, calm)
//   μ on BLA → out_gate↑ → anxiolysis
//
// Dynorphin (κ-ligand): MSN striatum synthesizes via DA→D1→CREB→prodynorphin
//   κ on VTA DA neuron → out_gate↑ → DA↓ (dysphoria, antireward)
//   κ on LC → out_gate↓ → NE↑ (stress, anxiety)
//   κ on BLA → out_gate↓ → anxiogenic
//
// Both are synaptic (not volume transmission), modeled like glut/gaba through Shot
//
// Summary
// Modulator	Where it hits	Direction	Effect in one word
// Norepinephrine	glut_gate, gaba_gate	↓ both	Louder
// Dopamine (D1)	out_gate	↓	Easier
// Dopamine (D2)	out_gate	↑	Harder
// Acetylcholine	glut_gate (selectively)	↓ on targets	Focus
// Serotonin (1A)	out_gate	↑	More stable
// Serotonin (2A)	out_gate	↓	More flexible
// Enkephalin (μ)	GABA/out_gate	↑	Pleasure / calm
// Dynorphin (κ)	out_gate	↑ on VTA, ↓ on LC/BLA	Brake / stress
//
// Six modulators, three gates, ten operations.

use serde::{Serialize, Deserialize};

mod compute;
mod actualize;
mod manage;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NeuronType {
    Logic,
    ThalamusCorticalToCortex,
    StartRecord,
    Flash,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub from: usize,
    pub weight: u16 // MAX::u8
}

pub struct Shot {
    pub from: usize,

    pub glut_dose: u16,        // <= u8::MAX
    pub gaba_dose: u16,        // <= u8::MAX
    pub dopamine_dose: u16,    // <= u8::MAX
    pub enkephalin_dose: u16,  // <= u8::MAX  — μ ligand, synaptic
    pub dynorphin_dose: u16,   // <= u8::MAX  — κ ligand, synaptic

    pub targets: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: usize,

    pub neuron_type: NeuronType,

    pub sources: Vec<Source>,

    pub refractory: usize,
    pub refractory_after_fire: usize,
    pub recovery_rate: u16,
    pub out_gate_baseline: u16,

    pub last_tic: usize,

    // --- Take pauses (all receptors) ---
    pub a1_take_pause: u16,
    pub a2_take_pause: u16,
    pub ht1a_take_pause: u16,
    pub ht2a_take_pause: u16,
    pub d1_take_pause: u16,
    pub d2_take_pause: u16,
    pub mu_take_pause: u16,
    pub kappa_take_pause: u16,

    // --- Clearance ---
    pub ne_clearance: u16,
    pub synapse_clearance: u16,
    pub dopamine_clearance: u16,
    pub serotonin_clearance: u16,
    pub enkephalin_clearance: u16,
    pub dynorphin_clearance: u16,

    // --- Gate baselines ---
    pub glut_gate_baseline: u16,
    pub gaba_gate_baseline: u16,

    // --- Current synapse levels ---
    pub dopamine: u16,
    pub norepinephrine: u16,
    pub serotonin: u16,
    pub enkephalin: u16,
    pub dynorphin: u16,

    pub glut: u16,
    pub gaba: u16,

    // --- Receptor gates ---
    pub glut_gate: u16,     // <= u8::MAX
    pub gaba_gate: u16,     // <= u8::MAX
    pub d1_gate: u16,       // <= u8::MAX
    pub d2_gate: u16,       // <= u8::MAX
    pub a1_gate: u16,       // <= u8::MAX
    pub a2_gate: u16,       // <= u8::MAX
    pub b1_gate: u16,       // <= u8::MAX
    pub ht1a_gate: u16,     // <= u8::MAX
    pub ht2a_gate: u16,     // <= u8::MAX
    pub mu_gate: u16,       // <= u8::MAX
    pub kappa_gate: u16,    // <= u8::MAX

    // --- Receptor sensitivities ---
    pub d1_sensitivity: u16,
    pub d2_sensitivity: u16,
    pub a1_sensitivity: u16,
    pub a2_sensitivity: u16,
    pub b1_sensitivity: u16,
    pub ht1a_sensitivity: u16,
    pub ht2a_sensitivity: u16,
    pub mu_sensitivity: u16,
    pub kappa_sensitivity: u16,

    // --- Output gate ---
    pub out_gate: u16,

    // --- Output balances ---
    pub glut_out_balance: u16,
    pub gaba_out_balance: u16,
    pub dopamine_out_balance: u16,
    pub enkephalin_out_balance: u16,
    pub dynorphin_out_balance: u16,
    pub serotonin_out_balance: u16,
    pub ne_out_balance: u16,

    pub glut_out_max_balance: u16,
    pub gaba_out_max_balance: u16,
    pub dopamine_out_max_balance: u16,
    pub enkephalin_out_max_balance: u16,
    pub dynorphin_out_max_balance: u16,
    pub serotonin_out_max_balance: u16,
    pub ne_out_max_balance: u16,

    // --- Output doses ---
    pub glut_dose: u16,      // <= u8::MAX
    pub gaba_dose: u16,      // <= u8::MAX
    pub dopamine_dose: u16,  // <= u8::MAX
    pub enkephalin_dose: u16,
    pub dynorphin_dose: u16,
    pub serotonin_dose: u16,
    pub ne_dose: u16,

    // --- Dynorphin feedback loop (MSN/striatum only) ---
    pub dynorphin_accumulator: u16,   // accumulates DA level each tick
    pub dynorphin_threshold: u16,     // DA threshold to trigger prodynorphin transcription
    pub dynorphin_delay: u16,         // ticks delay (gene transcription simulation)
    pub dynorphin_counter: u16,       // ticks since threshold exceeded

    // --- State ---
    pub fired_this_tic: bool,

    pub targets: Vec<usize>,

    pub dead: bool,

    /// Whether this neuron participates in Flash snapshots.
    /// Infrastructure neurons (RF, VTA, LC, DRN, VTA_GABA, Flash, StartRecord)
    /// set this to false. Default true for backward compat with old mesh.jsonl.
    #[serde(default = "default_recordable")]
    pub recordable: bool,
}

fn default_recordable() -> bool { true }