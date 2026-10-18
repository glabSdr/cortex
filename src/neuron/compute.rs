use crate::neuron::{Neuron, Shot};

impl Neuron {
    fn glut_after_gate(&mut self) -> u16 {
        if self.glut > self.glut_gate {
            if self.glut <= u8::MAX as u16 {
                let glut = self.glut;
                self.glut = 0;
                return glut;
            }

            self.glut -= u8::MAX as u16;
            return u8::MAX as u16;
        }

        0
    }

    fn gaba_after_gate(&mut self) -> u16 {
        if self.gaba > self.gaba_gate {
            if self.gaba <= u8::MAX as u16 {
                let gaba = self.gaba;
                self.gaba = 0;
                return gaba;
            }

            self.gaba = self.gaba.saturating_sub(u8::MAX as u16);

            return u8::MAX as u16;
        }

        0
    }
}

impl Neuron {
    fn fire (&mut self) -> Shot {
        self.fired_this_tic = true;
        self.glut_out_balance = self.glut_out_balance.saturating_sub(self.glut_dose);
        self.gaba_out_balance = self.gaba_out_balance.saturating_sub(self.gaba_dose);
        self.dopamine_out_balance = self.dopamine_out_balance.saturating_sub(self.dopamine_dose);

        self.refractory = self.refractory_after_fire;


        Shot {
            from: self.id,
            glut_dose: self.glut_dose,
            gaba_dose: self.gaba_dose,
            dopamine_dose: self.dopamine_dose,
            execute: self.programs.clone(),
            targets: self.targets.clone(),
        }
    }

    fn decide(&mut self, glut: u16, gaba: u16) -> bool {
        let glut_mid = glut.saturating_sub(gaba);
        glut_mid > self.out_gate
    }

    pub fn execute(&mut self, tic: usize) -> Option<Shot> {
        self.fired_this_tic = false;

        if self.refractory > 0
            || self.dead
            || self.glut_out_balance < self.glut_dose
            || self.gaba_out_balance < self.gaba_dose
            || self.dopamine_out_balance < self.dopamine_dose
        {
            return None;
        }

        let glut = self.glut_after_gate();
        let gaba = self.gaba_after_gate();

        match self.decide(glut, gaba) {
            true => Some(self.fire()),
            false => None,
        }
    }

    fn register_transmission(&mut self, glut: u16, gaba: u16, dopamine: u16) {
        self.glut += glut;
        self.gaba += gaba;
        self.dopamine += dopamine;
    }
}