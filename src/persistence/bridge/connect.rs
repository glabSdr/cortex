use crate::persistence::bridge::Bridge;
use tokio_postgres::{Error, NoTls};

impl Bridge {
    pub async fn connect(
        connection_string: &str,
        vector_dim: i32,
        novelty_threshold: f64,
    ) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        // Соединение живёт в фоновой задаче
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // pgvector extension
        client.execute("CREATE EXTENSION IF NOT EXISTS vector", &[]).await?;

        // Таблица 1: Хранилище — развёрнутая информация
        client.execute(
            "CREATE TABLE IF NOT EXISTS knowledge_store (
                id    BIGSERIAL PRIMARY KEY,
                value TEXT NOT NULL
            )",
            &[],
        ).await?;

        // Таблица 2: Поиск — вектор + ссылка на хранилище
        client.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS knowledge_search (
                    id       BIGSERIAL PRIMARY KEY,
                    store_id BIGINT NOT NULL REFERENCES knowledge_store(id) ON DELETE CASCADE,
                    embedding vector({vector_dim}) NOT NULL
                )"
            ),
            &[],
        ).await?;

        // Индекс для быстрого cosine-поиска (HNSW)
        client.execute(
            &format!(
                "CREATE INDEX IF NOT EXISTS knowledge_search_embedding_idx
                 ON knowledge_search
                 USING hnsw (embedding vector_cosine_ops)
                 WITH (m = 16, ef_construction = 64)"
            ),
            &[],
        ).await?;

        Ok(Self { client, vector_dim, novelty_threshold })
    }
}