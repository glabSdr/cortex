mod helpers;
mod runtime;
mod persistence;
mod mesh;

use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::mesh::ActivityRecorder;
use crate::runtime::NeuronEvent;

use cortical_neuron::{Shot, TRANSMISSION};

fn glut_yes (id: usize) -> TRANSMISSION {
    (id, GLUT_YES, 0, 0, 0, 0, 0, 0)
}

const RAGE_ADDRESS: usize = 3;
const PANIC_ADDRESS: usize = 4;
const ASSUMPTION_ADDRESS: usize = 9;

const LIKING: usize = 60;
const THREAT: usize = 61;
const ANTIREWARD: usize = 62;
const OWN: usize = 63;
const NOVELTY: usize = 64;
const CARE: usize = 65;


fn cat_text() -> Vec<(Vec<String>, Vec<TRANSMISSION>)> {
    let text = fs::read_to_string("dataset.txt").unwrap();
    let mut result = Vec::new();
    for block in text.split("====") {
        let (_name, etc) = block.split_once("----").unwrap();
        let (text, params) = etc.split_once("----").unwrap();

        let rows = text.split("\n").collect::<Vec<&str>>();
        let mut text = String::new();
        for row in rows.iter() {
            text.push_str(row);
            text.push_str(" ");
        }

        let rows = text.split_whitespace().collect::<Vec<&str>>();
        let mut text: Vec<String> = Vec::new();
        let mut i = 0;
        for word in rows.iter() {
            let pointer = text.len();
            if i == 0 {
                if text.len() > 0 { text[pointer - 1].push_str(word) };
                text.push(String::new());
                i = 2;
            } else {
                text[pointer - 1].push_str(word);
                text[pointer - 1].push_str(" ");
                i -= 1;
            }
        }

        let params = params.split_whitespace().collect::<Vec<&str>>();
        let mut transmissions = Vec::new();
        if params.contains(&"LIKING") {
            transmissions.push(glut_yes(LIKING))
        }
        if params.contains(&"THREAT") {
            transmissions.push(glut_yes(THREAT))
        }
        if params.contains(&"RAGE") {
            transmissions.push(glut_yes(RAGE_ADDRESS))
        }
        if params.contains(&"PANIC") {
            transmissions.push(glut_yes(PANIC_ADDRESS))
        }
        if params.contains(&"ANTIREWARD") {
            transmissions.push(glut_yes(ANTIREWARD))
        }
        if params.contains(&"OWN") {
            transmissions.push(glut_yes(OWN))
        }
        if params.contains(&"NOVELTY") {
            transmissions.push(glut_yes(NOVELTY))
        }
        if params.contains(&"CARE") {
            transmissions.push(glut_yes(CARE))
        }
        if params.contains(&"ASSUMPTION") {
            transmissions.push(glut_yes(ASSUMPTION_ADDRESS))
        }

        result.push((text, transmissions))
    };

    result
}

const NOVELTY_ID: usize = 17;
const GLUT_YES: u16 = 100;


fn main() {
    // let text = cat_text();
    // println!("{:?}", text);

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
    );
    println!("Memory store connected (pg + redis)\n");


    println!("=== Experiment 4: Avoidance + Liking + Flash ===\n");

    let ring_ids: Vec<usize> = vec![3, 4, 5, 6];

    let mut mesh = persistence::mesh::read_all();
    if mesh.is_empty() {
        panic!("mesh.jsonl not found or empty");
    }

    println!("N10 enkephalin_dose={} enk_bal={}", mesh[10].enkephalin_dose, mesh[10].enkephalin_out_balance);

    println!("Loaded {} neurons from mesh.jsonl\n", mesh.len());

    let mut to_mesh_transmissions: Arc::<Mutex::<Vec<TRANSMISSION>>>  = Arc::new(Mutex::new(Vec::new()));  // (neuron_id, glut, gaba, dopamine, norepinephrine, serotonin, enkephalin, dynorphin)

    // Activity recorder: 2 hubs, default weight 128
    let mut recorder = ActivityRecorder::new(2, 128);

    // START neuron (id=0) is pre-charged with glut=20 in mesh.jsonl
    // It will fire on tick 1 and kick all tonic loops

    // ── Enable STDP on key integration neurons (runtime only) ──
    // VTA_INPUT: learns which stimuli predict reward
    mesh[1].stdp_enabled = true;
    mesh[1].stdp_window = 20;
    mesh[1].ltp_rate = 12;     // stronger LTP for visible effect
    mesh[1].ltd_rate = 6;      // asymmetric: LTP > LTD for net potentiation
    mesh[1].weight_min = 30;   // don't lose connections entirely
    mesh[1].weight_max = 255;
    mesh[1].stdp_da_gated = true;  // three-factor: DA modulates learning
    // Start marker synapses at mid-range so STDP can move them both ways
    // Tonic ring connections (N6, N9) stay at 255 — they're infrastructure
    for src in mesh[1].sources.iter_mut() {
        if src.from != 6 && src.from != 9 {
            src.weight = 128;  // naive weight — STDP will shape it
        }
    }

    // LC_SIGNAL_INPUT: learns which stimuli demand attention
    mesh[16].stdp_enabled = true;
    mesh[16].stdp_window = 15;
    mesh[16].ltp_rate = 6;
    mesh[16].ltd_rate = 3;
    mesh[16].weight_min = 30;
    mesh[16].weight_max = 255;
    mesh[16].stdp_da_gated = false;  // NE pathway, not DA-gated
    // Start all LC_SIGNAL_INPUT synapses at mid-range
    for src in mesh[16].sources.iter_mut() {
        src.weight = 128;
    }


    helpers::log_header();

    let mut vta_last_fire: usize = 0;       // tonic VTA (for DA broadcast)
    let mut reward_last: usize = 0;        // phasic reward (ENK/N10) for STDP DA gating

    // Log initial STDP weights
    println!("  STDP initial weights:");
    print!("    N1 sources: ");
    for src in &mesh[1].sources {
        print!("N{}=w{} ", src.from, src.weight);
    }
    println!();
    print!("    N16 sources: ");
    for src in &mesh[16].sources {
        print!("N{}=w{} ", src.from, src.weight);
    }
    println!("\n");

    for tic in 1..=300 {
        // ── Stimulus injections: repeated exposure for STDP learning ──
        //
        // Training protocol:
        //   CARE + LIKING paired 4 times → should produce strong LTP on N65→N1
        //   NOVELTY alone 3 times → should produce minimal change (no DA)
        //   ANTIREWARD alone 2 times → should produce no LTP (no DA)
        //   THREAT 4 times → strong LTP on N61→N16 (not DA-gated)

        // THREAT → LC pathway (3 exposures → attention learning)
        if tic == 20 || tic == 80 || tic == 160 || tic == 240 { mesh[THREAT].glut = 100; }

        // CARE + LIKING paired: reward-predictive learning
        if tic == 40 { mesh[CARE].glut = 50; }
        if tic == 45 { mesh[LIKING].glut = 100; }
        if tic == 100 { mesh[CARE].glut = 50; }
        if tic == 105 { mesh[LIKING].glut = 100; }
        if tic == 180 { mesh[CARE].glut = 50; }
        if tic == 185 { mesh[LIKING].glut = 100; }
        if tic == 260 { mesh[CARE].glut = 50; }
        if tic == 265 { mesh[LIKING].glut = 100; }

        // NOVELTY alone: no reward follows → minimal DA-gated learning
        if tic == 60 || tic == 140 || tic == 220 { mesh[NOVELTY].glut = 50; }

        // ANTIREWARD alone: no reward → no DA-gated learning
        if tic == 120 || tic == 200 { mesh[ANTIREWARD].glut = 100; }

        let mut shots: Vec<Shot> = Vec::new();
        let mut events: Vec<NeuronEvent> = Vec::new();

        runtime::phase_0(&mut mesh, tic);
        runtime::phase_1(&mut mesh, to_mesh_transmissions.clone());

        let fire = runtime::phase_2(&mut mesh, tic, &mut shots, &mut events);

        if fire.vta_fired { vta_last_fire = tic; }
        if fire.reward_fired { reward_last = tic; }

        recorder.record_tick(&mesh, tic);

        if fire.start_record_fired {
            recorder.start_record(tic);
        }

        if fire.flash_fired {
            if let Some(result) = recorder.flash(&mut mesh, tic) {
                events.push(NeuronEvent {
                    neuron_id: usize::MAX,
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

        let stdp_events = runtime::phase_9(&mut mesh, tic, reward_last);

        helpers::log_tic(&mesh, tic, &ring_ids, &events);

        // Log STDP weight changes
        for se in &stdp_events {
            let kind = if se.is_ltp { "LTP" } else { "LTD" };
            println!("      STDP {}: N{}→N{} weight {}→{} (Δt={})", kind, se.pre_id, se.post_id, se.old_weight, se.new_weight, se.delta_t);
        }
    }

    helpers::log_summary(&mesh, &ring_ids);

    // ── STDP weight summary ──
    println!("\n=== STDP Weight Changes ===");
    print!("  N1 (VTA_INPUT) sources: ");
    for src in &mesh[1].sources {
        print!("N{}=w{} ", src.from, src.weight);
    }
    println!();
    print!("  N16 (LC_SIGNAL_INPUT) sources: ");
    for src in &mesh[16].sources {
        print!("N{}=w{} ", src.from, src.weight);
    }
    println!();
}