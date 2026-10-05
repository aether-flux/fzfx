use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

pub fn calc_fuzzy(query: &str, candidates: &[String]) -> Vec<f32> {
    let matcher = SkimMatcherV2::default();

    let raw_scores: Vec<f32> = candidates
        .iter()
        .map(|cmd| {
            matcher
                .fuzzy_match(cmd, query)
                .map(|score| score as f32)
                .unwrap_or(0.0)
        })
        .collect();

    let max_score = raw_scores.iter().cloned().fold(0.0, f32::max);

    if max_score == 0.0 {
        return vec![0.0; candidates.len()];
    }

    raw_scores.iter().map(|&s| s / max_score).collect()
}
