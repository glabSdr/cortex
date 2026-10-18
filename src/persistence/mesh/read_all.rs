use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::neuron::Neuron;
use crate::persistence::mesh::MESH_FILE;

pub fn read_all() -> Vec<Neuron> {
    let path = Path::new("./").join(MESH_FILE);

    if !path.exists() {
        return vec![];
    }

    let file = File::open(&path).expect("failed to open mesh file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| {
            let line = line.expect("failed to read line");
            if line.trim().is_empty() {
                return None;
            }
            Some(serde_json::from_str::<Neuron>(&line).expect("failed to parse neuron"))
        })
        .collect()
}