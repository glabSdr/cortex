use crate::neuron::{Neuron, Shot};

pub fn phase_4(mesh: &mut Vec<Neuron>, shots: &Vec<Shot>) {
    // Phase 4: deliver glut + gaba (weighted)
    for shot in shots.iter() {
        for &target in &shot.targets {
            let weight = mesh[target].sources.iter()
                .find(|s| s.from == shot.from)
                .map(|s| s.weight)
                .unwrap_or(255);

            if shot.glut_dose > 0 {
                let w = (shot.glut_dose as u32 * weight as u32 / 255) as u16;
                mesh[target].glut = (mesh[target].glut + w).min(u8::MAX as u16);
            }
            if shot.gaba_dose > 0 && target != shot.from {
                let w = (shot.gaba_dose as u32 * weight as u32 / 255) as u16;
                mesh[target].gaba = (mesh[target].gaba + w).min(u8::MAX as u16);
            }
        }
    }
}