use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use crate::neuron::Neuron;
use crate::persistence::mesh::MESH_FILE;
use crate::persistence::mesh::read_all::read_all;

pub fn save(neurons: Vec<Neuron>) {
    let path = Path::new("./").join(MESH_FILE);

    // Read the current state
    let mut existing: HashMap<usize, Neuron> = read_all()
        .into_iter()
        .map(|n| (n.id, n))
        .collect();

    // Apply updates
    for neuron in neurons {
        existing.insert(neuron.id, neuron);
    }

    // Collect and sort by id
    let mut all: Vec<Neuron> = existing.into_values().collect();
    all.sort_by_key(|n| n.id);

    // Rewrite the file
    let file = File::create(&path).expect("failed to create mesh file");
    let mut writer = BufWriter::new(file);

    for neuron in &all {
        let json = serde_json::to_string(neuron).expect("failed to serialize neuron");
        writeln!(writer, "{}", json).expect("failed to write line");
    }

    writer.flush().expect("failed to flush mesh file");
}