use crate::neuron::{Neuron, Shot};

pub fn phase_2(mesh: &mut Vec<Neuron>, tic: usize, shots: &mut Vec<Shot>, events: &mut Vec<String>) -> (bool, bool) {
    // Phase 2: execute
    let mut vta_fired = false;
    let mut lc_fired = false;

    for n in mesh.iter_mut() {
        if let Some(shot) = n.execute(tic) {
            if n.id == 2 { vta_fired = true; }
            if n.id == 7 { lc_fired = true; }
            match n.id {
                8 => events.push("BLA!".to_string()),
                9 => events.push(format!("FRZ(g={})", shot.gaba_dose)),
                _ => events.push(format!("N{}", n.id)),
            }
            shots.push(shot);
        }
    }

    (vta_fired, lc_fired)
}