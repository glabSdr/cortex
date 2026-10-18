use crate::persistence::bridge::Bridge;
use tokio_postgres::Error;

impl Bridge {
    /// Delete — удаляет из хранилища (каскад удалит из поиска)
    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        let deleted = self.client.execute(
            "DELETE FROM knowledge_store WHERE id = $1",
            &[&id],
        ).await?;

        Ok(deleted > 0)
    }
}