mod neuron;
mod helpers;
mod runtime;
mod persistence;
mod mesh;

use crate::mesh::ActivityRecorder;
use crate::neuron::Shot;
use crate::runtime::NeuronEvent;

fn main() {
    // ── Load .env ──
    match dotenvy::dotenv() {
        Ok(path) => println!(".env loaded from: {:?}", path),
        Err(e) => println!(".env not loaded: {}", e),
    }

    let pg_conn = std::env::var("PG_CONNECTION")
        .expect("PG_CONNECTION not set");
    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL not set");

    // ── Memory store: Postgres + Redis ──
    let store = persistence::bridge::Bridge::connect(
        &pg_conn,
        &redis_url,
        384,   // vector_dim
        0.85,  // novelty_threshold
    ).expect("failed to connect memory store");
    println!("Memory store connected (pg + redis)\n");



    println!("=== Experiment 4: Avoidance + Liking + Flash ===\n");

    let ring_ids: Vec<usize> = vec![3, 4, 5, 6];

    let mut mesh = persistence::mesh::read_all();
    if mesh.is_empty() {
        panic!("mesh.jsonl not found or empty");
    }
    println!("Loaded {} neurons from mesh.jsonl\n", mesh.len());

    // Activity recorder: 2 hubs, default weight 128
    let mut recorder = ActivityRecorder::new(2, 128);

    // Kick-start
    mesh[0].glut = 20;
    mesh[3].glut = 10;

    helpers::log_header();

    for tic in 1..=100 {
        // Threat injection at tick 20
        if tic == 20 {
            mesh[8].glut = 100;
        }
        // Own injection at tick 38 (modulates Liking & Threat)
        if tic == 38 {
            mesh[16].glut = 50;
        }
        // Liking injection at tick 40
        if tic == 40 {
            mesh[12].glut = 100;
        }
        // Antireward injection at tick 60
        if tic == 60 {
            mesh[15].glut = 100;
        }

        let mut shots: Vec<Shot> = Vec::new();
        let mut events: Vec<NeuronEvent> = Vec::new();

        runtime::phase_1(&mut mesh, tic);
        let fire = runtime::phase_2(&mut mesh, tic, &mut shots, &mut events);

        // Record activity (always, even when not in recording mode — buffer is cheap)
        recorder.record_tick(&mesh, tic);

        // StartRecord fired → begin recording
        if fire.start_record_fired {
            recorder.start_record(tic);
        }

        // Flash fired → snapshot: create connections from recorded activity
        if fire.flash_fired {
            if let Some(result) = recorder.flash(&mut mesh, tic) {
                events.push(NeuronEvent {
                    neuron_id: usize::MAX, // system event, no neuron
                    label: format!(
                        "SNAP(t{}..{} hubs={:?} +{}conn)",
                        result.window_start, result.window_end,
                        result.hubs, result.connections_created
                    ),
                });
            }
        }

        runtime::phase_3(&mut mesh, tic);
        runtime::phase_4(&mut mesh, &shots);
        runtime::phase_5(&mut mesh, &ring_ids, fire.vta_fired);
        runtime::phase_6(&mut mesh, fire.lc_fired);
        runtime::phase_7(&mut mesh, fire.drn_fired);
        runtime::phase_8(&mut mesh);

        helpers::log_tic(&mesh, tic, &ring_ids, &events);
    }

    helpers::log_summary(&mesh, &ring_ids);
}