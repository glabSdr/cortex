use crate::helpers::{MARKER_COLORS, RESET};
use crate::neuron::Neuron;
use crate::runtime::NeuronEvent;

/// Colorize event label based on neuron's marker associations.
/// - IS a marker → bold + color
/// - TARGETS a marker → same color, not bold
/// - No marker association → plain text
pub fn colorize(event: &NeuronEvent, mesh: &[Neuron]) -> String {
    // System events (SNAP, etc.) — no coloring
    if event.neuron_id >= mesh.len() {
        return event.label.clone();
    }

    // Check if this neuron IS a marker
    for m in MARKER_COLORS {
        if m.id == event.neuron_id {
            return format!("{}{}{}", m.bold, event.label, RESET);
        }
    }

    // Check if this neuron targets a marker (dynamic — picks up Flash connections)
    let targets = &mesh[event.neuron_id].targets;
    for m in MARKER_COLORS {
        if targets.contains(&m.id) {
            return format!("{}{}{}", m.normal, event.label, RESET);
        }
    }

    // No marker association — plain
    event.label.clone()
}

