use crate::persistence::bridge::{Bridge, StoreError};
use postgres::Error;

impl Bridge {
    /// Delete — removes from storage (cascade will remove from search)
    pub fn delete(&mut self, id: i64) -> Result<bool, StoreError> {
        self.invalidate_by_id(id);

        let deleted = self.client.execute(
            "DELETE FROM knowledge_store WHERE id = $1",
            &[&id],
        )?;

        Ok(deleted > 0)
    }
}