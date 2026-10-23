pub fn log_header() {
    println!("Architecture: 69 neurons");
    println!("  START[0] → VTA_TONIC, GABA_BRAKE, LC_TONIC, DRN loops");
    println!("  VTA: INPUT[1] → OUTPUT[2] ~~DA~~> all");
    println!("  LC:  INPUT[16] → DEPO → OUTPUT[19] ~~NE~~> all");
    println!("  DRN: PROP_GLUT vs PROP_GABA → STABLE → OUTPUT[33] ~~5HT~~> all");
    println!("  LIKING[60] → ENK + DYN_RING → VTA modulation");
    println!("  THREAT[61] → LC phasic | ANTIREWARD[62] --gaba--> VTA_INPUT");
    println!("  >>> THR@20 OWN@35 LIK@40 NOV@55 ANTI@70 CARE@85 <<<");
    println!();
    println!("{:<5} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3}  {:>3} {:>3}  {}",
             "tick", "VTA","LC","DRN",
             "Vt0","Vt1","Vt2","Vt3",
             "VIn","ViG",
             "FYg","Flh",
             "events");
    println!("{}  VIn=VTA_INPUT.glut  ViG=VTA_INPUT.out_gate  FYg=FLASH_YES.out_gate  Flh=FLASH",
             " ".repeat(5));
    println!("{}", "-".repeat(105));
}
