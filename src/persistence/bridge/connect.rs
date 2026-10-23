use crate::persistence::bridge::{Bridge, StoreError};
use postgres::{Client, Error, NoTls};

impl Bridge {
    pub fn connect(
        pg_connection_string: &str,
        redis_url: &str,
        vector_dim: i32,
        novelty_threshold: f64,
    ) -> Self {
        let mut client = Client::connect(pg_connection_string, NoTls).unwrap();

        client.execute("CREATE EXTENSION IF NOT EXISTS vector", &[]).unwrap();

        client.execute(
            "CREATE TABLE IF NOT EXISTS knowledge_store (
            id      BIGSERIAL PRIMARY KEY,
            value   TEXT NOT NULL,
            from_id BIGINT NULL,
            to_id   BIGINT NULL
        )",
            &[],
        ).unwrap();

        client.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS knowledge_search (
                id       BIGSERIAL PRIMARY KEY,
                store_id BIGINT NOT NULL REFERENCES knowledge_store(id) ON DELETE CASCADE,
                embedding vector({vector_dim}) NOT NULL
            )"
            ),
            &[],
        ).unwrap();

        client.execute(
            &format!(
                "CREATE INDEX IF NOT EXISTS knowledge_search_embedding_idx
             ON knowledge_search
             USING hnsw (embedding vector_cosine_ops)
             WITH (m = 16, ef_construction = 64)"
            ),
            &[],
        ).unwrap();

        let redis_client = redis::Client::open(redis_url).unwrap();
        let redis = redis_client.get_connection().unwrap();

        Self { client, redis, vector_dim, novelty_threshold }
    }
}