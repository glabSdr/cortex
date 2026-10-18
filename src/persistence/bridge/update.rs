use crate::persistence::bridge::{Bridge, CachedRecord, StoreError};
use postgres::Error;

impl Bridge {
    /// Update — updates the value and vector
    pub fn update(
        &mut self,
        id: i64,
        value: &str,
        embedding: &[f32],
        from_id: Option<i64>,
        to_id: Option<i64>,
    ) -> Result<bool, StoreError> {
        self.check_dim(embedding);

        self.invalidate_by_id(id);

        let updated = self.client.execute(
            "UPDATE knowledge_store SET value = $1, from_id = $3, to_id = $4 WHERE id = $2",
            &[&value, &id, &from_id, &to_id],
        )?;

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
        )?;

        let record = CachedRecord {
            id,
            value: value.to_string(),
            from_id,
            to_id,
        };
        self.cache_set_obj(&record);

        Ok(true)
    }
}