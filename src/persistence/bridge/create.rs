use crate::persistence::bridge::{Bridge, CachedRecord, StoreError};
use postgres::Error;

impl Bridge {
    /// Create — stores a value + its vector
    pub fn create(
        &mut self,
        value: &str,
        embedding: &[f32],
        from_id: Option<i64>,
        to_id: Option<i64>,
    ) -> Result<i64, StoreError> {
        self.check_dim(embedding);

        let row = self.client.query_one(
            "INSERT INTO knowledge_store (value, from_id, to_id) VALUES ($1, $2, $3) RETURNING id",
            &[&value, &from_id, &to_id],
        )?;
        let store_id: i64 = row.get(0);

        let vec_literal = Self::vec_to_pgvector(embedding);
        self.client.execute(
            &format!(
                "INSERT INTO knowledge_search (store_id, embedding)
             VALUES ($1, '{vec_literal}'::vector)"
            ),
            &[&store_id],
        )?;

        let record = CachedRecord {
            id: store_id,
            value: value.to_string(),
            from_id,
            to_id,
        };
        self.cache_set_obj(&record);

        Ok(store_id)
    }
}