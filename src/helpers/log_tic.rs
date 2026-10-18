use crate::helpers::colorize::colorize;
use crate::neuron::Neuron;
use crate::runtime::NeuronEvent;

pub fn log_tic(mesh: &[Neuron], tic: usize, _ring_ids: &[usize], events: &[NeuronEvent]) {
    let dot = |id: usize| -> &str {
        if mesh[id].fired_this_tic { "•" } else { " " }
    };

    let colored_events: String = if events.is_empty() {
        "—".to_string()
    } else {
        events.iter()
            .map(|e| colorize(e, mesh))
            .collect::<Vec<_>>()
            .join(" ")
    };

    println!("{:<5} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {}",
             tic,
             dot(0), dot(2), dot(7), dot(8), dot(9), dot(10), dot(11), dot(12), dot(15), dot(16),
             mesh[3].glut, mesh[4].glut, mesh[5].glut, mesh[6].glut,
             mesh[3].out_gate, mesh[4].out_gate, mesh[5].out_gate, mesh[6].out_gate,
             colored_events);
}
