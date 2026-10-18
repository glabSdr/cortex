use crate::neuron::Neuron;

pub fn phase_1(mesh: &mut Vec<Neuron>, tic: usize) {
    // Phase 1: prepare (clearance, gate recovery)
    for n in mesh.iter_mut() {
        n.prepare(tic);
    }
}