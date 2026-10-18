use crate::neuron::{Neuron, NeuronType, Shot};

pub struct FireResult {
    pub vta_fired: bool,
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

pub fn phase_2(mesh: &mut Vec<Neuron>, tic: usize, shots: &mut Vec<Shot>, events: &mut Vec<NeuronEvent>) -> FireResult {
    // Phase 2: execute (neurons decide & fire)
    let mut result = FireResult {
        vta_fired: false,
        lc_fired: false,
        drn_fired: false,
        start_record_fired: false,
        flash_fired: false,
    };

    for n in mesh.iter_mut() {
        if let Some(shot) = n.execute(tic) {
            if n.id == 2 { result.vta_fired = true; }
            if n.id == 7 { result.lc_fired = true; }
            if n.id == 10 { result.drn_fired = true; }
            if n.neuron_type == NeuronType::StartRecord { result.start_record_fired = true; }
            if n.neuron_type == NeuronType::Flash { result.flash_fired = true; }

            let label = match n.id {
                8 => "BLA!".to_string(),
                9 => format!("FRZ(g={})", shot.gaba_dose),
                10 => "DRN".to_string(),
                11 => format!("VGb(g={})", shot.gaba_dose),
                12 => format!("LIK(e={})", shot.enkephalin_dose),
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