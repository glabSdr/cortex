use crate::helpers::{MARKER_COLORS, RESET};
use cortical_neuron::CorticalNeuron;
use crate::runtime::NeuronEvent;

/// Colorize event label based on neuron's marker associations.
/// - IS a marker → bold + color
/// - TARGETS a marker → same color, not bold
/// - No marker association → plain text

pub fn colorize(event: &NeuronEvent, mesh: &[CorticalNeuron]) -> String {
    if event.neuron_id >= mesh.len() {
        return event.label.clone();
    }

    for m in MARKER_COLORS {
        if m.id == event.neuron_id {
            return format!("{}{}{}", m.bold, event.label, RESET);
        }
    }

    let targets = &mesh[event.neuron_id].targets;
    for m in MARKER_COLORS {
        if targets.contains(&m.id) {
            return format!("{}{}{}", m.normal, event.label, RESET);
        }
    }

    event.label.clone()
}

