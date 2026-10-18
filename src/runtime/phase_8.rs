use crate::neuron::Neuron;

pub fn phase_8(mesh: &mut Vec<Neuron>) {
    // Phase 8: apply all neuromodulators
    let active: Vec<bool> = mesh.iter().map(|n| n.fired_this_tic).collect();
    for n in mesh.iter_mut() {
        n.norepinephrine_apply();
        n.apply_dopamine(&active);
        n.serotonin_apply();
        n.enkephalin_apply();
        n.dynorphin_apply();
        n.dynorphin_feedback();
    }
}