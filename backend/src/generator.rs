// ...existing code...
use itertools::Itertools;
use crate::{dictionary::is_valid_word, routes::Apronym};

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
    
    matches
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
