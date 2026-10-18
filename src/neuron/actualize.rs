use crate::neuron::Neuron;

impl Neuron {
    pub fn prepare(&mut self, tic: usize) {
        if tic != self.last_tic {
            self.refractory = self.refractory.saturating_sub(tic.saturating_sub(self.last_tic));
            self.glut = self.glut.saturating_sub(self.synapse_clearance);
            self.gaba = self.gaba.saturating_sub(self.synapse_clearance);
            self.dopamine = self.dopamine.saturating_sub(self.dopamine_clearance);
            self.serotonin = self.serotonin.saturating_sub(self.serotonin_clearance);
            
            if self.out_gate < self.out_gate_baseline {
                self.out_gate = (self.out_gate + 1).min(self.out_gate_baseline);
            }
        }
    }

    pub fn finalize(&mut self, tic: usize) {
        if tic != self.last_tic {
            self.glut_out_balance = (self.glut_out_balance + self.recovery_rate).min(self.glut_out_max_balance);
            self.gaba_out_balance =  (self.gaba_out_balance + self.recovery_rate).min(self.gaba_out_max_balance);
            self.dopamine_out_balance = (self.dopamine_out_balance + self.recovery_rate).min(self.dopamine_out_max_balance);
            self.last_tic = tic;
        }
    }
}
