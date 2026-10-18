use crate::neuron::Neuron;

pub fn phase_7(mesh: &mut Vec<Neuron>) {
    // Phase 7: apply modulators + plasticity
    let active: Vec<bool> = mesh.iter().map(|n| n.fired_this_tic).collect();
    for n in mesh.iter_mut() {
        n.norepinephrine_apply();
        n.apply_dopamine(&active);
    }
}
