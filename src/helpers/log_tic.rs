use crate::neuron::Neuron;

pub fn log_tic(mesh: &Vec<Neuron>, tic: usize, ring_ids: &Vec<usize>, events: &Vec<String>) {
    let ring_firing: String = ring_ids.iter()
        .filter(|&&r| mesh[r].fired_this_tic)
        .map(|r| format!("R{}", r))
        .collect::<Vec<_>>()
        .join(" ");

    println!("{:<5} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>3} {:>3} {:>3} {:>3}  {:>4}  {:>3} {:>3} {:>3} {:>3}  {} {}",
             tic,
             mesh[0].glut, mesh[1].glut, mesh[2].glut,
             mesh[7].glut, mesh[8].glut, mesh[9].glut,
             mesh[3].glut, mesh[4].glut, mesh[5].glut, mesh[6].glut,
             mesh[3].gaba, mesh[4].gaba, mesh[5].gaba, mesh[6].gaba,
             mesh[9].gaba_out_balance,
             mesh[3].out_gate, mesh[4].out_gate, mesh[5].out_gate, mesh[6].out_gate,
             if events.is_empty() { "—".to_string() } else { events.join(" ") },
             if !ring_firing.is_empty() { format!("[{}]", ring_firing) } else { String::new() });
}