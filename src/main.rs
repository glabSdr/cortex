mod neuron;
mod helpers;
mod runtime;
mod persistence;

use crate::neuron::Shot;

fn main() {
    println!("=== Experiment 4: RF → VTA + LC → Ring + BLA → Freeze ===\n");

    let ring_ids: Vec<usize> = vec![3, 4, 5, 6];

    let mut mesh = persistence::mesh::read_all();

    let ne_dose: u16 = 10;

    for tic in 1..=60 {
        if tic == 20 {
            mesh[8].glut = 100;
        }

        let mut shots: Vec<Shot> = Vec::new();
        let mut events: Vec<String> = Vec::new();

        runtime::phase_1(&mut mesh, tic);
        let (vta_fired, lc_fired) = runtime::phase_2(&mut mesh, tic, &mut shots, &mut events);
        runtime::phase_3(&mut mesh, tic);
        runtime::phase_4(&mut mesh, &shots);
        runtime::phase_5(&mut mesh, &ring_ids, vta_fired);
        runtime::phase_6(&mut mesh, lc_fired, ne_dose);
        runtime::phase_7(&mut mesh);

        helpers::log_tic(&mesh, tic, &ring_ids, &events);

    }

    helpers::log_summary(&mesh, &ring_ids);
}