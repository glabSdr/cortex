use crate::helpers::{DRN_OUTPUT, FLASH_YES, LC_OUTPUT, STOP_VTA_GABA_OUT, VTA_INPUT, VTA_OUTPUT};
use cortical_neuron::CorticalNeuron;

pub fn log_summary(mesh: &[CorticalNeuron], ring_ids: &[usize]) {
    println!("\n=== Summary ===");
    println!("VTA_OUTPUT[{}]: da_bal={}/{}, out_gate={}/{}",
             VTA_OUTPUT,
             mesh[VTA_OUTPUT].dopamine_out_balance, mesh[VTA_OUTPUT].dopamine_out_max_balance,
             mesh[VTA_OUTPUT].out_gate, mesh[VTA_OUTPUT].out_gate_baseline);
    println!("LC_OUTPUT[{}]:  ne_bal={}/{}, out_gate={}/{}",
             LC_OUTPUT,
             mesh[LC_OUTPUT].ne_out_balance, mesh[LC_OUTPUT].ne_out_max_balance,
             mesh[LC_OUTPUT].out_gate, mesh[LC_OUTPUT].out_gate_baseline);
    println!("DRN_OUTPUT[{}]: ht_bal={}/{}, out_gate={}/{}",
             DRN_OUTPUT,
             mesh[DRN_OUTPUT].serotonin_out_balance, mesh[DRN_OUTPUT].serotonin_out_max_balance,
             mesh[DRN_OUTPUT].out_gate, mesh[DRN_OUTPUT].out_gate_baseline);
    println!("VTA_INPUT[{}]:  out_gate={}/{}, κ-dyn={}",
             VTA_INPUT,
             mesh[VTA_INPUT].out_gate, mesh[VTA_INPUT].out_gate_baseline,
             mesh[VTA_INPUT].dynorphin);
    for &rid in ring_ids {
        let n = &mesh[rid];
        println!("VTA_TONIC[{}]: glut_bal={}/{}, out_gate={}/{}",
                 rid, n.glut_out_balance, n.glut_out_max_balance,
                 n.out_gate, n.out_gate_baseline);
    }
    println!("FLASH_YES[{}]: out_gate={}/{}, d2_gate={}",
             FLASH_YES,
             mesh[FLASH_YES].out_gate, mesh[FLASH_YES].out_gate_baseline,
             mesh[FLASH_YES].d2_gate);
    println!("STOP_VTA_GABA[{}]: gaba_bal={}/{}, out_gate={}/{}",
             STOP_VTA_GABA_OUT,
             mesh[STOP_VTA_GABA_OUT].gaba_out_balance, mesh[STOP_VTA_GABA_OUT].gaba_out_max_balance,
             mesh[STOP_VTA_GABA_OUT].out_gate, mesh[STOP_VTA_GABA_OUT].out_gate_baseline);
}