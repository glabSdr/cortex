use crate::neuron::Neuron;

pub fn phase_5(mesh: &mut Vec<Neuron>, _ring_ids: &Vec<usize>, vta_fired: bool) {
    // Phase 5: DA broadcast (VTA → all neurons, volume transmission)
    // Neurons without D1/D2 receptors ignore DA (dopamine_apply returns early)
    if vta_fired {
        let da = mesh[2].dopamine_dose;
        let vta_id = 2;
        for n in mesh.iter_mut() {
            if n.id != vta_id { // don't broadcast to self
                n.dopamine += da;
            }
        }
    }
}