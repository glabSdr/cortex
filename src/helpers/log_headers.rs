pub fn log_header() {
    println!("Circuit:");
    println!("  RF[0] <-> RF[1]              eternal oscillator");
    println!("  RF[0] -> VTA[2], LC[7], DRN[10], VTA_GABA[11]");
    println!("  VTA[2] ~~DA~~> Ring[3-6]     DA broadcast");
    println!("  VTA[2] --glut--> Ring[3]     co-transmission");
    println!("  LC[7]  ~~NE~~> all           NE broadcast");
    println!("  DRN[10] ~~5-HT~~> all        serotonin broadcast");
    println!("  VTA_GABA[11] --gaba--> VTA[2]  tonic inhibition");
    println!("  Liking[12] --enk--> VTA_GABA[11]  μ disinhibition");
    println!("  BLA[8] -> Freeze[9] + LC[7]");
    println!("  Freeze[9] --gaba--> Ring + DRN[10]  (avoidance: 5-HT↓)");
    println!("  >>> THREAT at tick 20,  LIKING at tick 40 <<<");
    println!();
    println!("{:<5} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {}",
             "tick", "RF0","VTA","LC","BLA","FRZ","DRN","VGb","Lik",
             "R3","R4","R5","R6",
             "R3o","R4o","R5o","R6o",
             "events");
    println!("{}", "-".repeat(110));
}
