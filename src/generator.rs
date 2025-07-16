use itertools::Itertools;

/// Generate possible apronyms based on input terms
pub fn generate_apronyms(terms: Vec<String>, min_len: usize, max_len: usize) -> Vec<String> {
    let mut results = Vec::new();

    // Example: Take first 1..n letters of each term and create combinations
    let term_variants: Vec<Vec<String>> = terms
        .iter()
        .map(|term| {
            (1..=term.len().min(3)) // limit to first 3 letters max per word
                .map(|i| term[..i].to_uppercase())
                .collect()
        })
        .collect();

    // Cartesian product of variants
    for combo in term_variants.iter().multi_cartesian_product() {
        let candidate = combo.iter().map(|s| s.as_str()).collect::<String>();
        if candidate.len() >= min_len && candidate.len() <= max_len {
            // Here you’d check against a dictionary
            if is_valid_word(&candidate) {
                results.push(candidate);
            }
        }
    }

    results
}

/// Dummy dictionary check — replace with real lookup
fn is_valid_word(word: &str) -> bool {
    true
}