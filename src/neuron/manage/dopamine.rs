use crate::neuron::Neuron;

// Дофамин → out_gate + пластичность
//
// 1. Модуляция out_gate:
//    D1 → out_gate -= amount   // "стреляй легче"
//    D2 → out_gate += amount   // "стреляй труднее"
//
// 2. Пластичность (правило трёх факторов):
//    Пресинаптический нейрон стрелял (source.from был active) +
//    Постсинаптический нейрон стрелял (self.firing) +
//    DA присутствует →
//    D1: source.weight ↑ (LTP, связь укрепляется)
//    D2: source.weight ↓ (LTD, связь ослабевает)
//
//    Пластичность на ВХОДЯЩИХ связях (sources), решает получатель.

const DOPAMINE_TAKE: u16 = 2;
const PLASTICITY_BASE_DELTA: u16 = 3; // bdnf_weight_delta, в будущем из генетики

impl Neuron {
    pub fn diffuse_dopamine(&mut self, dopamine: u16) {
        self.dopamine = dopamine;
    }

    fn download_dopamine(&mut self) -> (u16, u16) {
        if self.d1_take_pause == 0 && self.d2_take_pause == 0 {
            self.dopamine = 0;
            return (0, 0);
        }

        let mut d1: u16 = 0;
        let mut d1_counter = self.d1_take_pause;

        let mut d2: u16 = 0;
        let mut d2_counter = self.d2_take_pause;

        loop {
            if (d1 >= u8::MAX as u16 && d2 >= u8::MAX as u16) || self.dopamine == 0 {
                break;
            }

            if d1_counter == 0 {
                self.dopamine = self.dopamine.saturating_sub(DOPAMINE_TAKE);
                d1 += DOPAMINE_TAKE;
                d1_counter = self.d1_take_pause;
            }

            if d2_counter == 0 {
                self.dopamine = self.dopamine.saturating_sub(DOPAMINE_TAKE);
                d2 += DOPAMINE_TAKE;
                d2_counter = self.d2_take_pause;
            }

            d1_counter -= 1;
            d2_counter -= 1;
        }

        if d1 < self.d1_gate {
            d1 = 0;
        }

        if d2 < self.d2_gate {
            d2 = 0;
        }

        (d1, d2)
    }

    pub fn apply_dopamine(&mut self, active_neurons: &[bool]) {
        let (d1, d2) = self.download_dopamine();

        // ── 1. Модуляция out_gate ──
        let d1_effect = (d1 * self.d1_sensitivity / 255).min(u8::MAX as u16);
        let d2_effect = (d2 * self.d2_sensitivity / 255).min(u8::MAX as u16);

        self.out_gate = self.out_gate.saturating_sub(d1_effect);
        self.out_gate = self.out_gate.saturating_add(d2_effect);

        // ── 2. Пластичность (три фактора: pre fired + post fired + DA) ──
        if !self.fired_this_tic { return; }
        if d1_effect == 0 && d2_effect == 0 { return; }

        for source in self.sources.iter_mut() {
            // Пресинаптический нейрон должен был быть активен в этот тик
            if !active_neurons[source.from] { continue; }

            // D1: LTP — укрепить входящую связь
            if d1_effect > 0 {
                let strengthen = PLASTICITY_BASE_DELTA * d1_effect / 255;
                source.weight = source.weight.saturating_add(strengthen).min(u8::MAX as u16);
            }

            // D2: LTD — ослабить входящую связь
            if d2_effect > 0 {
                let weaken = PLASTICITY_BASE_DELTA * d2_effect / 255;
                source.weight = source.weight.saturating_sub(weaken);
            }
        }
    }
}