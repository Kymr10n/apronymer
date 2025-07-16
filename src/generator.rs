use itertools::Itertools;
use crate::dictionary::is_valid_word;

/// Generate possible apronyms based on input terms
pub fn generate_apronyms(terms: Vec<String>, min_len: usize, max_len: usize) -> Vec<String> {
    let term_variants = permutate(&terms, min_len, max_len);
    match_term(term_variants)
}

fn permutate(terms: &[String], min_len: usize, max_len: usize) -> Vec<Vec<String>> {
    let first_letters: Vec<char> = terms
        .iter()
        .filter_map(|term| term.chars().next())
        .collect();

    let mut results = Vec::new();

    for size in min_len..=max_len {
        for combo in first_letters.iter().copied().combinations(size) {
            let word = combo.into_iter().collect::<String>().to_uppercase();
            results.push(vec![word]);
        }
    }

    results
}

fn match_term(term_variants: Vec<Vec<String>>) -> Vec<String> {
    term_variants
        .iter()
        .multi_cartesian_product()
        .filter_map(|combo| {
            let candidate = combo.iter().map(|s| s.as_str()).collect::<String>();
            if is_valid_word(&candidate) {
                Some(candidate)
            } else {
                None
            }
        })
        .collect()
}