use cortical_neuron::CorticalNeuron;

pub fn phase_7(mesh: &mut Vec<CorticalNeuron>, drn_fired: bool) {
    // Phase 7: Serotonin broadcast (DRN → all)
    if drn_fired {
        let ht = mesh[10].serotonin_dose;
        for n in mesh.iter_mut() {
            n.serotonin += ht;
        }
    }
}