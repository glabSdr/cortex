use crate::persistence::bridge::Bridge;
use tokio_postgres::Error;

impl Bridge {
    /// ReadOne — по id из хранилища
    pub async fn read_one(&self, id: i64) -> Result<Option<String>, Error> {
        let rows = self.client.query(
            "SELECT value FROM knowledge_store WHERE id = $1",
            &[&id],
        ).await?;

        Ok(rows.first().map(|r| r.get(0)))
    }

    /// ReadAll — все записи из хранилища
    pub async fn read_all(&self) -> Result<Vec<(i64, String)>, Error> {
        let rows = self.client.query(
            "SELECT id, value FROM knowledge_store ORDER BY id",
            &[],
        ).await?;

        Ok(rows.iter().map(|r| (r.get(0), r.get(1))).collect())
    }
}