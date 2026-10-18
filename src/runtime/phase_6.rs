use crate::neuron::Neuron;

pub fn phase_6(mesh: &mut Vec<Neuron>, lc_fired: bool, ne_dose: u16) {
    // Phase 6: NE broadcast
    if lc_fired {
        for n in mesh.iter_mut() {
            n.norepinephrine += ne_dose;
        }
    }
}