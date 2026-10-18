use crate::neuron::Neuron;

pub fn phase_5(mesh: &mut Vec<Neuron>, ring_ids: &Vec<usize>, vta_fired: bool) {
    // Phase 5: DA broadcast
    if vta_fired {
        let da = mesh[2].dopamine_dose;
        for &rid in ring_ids {
            mesh[rid].dopamine += da;
        }
    }
}