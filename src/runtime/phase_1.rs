use cortical_neuron::CorticalNeuron;
use std::sync::{Mutex, Arc};

pub fn phase_1(mesh: &mut Vec<CorticalNeuron>, to_mesh_transmissions: Arc<Mutex<Vec<(usize, u16, u16, u16, u16, u16, u16, u16)>>>) {
    // Phase 1: register transmissions
    let mut to_mesh_transmissions = to_mesh_transmissions.lock().unwrap();
    for transmission in to_mesh_transmissions.iter() {
        let id = transmission.0;
        mesh[id].register_transmission(
            transmission.1,
            transmission.2,
            transmission.3,
            transmission.4,
            transmission.5,
            transmission.6,
            transmission.7,
        )
    }

    to_mesh_transmissions.clear();
}