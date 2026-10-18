

//Да, каждый модулятор влияет на конкретные gate, но каждый — на свой и по-своему.
// Норадреналин → glut_gate и gaba_gate (оба)
//
// НЕ снижает оба входных порога. Нейрон начинает «слышать» больше — и глутамата, и ГАМК. Кажется контрпродуктивным: зачем слышать больше тормоза? Но эффект — усиление контраста. Без НЕ: сильный сигнал = 100, шум = 30, gate = 40. Сигнал проходит (100 > 40), шум отсечён (30 < 40). С НЕ: gate снижается до 15. Сигнал проходит (100 > 15), шум тоже проходит (30 > 15). Но разница 100 vs 30 теперь видна в полном масштабе, а не обрезана на уровне 40. Нейрон получает более точную картину распределения входов. Сильное остаётся сильным, слабое видно как слабое, а не обрезано до нуля.
//
// NE → glut_gate -= amount
// NE → gaba_gate -= amount
// // "слышу всё громче"
//

//
// Ацетилхолин → glut_gate (избирательно)
//
// ACh снижает glut_gate не на всех нейронах, а только на тех, которые обслуживают текущий фокус внимания. Остальным не меняет ничего или даже повышает. Это пространственный фильтр: «вот эти входы важны, остальные нет».
//
// ACh → glut_gate -= amount (на целевых)
// ACh → glut_gate += amount (на нецелевых)  // опционально
// // "слышу только это"
//
// Серотонин → out_gate (глобально, в обе стороны)
//
// Серотонин сложнее остальных потому что через разные рецепторы делает противоположное. Но если упростить до одного эффекта: серотонин повышает out_gate глобально. Текущий активный процесс уже выше порога — он продолжает работать. Новые стимулы, которые раньше были чуть выше порога, теперь ниже — они не могут прервать текущий процесс.
//
// 5-HT (1A) → out_gate += amount  // "не отвлекайся, держи текущий процесс"
// 5-HT (2A) → out_gate -= amount  // "ломай текущий паттерн" (психоделики)
//
// Сводка
// Модулятор	Куда бьёт	Направление	Эффект в одном слове
// Норадреналин	glut_gate, gaba_gate	↓ оба	Громче
// Дофамин (D1)	out_gate	↓	Легче
// Дофамин (D2)	out_gate	↑	Труднее
// Ацетилхолин	glut_gate (избирательно)	↓ на целевых	Фокус
// Серотонин (1A)	out_gate	↑	Стабильнее
// Серотонин (2A)	out_gate	↓	Подвижнее
//
// Четыре модулятора, три gate, шесть операций. Вся оркестрация — это комбинация этих шести операций на разных группах нейронов с разной интенсивностью.

use serde::{Deserialize, Serialize};

mod compute;
mod actualize;
mod manage;


#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Program {
    Threat(u16),
    Disgust(u16),
    Antireward(u16),
    Own(u16),
    Novel(u16),
    Liking(u16),
    None
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub from: usize,
    pub weight: u16 // MAX::u8
}

pub struct Shot {
    pub from: usize,
    
    pub glut_dose: u16, // <= u8::MAX
    pub gaba_dose: u16, // <= u8::MAX
    pub dopamine_dose: u16, // <= u8::MAX

    pub execute: [Program; 6],
    pub targets: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Neuron {
    pub id: usize,

    pub sources: Vec<Source>,

    pub refractory: usize,
    pub refractory_after_fire: usize,
    pub recovery_rate: u16,
    pub out_gate_baseline: u16,

    pub last_tic: usize,

    pub a1_take_pause: u16,
    pub a2_take_pause: u16,
    pub ht1a_take_pause: u16,
    pub ht2a_take_pause: u16,
    
    pub ne_clearance: u16,

    pub glut_gate_baseline: u16,  
    pub gaba_gate_baseline: u16,
    
    pub dopamine: u16,
    pub norepinephrine: u16,
    pub serotonin: u16,

    pub d1_take_pause: u16,
    pub d2_take_pause: u16,

    pub glut: u16,
    pub gaba: u16,
    // d1
    // d2

    pub glut_gate: u16, // <= u8::MAX
    pub gaba_gate: u16, // <= u8::MAX
    pub d1_gate: u16, // <= u8::MAX
    pub d2_gate: u16, // <= u8::MAX
    pub a1_gate: u16, // <= u8::MAX
    pub a2_gate: u16, // <= u8::MAX
    pub b1_gate: u16, // <= u8::MAX
    pub ht1a_gate: u16, // <= u8::MAX
    pub ht2a_gate: u16, // <= u8::MAX

    pub d1_sensitivity: u16,
    pub d2_sensitivity: u16,
    pub a1_sensitivity: u16,
    pub a2_sensitivity: u16,
    pub b1_sensitivity: u16,
    pub ht1a_sensitivity: u16,
    pub ht2a_sensitivity: u16,

    pub synapse_clearance: u16,
    pub dopamine_clearance: u16,
    pub serotonin_clearance: u16,

    pub out_gate: u16,

    pub programs: [Program; 6],

    pub glut_out_balance: u16,
    pub gaba_out_balance: u16,
    pub dopamine_out_balance: u16,

    pub glut_out_max_balance: u16,
    pub gaba_out_max_balance: u16,
    pub dopamine_out_max_balance: u16,

    pub glut_dose: u16, // <= u8::MAX
    pub gaba_dose: u16,  // <= u8::MAX
    pub dopamine_dose: u16, // <= u8::MAX

    pub fired_this_tic: bool,

    pub targets: Vec<usize>,

    pub dead: bool
}