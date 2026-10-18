use crate::persistence::bridge::{Bridge, SearchResult};
use tokio_postgres::Error;

impl Bridge {
    // -------------------------------------------------------------------------
    // Search
    // -------------------------------------------------------------------------

    /// Search — векторный поиск, топ-5 по cosine similarity.
    /// Возвращает результаты если лучший score >= novelty_threshold.
    /// Иначе — пустой вектор (= новизна).
    pub async fn search(&self, embedding: &[f32]) -> Result<Vec<SearchResult>, Error> {
        self.check_dim(embedding);

        let vec_literal = Self::vec_to_pgvector(embedding);

        // cosine distance в pgvector: <=> возвращает (1 - cosine_similarity)
        // поэтому score = 1 - distance
        let rows = self.client.query(
            &format!(
                "SELECT s.store_id, k.value,
                        1.0 - (s.embedding <=> '{vec_literal}'::vector) AS score
                 FROM knowledge_search s
                 JOIN knowledge_store k ON k.id = s.store_id
                 ORDER BY s.embedding <=> '{vec_literal}'::vector
                 LIMIT 5"
            ),
            &[],
        ).await?;

        let results: Vec<SearchResult> = rows.iter().map(|r| {
            SearchResult {
                id: r.get(0),
                value: r.get(1),
                score: r.get(2),
            }
        }).collect();

        // Если лучший результат ниже порога — это новизна, возвращаем пусто
        if results.is_empty() || results[0].score < self.novelty_threshold {
            return Ok(vec![]);
        }

        Ok(results)
    }
}