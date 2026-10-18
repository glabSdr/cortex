use crate::neuron::Neuron;

pub fn log_summary(mesh: &Vec<Neuron>, ring_ids: &Vec<usize>) {
    println!("\n=== Summary ===");
    println!("Freeze[9]: gaba_bal={}/{}, glut={}",
             mesh[9].gaba_out_balance, mesh[9].gaba_out_max_balance, mesh[9].glut);
    for &rid in ring_ids {
        let n = &mesh[rid];
        println!("Ring[{}]: bal={}/{}, out_gate={}/{}, gaba={}",
                 rid, n.glut_out_balance, n.glut_out_max_balance,
                 n.out_gate, n.out_gate_baseline, n.gaba);
    }
}