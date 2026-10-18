use crate::mesh::ActivityRecorder;
use crate::neuron::Neuron;

impl ActivityRecorder {
    /// Record which recordable neurons fired this tick.
    /// Called every tick after phase_2 (execute).
    pub fn record_tick(&mut self, mesh: &[Neuron], tick: usize) {
        // Ensure history is large enough
        while self.history.len() <= tick {
            self.history.push(Vec::new());
        }

        let fired: Vec<usize> = mesh.iter()
            .filter(|n| n.fired_this_tic && n.recordable)
            .map(|n| n.id)
            .collect();

        self.history[tick] = fired;
    }

}