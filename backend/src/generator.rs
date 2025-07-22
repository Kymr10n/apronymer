use itertools::Itertools;
use crate::{dictionary::is_valid_word, routes::Apronym};
use tracing;

/// Generate possible apronyms based on input terms
/// 
/// This function creates apronyms by taking variable-length prefixes from each term
/// and combining them in different permutations. For each permutation, it generates
/// all possible combinations of prefix lengths (1 to frag_len) for each selected term.
/// 
/// # Arguments
/// * `terms` - Vector of input terms to create apronyms from
/// * `frag_len` - Maximum number of characters to take from each term (1-3)
/// * `min_len` - Minimum number of terms to use in each apronym
/// * `max_len` - Maximum number of terms to use in each apronym
/// 
/// # Returns
/// Vector of valid apronyms that exist in the dictionary
pub fn generate_apronyms(terms: Vec<String>, frag_len: usize, min_len: usize, max_len: usize) -> Vec<Apronym> {
    tracing::info!(
        "Starting apronym generation: {} terms, frag_len={}, min_len={}, max_len={}", 
        terms.len(), frag_len, min_len, max_len
    );
    tracing::debug!("Input terms: {:?}", terms);

    let mut matches = Vec::new();

    // Generate all possible permutations of term indices
    let permutations = permutate(terms.len(), min_len, max_len);
    tracing::debug!("Generated {} permutations", permutations.len());
    
    let total_combinations = permutations.iter()
        .map(|perm| frag_len.pow(perm.len() as u32))
        .sum::<usize>();
    tracing::info!("Processing {} total fragment combinations", total_combinations);

    for (perm_idx, perm) in permutations.iter().enumerate() {
        tracing::debug!("Processing permutation {}/{}: {:?}", perm_idx + 1, permutations.len(), perm);
        
        // For each permutation, generate all possible fragment length combinations
        // i represents a number in base frag_len, where each digit corresponds to 
        // the fragment length (1-frag_len) for the term at that position
        let total_combinations = frag_len.pow(perm.len() as u32);
        
        // Safety check: prevent potential overflow or excessive computation
        if total_combinations > 10_000 {
            tracing::warn!("Skipping permutation with {} combinations (exceeds safety limit)", total_combinations);
            continue;
        }
        
        for i in 0..total_combinations {
            let mut apronym = Apronym {
                text: String::new(),
                terms: Vec::new(),
            };

            // Build the apronym by extracting fragments from each selected term
            for j in 0..perm.len() {
                // Calculate fragment length for term at position j
                // This uses base frag_len arithmetic to generate all combinations
                let fragment_length = (i / frag_len.pow(j as u32) % frag_len) + 1;
                
                let term = &terms[perm[j]];
                
                // Safety check: ensure fragment_length doesn't exceed term length
                let safe_fragment_length = fragment_length.min(term.chars().count());
                let fragment: String = term.chars().take(safe_fragment_length).collect();
                
                apronym.text += &fragment;
                apronym.terms.push(term.clone());
            }

            tracing::trace!("Generated candidate: '{}' from terms {:?}", apronym.text, apronym.terms);

            // Check if the generated text is a valid dictionary word
            if is_valid_word(&apronym.text) {
                tracing::debug!("Found valid apronym: '{}' from terms {:?}", apronym.text, apronym.terms);
                matches.push(apronym);
            }
        }
    }

    tracing::info!("Generated {} valid apronyms", matches.len());
    tracing::debug!("Valid apronyms: {:?}", matches.iter().map(|a| &a.text).collect::<Vec<_>>());
    
    return matches;
}

/// Generate permutations of indices based on term count
/// 
/// Creates all possible permutations of term indices for apronym lengths
/// between min_len and max_len (inclusive).
/// 
/// # Arguments
/// * `term_count` - Total number of available terms
/// * `min_len` - Minimum number of terms to select
/// * `max_len` - Maximum number of terms to select
/// 
/// # Returns
/// Vector of permutations, where each permutation is a vector of term indices
/// 
/// # Example
/// ```
/// // For 3 terms with min_len=2, max_len=2:
/// // Returns: [[0,1], [0,2], [1,0], [1,2], [2,0], [2,1]]
/// ```
fn permutate(term_count: usize, min_len: usize, max_len: usize) -> Vec<Vec<usize>> {
    tracing::debug!("Generating permutations: {} terms, lengths {}-{}", term_count, min_len, max_len);
    
    let indices: Vec<usize> = (0..term_count).collect();

    let result: Vec<Vec<usize>> = (min_len..=max_len)
        .flat_map(|size| {
            tracing::trace!("Generating permutations of size {}", size);
            indices.iter().copied().permutations(size)
        })
        .collect();
    
    tracing::debug!("Generated {} total permutations", result.len());
    result
}
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_generate_apronyms_valid_input() {
        let terms = vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()];
        let _results = generate_apronyms(terms, 1, 2, 3);
        // Can't assert exact contents due to dictionary check, but should not panic
        // This test ensures the function completes without errors
    }

    #[test]
    fn test_generate_apronyms_empty_terms() {
        let terms: Vec<String> = vec![];
        let results = generate_apronyms(terms, 1, 1, 3);
        assert!(results.is_empty(), "Empty terms should return empty results");
    }

    #[test]
    fn test_generate_apronyms_single_term() {
        let terms = vec!["Test".to_string()];
        let _results = generate_apronyms(terms, 2, 1, 1);
        // Should generate apronyms with prefixes "T" and "Te"
        // Results depend on dictionary, but function should not panic
    }

    #[test]
    fn test_generate_apronyms_different_frag_lengths() {
        let terms = vec!["Auto".to_string(), "Baum".to_string()];
        
        // Test with frag_len = 1 (first letters only)
        let _results_1 = generate_apronyms(terms.clone(), 1, 2, 2);
        
        // Test with frag_len = 2 (up to 2 characters)
        let _results_2 = generate_apronyms(terms.clone(), 2, 2, 2);
        
        // Test with frag_len = 3 (up to 3 characters)
        let _results_3 = generate_apronyms(terms, 3, 2, 2);
        
        // Results may vary based on dictionary, but should not panic
    }

    #[test]
    fn test_generate_apronyms_min_max_len_constraints() {
        let terms = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
        
        // Test min_len = max_len = 2 (exactly 2 terms)
        let _results_2 = generate_apronyms(terms.clone(), 1, 2, 2);
        
        // Test min_len = max_len = 3 (exactly 3 terms)
        let _results_3 = generate_apronyms(terms.clone(), 1, 3, 3);
        
        // Test min_len = 1, max_len = 3 (1 to 3 terms)
        let _results_range = generate_apronyms(terms, 1, 1, 3);
        
        // All should complete without panic
    }

    #[test]
    fn test_permutate_correct_count() {
        // Test permutations of 3 items taken 2 at a time
        // Should be P(3,2) = 3!/(3-2)! = 6 permutations
        let result = permutate(3, 2, 2);
        assert_eq!(result.len(), 6, "Should generate 6 permutations for P(3,2)");
        
        // Check that all permutations have length 2
        assert!(result.iter().all(|perm| perm.len() == 2), "All permutations should have length 2");
        
        // Check that permutations contain valid indices
        assert!(result.iter().all(|perm| perm.iter().all(|&idx| idx < 3)), "All indices should be < 3");
    }

    #[test]
    fn test_permutate_range() {
        // Test permutations with a range of lengths
        let result = permutate(3, 1, 2);
        
        // Should include:
        // - 3 permutations of length 1: [0], [1], [2]
        // - 6 permutations of length 2: [0,1], [0,2], [1,0], [1,2], [2,0], [2,1]
        // Total: 9 permutations
        assert_eq!(result.len(), 9, "Should generate 9 permutations for lengths 1-2");
        
        // Count permutations by length
        let len_1_count = result.iter().filter(|perm| perm.len() == 1).count();
        let len_2_count = result.iter().filter(|perm| perm.len() == 2).count();
        
        assert_eq!(len_1_count, 3, "Should have 3 permutations of length 1");
        assert_eq!(len_2_count, 6, "Should have 6 permutations of length 2");
    }

    #[test]
    fn test_permutate_empty() {
        let result = permutate(0, 1, 2);
        assert!(result.is_empty(), "No terms should result in no permutations");
    }

    #[test]
    fn test_permutate_single_term() {
        let result = permutate(1, 1, 1);
        assert_eq!(result.len(), 1, "Single term should generate one permutation");
        assert_eq!(result[0], vec![0], "Single permutation should be [0]");
    }

    #[test]
    fn test_permutate_invalid_range() {
        // Test when min_len > max_len
        let result = permutate(3, 3, 2);
        assert!(result.is_empty(), "Invalid range (min > max) should return empty");
    }

    #[test]
    fn test_permutate_exceeds_term_count() {
        // Test when min_len > term_count
        let result = permutate(2, 3, 3);
        assert!(result.is_empty(), "Requesting more terms than available should return empty");
    }

    #[test]
    fn test_fragment_length_calculation() {
        // This test verifies the fragment length calculation logic
        // For frag_len=2, perm.len()=2, we should get combinations:
        // i=0: fragment_lengths=[1,1], i=1: [2,1], i=2: [1,2], i=3: [2,2]
        
        let frag_len: usize = 2;
        let perm_len: usize = 2;
        let expected_combinations = vec![
            vec![1, 1], // i=0
            vec![2, 1], // i=1  
            vec![1, 2], // i=2
            vec![2, 2], // i=3
        ];
        
        for i in 0..frag_len.pow(perm_len as u32) {
            let mut actual_lengths = Vec::new();
            for j in 0..perm_len {
                let fragment_length = (i / frag_len.pow(j as u32) % frag_len) + 1;
                actual_lengths.push(fragment_length);
            }
            assert_eq!(actual_lengths, expected_combinations[i], 
                "Fragment length calculation for i={} should match expected", i);
        }
    }

    #[test]
    fn test_apronym_structure() {
        // Test that Apronym structure is correctly populated
        let terms = vec!["Test".to_string(), "Word".to_string()];
        let results = generate_apronyms(terms.clone(), 1, 2, 2);
        
        // Verify that all results have non-empty text and terms
        for apronym in &results {
            assert!(!apronym.text.is_empty(), "Apronym text should not be empty");
            assert!(!apronym.terms.is_empty(), "Apronym terms should not be empty");
            assert!(apronym.terms.len() >= 1 && apronym.terms.len() <= 2, 
                "Apronym should have 1-2 terms as requested");
            
            // Verify that all terms are from the original input
            for term in &apronym.terms {
                assert!(terms.contains(term), "All apronym terms should be from input terms");
            }
        }
    }
}