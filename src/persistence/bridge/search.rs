use redis::Commands;
use crate::persistence::bridge::{Bridge, CachedRecord, SearchResult, StoreError, CACHE_TTL_SEC};
use postgres::Error;

impl Bridge {
    // -------------------------------------------------------------------------
    // Search
    // -------------------------------------------------------------------------

    /// Search — vector search, top 5 by cosine similarity.
    /// Returns results if the best score >= novelty_threshold.
    /// Otherwise — an empty vector (= novelty).
    pub fn search(&mut self, embedding: &[f32]) -> Result<Vec<SearchResult>, StoreError> {
        self.check_dim(embedding);

        let search_key = Self::key_search(embedding);

        let cached_ids: Vec<i64> = self.redis.smembers(&search_key).unwrap_or_default();
        if !cached_ids.is_empty() {
            let _: Result<(), _> = self.redis.expire(&search_key, CACHE_TTL_SEC as i64);

            let mut results = Vec::new();
            for oid in &cached_ids {
                if let Some(record) = self.cache_get_obj(*oid) {
                    results.push(SearchResult {
                        id: record.id,
                        value: record.value,
                        from_id: record.from_id,
                        to_id: record.to_id,
                        score: 1.0,
                    });
                }
            }
            if results.len() == cached_ids.len() {
                return Ok(results);
            }
        }

        let vec_literal = Self::vec_to_pgvector(embedding);

        let rows = self.client.query(
            &format!(
                "SELECT s.store_id, k.value, k.from_id, k.to_id,
                    1.0 - (s.embedding <=> '{vec_literal}'::vector) AS score
             FROM knowledge_search s
             JOIN knowledge_store k ON k.id = s.store_id
             ORDER BY s.embedding <=> '{vec_literal}'::vector
             LIMIT 5"
            ),
            &[],
        )?;

        let results: Vec<SearchResult> = rows.iter().map(|r| {
            SearchResult {
                id: r.get(0),
                value: r.get(1),
                from_id: r.get(2),
                to_id: r.get(3),
                score: r.get(4),
            }
        }).collect();

        if results.is_empty() || results[0].score < self.novelty_threshold {
            return Ok(vec![]);
        }

        for result in &results {
            if self.cache_get_obj(result.id).is_none() {
                let record = CachedRecord {
                    id: result.id,
                    value: result.value.clone(),
                    from_id: result.from_id,
                    to_id: result.to_id,
                };
                self.cache_set_obj(&record);
            }
            self.cache_link_search(&search_key, result.id);
        }

        Ok(results)
    }
}