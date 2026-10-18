mod helpers;
mod search;
mod delete;
mod update;
mod read;
mod create;
mod connect;

use tokio_postgres::Client;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: i64,
    pub value: String,
    pub score: f64,  // cosine similarity: 1.0 = идентично, 0.0 = ортогонально
}

pub struct Bridge {
    client: Client,
    vector_dim: i32,
    novelty_threshold: f64,
}