use itertools::Itertools;
use crate::{dictionary::is_valid_word, routes::Apronym};

/// Generate possible apronyms based on input terms
pub fn generate_apronyms(terms: Vec<String>, min_len: usize, max_len: usize) -> Vec<Apronym> {
    let variants = permutate(terms.len(), min_len, max_len);
    match_terms(variants, &terms)
}

/// Generate permutations of indices based on term count
fn permutate(term_count: usize, min_len: usize, max_len: usize) -> Vec<Vec<usize>> {
    let indices: Vec<usize> = (0..term_count).collect();

    (min_len..=max_len)
        .flat_map(|size| indices.iter().copied().permutations(size))
        .collect()
}

/// Filter valid apronyms and attach associated terms
fn match_terms(index_combos: Vec<Vec<usize>>, terms: &[String]) -> Vec<Apronym> {
    index_combos
        .into_iter()
        .filter_map(|indices| {
            let name = build_apronym(&indices, terms);
            if is_valid_word(&name) {
                Some(Apronym {
                    name,
                    terms: terms_by_indices(&indices, terms),
                })
            } else {
                None
            }
        })
        .collect()
}

/// Build an apronym from first letters of selected terms
fn build_apronym(indices: &[usize], terms: &[String]) -> String {
    indices
        .iter()
        .filter_map(|&i| terms.get(i))
        .filter_map(|term| term.chars().next())
        .collect::<String>()
        .to_uppercase()
}

/// Get subset of terms by indices
fn terms_by_indices(indices: &[usize], terms: &[String]) -> Vec<String> {
    indices
        .iter()
        .filter_map(|&i| terms.get(i).cloned())
        .collect()
}