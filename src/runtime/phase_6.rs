use cortical_neuron::CorticalNeuron;

pub fn phase_6(mesh: &mut Vec<CorticalNeuron>, lc_fired: bool) {
    // Phase 6: NE broadcast (LC → all)
    if lc_fired {
        let ne = mesh[7].ne_dose;
        for n in mesh.iter_mut() {
            n.norepinephrine += ne;
        }
    }
}