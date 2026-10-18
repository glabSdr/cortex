use crate::neuron::Neuron;

// Норадреналин → glut_gate, gaba_gate + пластичность
//
// 1. Модуляция входных порогов (контраст):
//    α1 → glut_gate ↓, gaba_gate ↓  // "слышу всё громче"
//    α2 → out_gate ↑                 // самоторможение (преимущественно LC)
//
// 2. Усиление пластичности (β1):
//    Если нейрон firing + DA присутствует + NE присутствует:
//    synaptic_weight delta умножается на β1 множитель.
//    "Эмоциональные события запоминаются сильнее"
//
// NE распространяется диффузно (broadcast), как DA.
// LC покрывает практически весь мозг — broadcast на все группы.

const NE_TAKE: u16 = 2;

impl Neuron {
    pub fn diffuse_norepinephrine(&mut self, norepinephrine: u16) {
        self.norepinephrine = norepinephrine;
    }

    fn download_norepinephrine(&mut self) -> (u16, u16) {
        if self.a1_take_pause == 0 && self.a2_take_pause == 0 {
            self.norepinephrine = 0;
            return (0, 0);
        }
        
        let mut a1: u16 = 0;
        let mut a1_counter = self.a1_take_pause;

        let mut a2: u16 = 0;
        let mut a2_counter = self.a2_take_pause;

        loop {
            if (a1 >= u8::MAX as u16 && a2 >= u8::MAX as u16) || self.norepinephrine == 0 {
                break;
            }

            if a1_counter == 0 && self.a1_take_pause > 0 {
                self.norepinephrine = self.norepinephrine.saturating_sub(NE_TAKE);
                a1 += NE_TAKE;
                a1_counter = self.a1_take_pause;
            }

            if a2_counter == 0 && self.a2_take_pause > 0 {
                self.norepinephrine = self.norepinephrine.saturating_sub(NE_TAKE);
                a2 += NE_TAKE;
                a2_counter = self.a2_take_pause;
            }

            a1_counter = a1_counter.saturating_sub(1);
            a2_counter = a2_counter.saturating_sub(1);
        }

        if a1 < self.a1_gate {
            a1 = 0;
        }

        if a2 < self.a2_gate {
            a2 = 0;
        }

        (a1, a2)
    }

    pub fn norepinephrine_apply(&mut self) {
        let (a1, a2) = self.download_norepinephrine();

        // ── 1. α1: снижает glut_gate и gaba_gate (контраст) ──
        let a1_effect = (a1 * self.a1_sensitivity / 255).min(u8::MAX as u16);
        if a1_effect > 0 {
            self.glut_gate = self.glut_gate.saturating_sub(a1_effect);
            self.gaba_gate = self.gaba_gate.saturating_sub(a1_effect);
        }

        // ── 2. α2: повышает out_gate (самоторможение LC) ──
        let a2_effect = (a2 * self.a2_sensitivity / 255).min(u8::MAX as u16);
        if a2_effect > 0 {
            self.out_gate = self.out_gate.saturating_add(a2_effect);
        }

        self.norepinephrine = 0;
    }

    /// β1 множитель для пластичности.
    /// Вызывается из dopamine_apply при расчёте delta.
    /// Возвращает множитель: 1 без NE, >1 с NE.
    pub fn beta1_plasticity_multiplier(&self, ne_amount: u16) -> u16 {
        if self.b1_sensitivity == 0 || ne_amount < self.b1_gate {
            return 1;
        }
        let raw = (ne_amount - self.b1_gate).min(u8::MAX as u16);
        1 + raw * self.b1_sensitivity / 255
    }
}