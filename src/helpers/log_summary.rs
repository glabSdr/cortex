use crate::neuron::Neuron;

pub fn log_summary(mesh: &[Neuron], ring_ids: &[usize]) {
    println!("\n=== Summary ===");
    println!("DRN[10]: serotonin_bal={}/{}", mesh[10].serotonin_out_balance, mesh[10].serotonin_out_max_balance);
    println!("LC[7]:   ne_bal={}/{}", mesh[7].ne_out_balance, mesh[7].ne_out_max_balance);
    println!("VTA_GABA[11]: gaba_bal={}/{}, out_gate={}/{}, enk={}",
             mesh[11].gaba_out_balance, mesh[11].gaba_out_max_balance,
             mesh[11].out_gate, mesh[11].out_gate_baseline, mesh[11].enkephalin);
    println!("VTA[2]: da_bal={}/{}, out_gate={}/{}, gaba={}",
             mesh[2].dopamine_out_balance, mesh[2].dopamine_out_max_balance,
             mesh[2].out_gate, mesh[2].out_gate_baseline, mesh[2].gaba);
    println!("Freeze[9]: gaba_bal={}/{}", mesh[9].gaba_out_balance, mesh[9].gaba_out_max_balance);
    for &rid in ring_ids {
        let n = &mesh[rid];
        println!("Ring[{}]: glut_bal={}/{}, out_gate={}/{}",
                 rid, n.glut_out_balance, n.glut_out_max_balance,
                 n.out_gate, n.out_gate_baseline);
    }
}

