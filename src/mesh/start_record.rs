use crate::mesh::ActivityRecorder;

impl ActivityRecorder {
    /// StartRecord neuron fired — begin recording from this tick.
    pub fn start_record(&mut self, tick: usize) {
        self.recording = true;
        self.record_start_tick = tick;
    }
}