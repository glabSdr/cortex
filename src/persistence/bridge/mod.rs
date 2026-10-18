mod helpers;
mod search;
mod delete;
mod update;
mod read;
mod create;
mod connect;
mod cache;

use serde::{Deserialize, Serialize};
use postgres::Client;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: i64,
    pub value: String,
    pub from_id: Option<i64>,
    pub to_id: Option<i64>,
    pub score: f64,
}

pub struct Bridge {
    client: Client,
    redis: redis::Connection,
    vector_dim: i32,
    novelty_threshold: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CachedRecord {
    pub id: i64,
    pub value: String,
    pub from_id: Option<i64>,
    pub to_id: Option<i64>,
}

pub type StoreError = Box<dyn std::error::Error + Send + Sync>;

const CACHE_TTL_SEC: u64 = 86400;