mod add_connection;
mod flash;
mod start_record;
mod record_tick;

use std::collections::HashMap;
use cortical_neuron::Source;

/// Tracks neuron firing history and performs Flash snapshots
/// to create associative connections between co-active neurons.
pub struct ActivityRecorder {
    /// Whether we're currently recording activity
    pub recording: bool,
    /// Tick when recording started
    pub record_start_tick: usize,
    /// Per-tick firing history: history[tick] = vec of neuron IDs that fired
    pub history: Vec<Vec<usize>>,
    /// How many hub neurons to select during flash
    pub hub_count: usize,
    /// Default weight for newly created connections
    pub default_weight: u16,
}

impl ActivityRecorder {
    pub fn new(hub_count: usize, default_weight: u16) -> Self {
        Self {
            recording: false,
            record_start_tick: 0,
            history: Vec::new(),
            hub_count,
            default_weight,
        }
    }
}

/// Result of a Flash snapshot for logging
pub struct FlashResult {
    pub window_start: usize,
    pub window_end: usize,
    pub hubs: Vec<usize>,
    pub participants: Vec<usize>,
    pub connections_created: usize,
}