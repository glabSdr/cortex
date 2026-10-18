use crate::persistence::bridge::Bridge;
use tokio_postgres::Error;

impl Bridge {
    /// Update — обновляет значение и вектор
    pub async fn update(&self, id: i64, value: &str, embedding: &[f32]) -> Result<bool, Error> {
        self.check_dim(embedding);

        let updated = self.client.execute(
            "UPDATE knowledge_store SET value = $1 WHERE id = $2",
            &[&value, &id],
        ).await?;

        if updated == 0 {
            return Ok(false);
        }

        let vec_literal = Self::vec_to_pgvector(embedding);
        self.client.execute(
            &format!(
                "UPDATE knowledge_search SET embedding = '{vec_literal}'::vector
                 WHERE store_id = $1"
            ),
            &[&id],
        ).await?;

        Ok(true)
    }
}