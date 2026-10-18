use crate::persistence::bridge::Bridge;

impl Bridge {
    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// Форматирует f32-слайс в pgvector literal: "[0.1,0.2,0.3]"
    pub fn vec_to_pgvector(v: &[f32]) -> String {
        let inner: Vec<String> = v.iter().map(|x| format!("{x}")).collect();
        format!("[{}]", inner.join(","))
    }

    /// Паника если размерность вектора не совпадает с заданной при connect
    pub fn check_dim(&self, embedding: &[f32]) {
        assert_eq!(
            embedding.len() as i32,
            self.vector_dim,
            "embedding dim {} != expected {}",
            embedding.len(),
            self.vector_dim
        );
    }
}