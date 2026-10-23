use cortical_neuron::CorticalNeuron;

pub fn phase_3(mesh: &mut Vec<CorticalNeuron>, tic: usize) {
    // Phase 3: finalize (balance recovery)
    for n in mesh.iter_mut() {
        n.finalize(tic);
    }
}