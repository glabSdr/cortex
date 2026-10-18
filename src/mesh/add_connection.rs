use crate::mesh::ActivityRecorder;
use crate::neuron::{Neuron, Source};

impl ActivityRecorder {
    /// Add a directed connection from_id → to_id.
    /// Updates both targets (on sender) and sources (on receiver).
    /// Returns 1 if created, 0 if already existed.
    pub fn add_connection(mesh: &mut Vec<Neuron>, from_id: usize, to_id: usize, weight: u16) -> usize {
        // Skip if connection already exists
        if mesh[from_id].targets.contains(&to_id) {
            return 0;
        }

        // Add to targets of source neuron
        mesh[from_id].targets.push(to_id);

        // Add to sources of target neuron
        if !mesh[to_id].sources.iter().any(|s| s.from == from_id) {
            mesh[to_id].sources.push(Source { from: from_id, weight });
        }

        1
    }
}