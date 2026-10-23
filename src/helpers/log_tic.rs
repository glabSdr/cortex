use crate::helpers::colorize::colorize;
use crate::helpers::{DRN_OUTPUT, FLASH, FLASH_YES, LC_OUTPUT, VTA_INPUT, VTA_OUTPUT};
use crate::runtime::NeuronEvent;
use cortical_neuron::CorticalNeuron;


pub fn log_tic(mesh: &[CorticalNeuron], tic: usize, ring_ids: &[usize], events: &[NeuronEvent]) {
    let dot = |id: usize| -> &str {
        if id < mesh.len() && mesh[id].fired_this_tic { "•" } else { " " }
    };

    let colored_events: String = if events.is_empty() {
        "—".to_string()
    } else {
        events.iter()
            .map(|e| colorize(e, mesh))
            .collect::<Vec<_>>()
            .join(" ")
    };

    let flash_dot = if FLASH < mesh.len() && mesh[FLASH].fired_this_tic { "!" } else { " " };

    println!("{:<5} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3}  {:>3} {:>3}  {}",
             tic,
             dot(VTA_OUTPUT), dot(LC_OUTPUT), dot(DRN_OUTPUT),
             mesh[ring_ids[0]].glut, mesh[ring_ids[1]].glut,
             mesh[ring_ids[2]].glut, mesh[ring_ids[3]].glut,
             mesh[VTA_INPUT].glut,
             mesh[VTA_INPUT].out_gate,
             mesh[FLASH_YES].out_gate,
             flash_dot,
             colored_events);
}