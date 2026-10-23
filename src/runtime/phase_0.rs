use cortical_neuron::CorticalNeuron;

pub fn phase_0(mesh: &mut Vec<CorticalNeuron>, tic: usize) {
    // Phase 0: prepare (clearance, gate recovery)
    for n in mesh.iter_mut() {
        n.prepare(tic);
    }
}