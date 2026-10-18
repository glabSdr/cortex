

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
// Summary
// Modulator	Where it hits	Direction	Effect in one word
// Norepinephrine	glut_gate, gaba_gate	↓ both	Louder
// Dopamine (D1)	out_gate	↓	Easier
// Dopamine (D2)	out_gate	↑	Harder
// Acetylcholine	glut_gate (selectively)	↓ on targets	Focus
// Serotonin (1A)	out_gate	↑	More stable
// Serotonin (2A)	out_gate	↓	More flexible
//
// Four modulators, three gates, six operations. All orchestration is a combination of these six operations across different neuron groups at different intensities.

use serde::{Deserialize, Serialize};

mod compute;
mod actualize;
mod manage;


#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Program {
    Threat(u16),
    Disgust(u16),
    Antireward(u16),
    Own(u16),
    Novel(u16),
    Liking(u16),
    None
}

#[derive(Serialize, Deserialize)]
pub enum NeuronType {
    Logic,
    ThalamusCorticalToCortex,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub from: usize,
    pub weight: u16 // MAX::u8
}

pub struct Shot {
    pub from: usize,
    
    pub glut_dose: u16, // <= u8::MAX
    pub gaba_dose: u16, // <= u8::MAX
    pub dopamine_dose: u16, // <= u8::MAX

    pub execute: [Program; 6],
    pub targets: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Neuron {
    pub id: usize,

    pub neuron_type: NeuronType,

    pub sources: Vec<Source>,

    pub refractory: usize,
    pub refractory_after_fire: usize,
    pub recovery_rate: u16,
    pub out_gate_baseline: u16,

    pub last_tic: usize,

    pub a1_take_pause: u16,
    pub a2_take_pause: u16,
    pub ht1a_take_pause: u16,
    pub ht2a_take_pause: u16,
    
    pub ne_clearance: u16,

    pub glut_gate_baseline: u16,  
    pub gaba_gate_baseline: u16,
    
    pub dopamine: u16,
    pub norepinephrine: u16,
    pub serotonin: u16,

    pub d1_take_pause: u16,
    pub d2_take_pause: u16,

    pub glut: u16,
    pub gaba: u16,
    // d1
    // d2

    pub glut_gate: u16, // <= u8::MAX
    pub gaba_gate: u16, // <= u8::MAX
    pub d1_gate: u16, // <= u8::MAX
    pub d2_gate: u16, // <= u8::MAX
    pub a1_gate: u16, // <= u8::MAX
    pub a2_gate: u16, // <= u8::MAX
    pub b1_gate: u16, // <= u8::MAX
    pub ht1a_gate: u16, // <= u8::MAX
    pub ht2a_gate: u16, // <= u8::MAX

    pub d1_sensitivity: u16,
    pub d2_sensitivity: u16,
    pub a1_sensitivity: u16,
    pub a2_sensitivity: u16,
    pub b1_sensitivity: u16,
    pub ht1a_sensitivity: u16,
    pub ht2a_sensitivity: u16,

    pub synapse_clearance: u16,
    pub dopamine_clearance: u16,
    pub serotonin_clearance: u16,

    pub out_gate: u16,

    pub programs: [Program; 6],

    pub glut_out_balance: u16,
    pub gaba_out_balance: u16,
    pub dopamine_out_balance: u16,

    pub glut_out_max_balance: u16,
    pub gaba_out_max_balance: u16,
    pub dopamine_out_max_balance: u16,

    pub glut_dose: u16, // <= u8::MAX
    pub gaba_dose: u16,  // <= u8::MAX
    pub dopamine_dose: u16, // <= u8::MAX

    pub fired_this_tic: bool,

    pub targets: Vec<usize>,

    pub dead: bool
}