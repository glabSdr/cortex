use crate::neuron::Neuron;

impl Neuron {
    pub fn prepare(&mut self, tic: usize) {
        if tic != self.last_tic {
            self.refractory = self.refractory.saturating_sub(tic.saturating_sub(self.last_tic));
            self.glut = self.glut.saturating_sub(self.synapse_clearance);
            self.gaba = self.gaba.saturating_sub(self.synapse_clearance);
            self.dopamine = self.dopamine.saturating_sub(self.dopamine_clearance);
            self.serotonin = self.serotonin.saturating_sub(self.serotonin_clearance);
            self.norepinephrine = self.norepinephrine.saturating_sub(self.ne_clearance);
            self.enkephalin = self.enkephalin.saturating_sub(self.enkephalin_clearance);
            self.dynorphin = self.dynorphin.saturating_sub(self.dynorphin_clearance);

            // Bidirectional out_gate recovery toward baseline
            if self.out_gate < self.out_gate_baseline {
                self.out_gate = (self.out_gate + 1).min(self.out_gate_baseline);
            } else if self.out_gate > self.out_gate_baseline {
                self.out_gate = self.out_gate.saturating_sub(1).max(self.out_gate_baseline);
            }

            // Bidirectional glut_gate recovery toward baseline
            if self.glut_gate < self.glut_gate_baseline {
                self.glut_gate = (self.glut_gate + 1).min(self.glut_gate_baseline);
            } else if self.glut_gate > self.glut_gate_baseline {
                self.glut_gate = self.glut_gate.saturating_sub(1).max(self.glut_gate_baseline);
            }

            // Bidirectional gaba_gate recovery toward baseline
            if self.gaba_gate < self.gaba_gate_baseline {
                self.gaba_gate = (self.gaba_gate + 1).min(self.gaba_gate_baseline);
            } else if self.gaba_gate > self.gaba_gate_baseline {
                self.gaba_gate = self.gaba_gate.saturating_sub(1).max(self.gaba_gate_baseline);
            }
        }
    }

    pub fn finalize(&mut self, tic: usize) {
        if tic != self.last_tic {
            self.glut_out_balance = (self.glut_out_balance + self.recovery_rate).min(self.glut_out_max_balance);
            self.gaba_out_balance = (self.gaba_out_balance + self.recovery_rate).min(self.gaba_out_max_balance);
            self.dopamine_out_balance = (self.dopamine_out_balance + self.recovery_rate).min(self.dopamine_out_max_balance);
            self.enkephalin_out_balance = (self.enkephalin_out_balance + self.recovery_rate).min(self.enkephalin_out_max_balance);
            self.dynorphin_out_balance = (self.dynorphin_out_balance + self.recovery_rate).min(self.dynorphin_out_max_balance);
            self.serotonin_out_balance = (self.serotonin_out_balance + self.recovery_rate).min(self.serotonin_out_max_balance);
            self.ne_out_balance = (self.ne_out_balance + self.recovery_rate).min(self.ne_out_max_balance);
            self.last_tic = tic;
        }
    }
}