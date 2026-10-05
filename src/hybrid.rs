use crate::fuzzy::calc_fuzzy;

#[derive(Debug)]
pub struct HybridMatch {
    pub command: String,
    pub final_score: f32,
    pub _vector_score: f32,
    pub _fuzzy_score: f32,
}

impl HybridMatch {
    pub fn rerank(query: &str, candidates: Vec<(String, f32)>, alpha: f32) -> Vec<HybridMatch> {
        let commands: Vec<String> = candidates.iter().map(|(c, _)| c.clone()).collect();
        let fuzzy_scores = calc_fuzzy(query, &commands);

        let mut hybrid: Vec<HybridMatch> = candidates
            .into_iter()
            .zip(fuzzy_scores)
            .map(|((cmd, vec_score), fuz_score)| {
                let vec_norm = vec_score.max(0.0);

                let final_score = (alpha * vec_norm) + ((1.0 - alpha) * fuz_score);
                HybridMatch {
                    command: cmd,
                    final_score,
                    _vector_score: vec_norm,
                    _fuzzy_score: fuz_score,
                }
            })
            .collect();

        hybrid.sort_by(|a, b| b.final_score.total_cmp(&a.final_score));
        hybrid
    }
}
