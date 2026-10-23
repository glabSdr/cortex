use std::collections::HashMap;
use crate::mesh::{ActivityRecorder, FlashResult};
use cortical_neuron::CorticalNeuron;

impl ActivityRecorder {
    /// Flash neuron fired — create connections based on recorded activity.
    /// Returns a description of what was created for logging.
    pub fn flash(&mut self, mesh: &mut Vec<CorticalNeuron>, tick: usize) -> Option<FlashResult> {
        if !self.recording {
            return None;
        }
        self.recording = false;

        let start = self.record_start_tick;
        let end = (tick + 1).min(self.history.len()); // inclusive of current tick

        // Count fires per neuron across the recording window
        let mut fire_counts: HashMap<usize, usize> = HashMap::new();
        for t in start..end {
            for &id in &self.history[t] {
                *fire_counts.entry(id).or_insert(0) += 1;
            }
        }

        if fire_counts.is_empty() {
            return None;
        }

        // Sort by fire count descending
        let mut sorted: Vec<(usize, usize)> = fire_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        // Select hubs (top N most active)
        let hub_count = self.hub_count.min(sorted.len());
        let hubs: Vec<usize> = sorted[..hub_count].iter().map(|&(id, _)| id).collect();
        let others: Vec<usize> = sorted[hub_count..].iter().map(|&(id, _)| id).collect();

        let weight = self.default_weight;
        let mut connections_created: usize = 0;

        // Connect each non-hub to each hub (bidirectional)
        for &hub_id in &hubs {
            for &other_id in &others {
                connections_created += Self::add_connection(mesh, other_id, hub_id, weight);
                connections_created += Self::add_connection(mesh, hub_id, other_id, weight);
            }
        }

        // Connect hubs to each other (bidirectional)
        for i in 0..hubs.len() {
            for j in (i + 1)..hubs.len() {
                connections_created += Self::add_connection(mesh, hubs[i], hubs[j], weight);
                connections_created += Self::add_connection(mesh, hubs[j], hubs[i], weight);
            }
        }

        Some(FlashResult {
            window_start: start,
            window_end: tick,
            hubs: hubs.clone(),
            participants: sorted.iter().map(|&(id, _)| id).collect(),
            connections_created,
        })
    }
}