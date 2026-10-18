use crate::persistence::bridge::{Bridge, CachedRecord, StoreError, CACHE_TTL_SEC};
use postgres::Error;
use redis::Commands;

impl Bridge {
    /// ReadOne — by id from storage
    pub fn read_one(&mut self, id: i64) -> Result<Option<CachedRecord>, StoreError> {
        if let Some(cached) = self.cache_get_obj(id) {
            return Ok(Some(cached));
        }

        let rows = self.client.query(
            "SELECT value, from_id, to_id FROM knowledge_store WHERE id = $1",
            &[&id],
        )?;

        match rows.first() {
            Some(r) => {
                let record = CachedRecord {
                    id,
                    value: r.get(0),
                    from_id: r.get(1),
                    to_id: r.get(2),
                };
                self.cache_set_obj(&record);
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    pub fn read_by_from(&mut self, from_id: i64) -> Result<Vec<CachedRecord>, StoreError> {
        let from_key = Self::key_from(from_id);

        let cached_ids: Vec<i64> = self.redis.smembers(&from_key).unwrap_or_default();
        if !cached_ids.is_empty() {
            let _: Result<(), _> = self.redis.expire(&from_key, CACHE_TTL_SEC as i64);

            let mut results = Vec::new();
            for oid in &cached_ids {
                if let Some(record) = self.cache_get_obj(*oid) {
                    results.push(record);
                }
            }
            if results.len() == cached_ids.len() {
                return Ok(results);
            }
        }

        let rows = self.client.query(
            "SELECT id, value, from_id, to_id FROM knowledge_store WHERE from_id = $1",
            &[&from_id],
        )?;

        let mut results = Vec::new();
        for row in &rows {
            let record = CachedRecord {
                id: row.get(0),
                value: row.get(1),
                from_id: row.get(2),
                to_id: row.get(3),
            };

            if self.cache_get_obj(record.id).is_none() {
                self.cache_set_obj(&record);
            }
            self.cache_link_from(from_id, record.id);

            results.push(record);
        }

        Ok(results)
    }

    /// ReadAll — all records from storage
    pub fn read_all(&mut self) -> Result<Vec<CachedRecord>, StoreError> {
        let rows = self.client.query(
            "SELECT id, value, from_id, to_id FROM knowledge_store ORDER BY id",
            &[],
        )?;

        Ok(rows.iter().map(|r| CachedRecord {
            id: r.get(0),
            value: r.get(1),
            from_id: r.get(2),
            to_id: r.get(3),
        }).collect())
    }
}