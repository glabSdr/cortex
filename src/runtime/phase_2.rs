use cortical_neuron::{CorticalNeuron, NeuronType, Shot};

pub struct FireResult {
    pub vta_fired: bool,
    pub vta_input_fired: bool,  // N1 phasic — marker-driven VTA activation
    pub reward_fired: bool,     // N10 (ENKEPHALIN_TO_VTA) — actual reward signal
    pub lc_fired: bool,
    pub drn_fired: bool,
    pub start_record_fired: bool,
    pub flash_fired: bool,
}

/// Event with neuron ID for color-coding in log output
pub struct NeuronEvent {
    pub neuron_id: usize,
    pub label: String,
}

// ── Neuron ID constants (keep in sync with gen_mesh.py) ──
pub const VTA_OUTPUT: usize = 2;
pub const LC_OUTPUT: usize = 19;
pub const DRN_OUTPUT: usize = 33;

pub fn phase_2(mesh: &mut Vec<CorticalNeuron>, tic: usize, shots: &mut Vec<Shot>, events: &mut Vec<NeuronEvent>) -> FireResult {
    let mut result = FireResult {
        vta_fired: false,
        vta_input_fired: false,
        reward_fired: false,
        lc_fired: false,
        drn_fired: false,
        start_record_fired: false,
        flash_fired: false,
    };

    for n in mesh.iter_mut() {
        if let Some(shot) = n.execute(tic) {
            if n.id == VTA_OUTPUT { result.vta_fired = true; }
            if n.id == 1 { result.vta_input_fired = true; }  // VTA_INPUT phasic
            if n.id == 10 { result.reward_fired = true; }     // ENKEPHALIN_TO_VTA = reward
            if n.id == LC_OUTPUT  { result.lc_fired = true; }
            if n.id == DRN_OUTPUT { result.drn_fired = true; }
            if n.neuron_type == NeuronType::StartRecord { result.start_record_fired = true; }
            if n.neuron_type == NeuronType::Flash { result.flash_fired = true; }

            let label = match n.id {
                0  => "START".to_string(),
                1  => "VIn".to_string(),
                2  => format!("VTA(da={})", shot.dopamine_dose),
                10 => format!("ENK(e={})", shot.enkephalin_dose),
                11 => "DYNs".to_string(),
                16 => "LCin".to_string(),
                19 => format!("LC(ne={})", n.ne_dose),
                31 => format!("STPs(g={})", shot.gaba_dose),
                33 => format!("DRN(ht={})", n.serotonin_dose),
                44 => "SSR".to_string(),
                56 => "FY!".to_string(),
                60 => format!("LIK(e={})", shot.enkephalin_dose),
                61 => "THR!".to_string(),
                62 => format!("ANTI(g={})", shot.gaba_dose),
                63 => "OWN!".to_string(),
                64 => "NOV!".to_string(),
                65 => "CARE".to_string(),
                66 => "RAGE!".to_string(),
                67 => "PANIC!".to_string(),
                68 => "ASMP".to_string(),
                _ => match n.neuron_type {
                    NeuronType::StartRecord => "REC!".to_string(),
                    NeuronType::Flash => "FLASH!".to_string(),
                    _ => format!("N{}", n.id),
                }
            };

            events.push(NeuronEvent { neuron_id: n.id, label });
            shots.push(shot);
        }
    }

    result
}