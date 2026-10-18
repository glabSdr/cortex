use crate::persistence::bridge::Bridge;
use tokio_postgres::Error;

impl Bridge {
    /// Create — записывает значение + его вектор
    pub async fn create(&self, value: &str, embedding: &[f32]) -> Result<i64, Error> {
        self.check_dim(embedding);

        let row = self.client.query_one(
            "INSERT INTO knowledge_store (value) VALUES ($1) RETURNING id",
            &[&value],
        ).await?;
        let store_id: i64 = row.get(0);

        let vec_literal = Self::vec_to_pgvector(embedding);
        self.client.execute(
            &format!(
                "INSERT INTO knowledge_search (store_id, embedding)
                 VALUES ($1, '{vec_literal}'::vector)"
            ),
            &[&store_id],
        ).await?;

        Ok(store_id)
    }
}