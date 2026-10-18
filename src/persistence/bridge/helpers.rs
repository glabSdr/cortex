use std::hash::{DefaultHasher, Hash, Hasher};
use crate::persistence::bridge::Bridge;

impl Bridge {
    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// Formats an f32 slice into a pgvector literal: "[0.1,0.2,0.3]"
    pub fn vec_to_pgvector(v: &[f32]) -> String {
        let inner: Vec<String> = v.iter().map(|x| format!("{x}")).collect();
        format!("[{}]", inner.join(","))
    }

    /// Panics if the vector dimension doesn't match the one given at connect
    pub fn check_dim(&self, embedding: &[f32]) {
        assert_eq!(
            embedding.len() as i32,
            self.vector_dim,
            "embedding dim {} != expected {}",
            embedding.len(),
            self.vector_dim
        );
    }

    pub fn key_obj(id: i64) -> String {
        format!("ks:obj:{id}")
    }

    pub fn key_from(from_id: i64) -> String {
        format!("ks:from:{from_id}")
    }

    pub fn key_rev(id: i64) -> String {
        format!("ks:rev:{id}")
    }

    pub fn key_search(embedding: &[f32]) -> String {
        let mut hasher = DefaultHasher::new();
        for &v in embedding {
            v.to_bits().hash(&mut hasher);
        }
        format!("ks:search:{}", hasher.finish())
    }
}