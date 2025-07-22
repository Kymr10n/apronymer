use itertools::Itertools;
use crate::{dictionary::is_valid_word, routes::Apronym};
use tracing;

pub struct Fragment {
    text: String, // The text content of the fragment
    term: String, // Index of the term in the original text
}

/// Generate possible apronyms based on input terms
pub fn generate_apronyms(terms: Vec<String>, term_len: usize, min_len: usize, max_len: usize) -> Vec<Apronym> {
    tracing::info!("Generating apronyms: terms={:?}, min_len={}, max_len={}", terms, min_len, max_len);
    
    let mut fragments: Vec<Fragment> = Vec::new(); // Placeholder for future fragment logic

    for term in &terms {
        fragments.push(Fragment {
            text: term.chars().take(term_len).collect(),
            term: term.clone(),
        });
    }
    
    let variants = permutate(fragments.len(), min_len, max_len);
    
    match_terms(variants, &fragments)
}

/// Generate permutations of indices based on term count
fn permutate(term_count: usize, min_len: usize, max_len: usize) -> Vec<Vec<usize>> {
    let indices: Vec<usize> = (0..term_count).collect();

    (min_len..=max_len)
        .flat_map(|size| indices.iter().copied().permutations(size))
        .collect()
}

/// Filter valid apronyms and attach associated terms
fn match_terms(index_combos: Vec<Vec<usize>>, fragments: &[Fragment]) -> Vec<Apronym> {
    tracing::debug!("Matching terms for {} combinations", index_combos.len());
    index_combos
        .into_iter()
        .filter_map(|indices| {
            let name = build_apronym(&indices, fragments);
            if is_valid_word(&name) {
                Some(Apronym {
                    name,
                    terms: terms_by_indices(&indices, fragments),
                })
            } else {
                None
            }
        })
        .collect()
}

/// Build an apronym from first letters of selected terms
fn build_apronym(indices: &[usize], terms: &[Fragment]) -> String {
    indices
        .iter()
        .filter_map(|&i| terms.get(i))
        .filter_map(|fragment| fragment.text.chars().next())
        .collect::<String>()
        .to_uppercase()
}

/// Get subset of terms by indices
fn terms_by_indices(indices: &[usize], fragments: &[Fragment]) -> Vec<String> {
    indices
        .iter()
        .filter_map(|&i| fragments.get(i).map(|f| f.term.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_generate_apronyms_valid_input() {
        let terms = vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()];
        let results = generate_apronyms(terms, 1, 2, 3);
        // Can't assert exact contents due to dictionary check, but should not panic
        assert!(!results.is_empty());
    }

    #[test]
    fn test_generate_apronyms_empty_terms() {
        let terms: Vec<String> = vec![];
        let results = generate_apronyms(terms, 2, 1, 3);
        assert!(results.is_empty());
    }

    #[test]
    fn test_permutate_creates_correct_length() {
        let terms = vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()];
        let variants = permutate(terms.iter().count(), 2, 3);
        assert!(variants.iter().all(|idx| idx.len() >= 2 && idx.len() <= 3));
    }

    #[test]
    fn test_match_terms_filters_valid_words() {
        let terms = vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()];
        let fragments: Vec<Fragment> = terms.iter().map(|t| Fragment { text: t.chars().take(1).collect(), term: t.clone() }).collect();
        let indices = vec![vec![0, 1, 2]];  // assuming indices into terms
        let matches = match_terms(indices, &fragments);
        // Will be empty unless "AEI" exists in dictionary
        assert!(matches.is_empty());
    }
}