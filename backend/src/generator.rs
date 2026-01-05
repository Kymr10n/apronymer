use itertools::Itertools;
use rayon::prelude::*;
use crate::{dictionary::is_valid_word, routes::Apronym};

/// Default maximum number of results to return
pub const DEFAULT_MAX_RESULTS: usize = 100;

/// Minimum total combinations to trigger parallel processing
const PARALLEL_COMBINATIONS_THRESHOLD: usize = 1000;

/// Minimum permutation count to trigger parallel processing
const PARALLEL_PERMUTATIONS_THRESHOLD: usize = 10;

/// Maximum combinations per permutation (safety limit)
const MAX_COMBINATIONS_PER_PERMUTATION: usize = 10_000;

/// Generate possible apronyms based on input terms (optimized version)
/// 
/// This function creates apronyms by taking variable-length prefixes from each term
/// and combining them in different permutations. Uses parallel processing for better performance.
/// 
/// # Arguments
/// * `terms` - Vector of input terms to create apronyms from
/// * `frag_len` - Maximum number of characters to take from each term (1-3)
/// * `min_len` - Minimum number of terms to use in each apronym
/// * `max_len` - Maximum number of terms to use in each apronym
/// 
/// # Returns
/// Vector of valid apronyms that exist in the dictionary (limited to DEFAULT_MAX_RESULTS)
pub fn generate_apronyms(terms: Vec<String>, frag_len: usize, min_len: usize, max_len: usize) -> Vec<Apronym> {
    generate_apronyms_with_limit(terms, frag_len, min_len, max_len, DEFAULT_MAX_RESULTS)
}

/// Generate apronyms with a configurable result limit for performance optimization
/// 
/// This function is primarily used for testing with custom limits.
/// For normal use, prefer `generate_apronyms` which uses DEFAULT_MAX_RESULTS.
pub fn generate_apronyms_with_limit(
    terms: Vec<String>, 
    frag_len: usize, 
    min_len: usize, 
    max_len: usize,
    max_results: usize
) -> Vec<Apronym> {
    tracing::info!(
        "Starting optimized apronym generation: {} terms, frag_len={}, min_len={}, max_len={}, max_results={}", 
        terms.len(), frag_len, min_len, max_len, max_results
    );
    tracing::debug!("Input terms: {:?}", terms);

    // Generate all possible permutations of term indices
    let permutations = permutate(terms.len(), min_len, max_len);
    tracing::debug!("Generated {} permutations", permutations.len());
    
    let total_combinations = permutations.iter()
        .map(|perm| frag_len.pow(perm.len() as u32))
        .sum::<usize>();
    tracing::info!("Processing {} total fragment combinations", total_combinations);

    // Early exit if no permutations
    if permutations.is_empty() {
        return Vec::new();
    }

    // Use adaptive processing: parallel for large workloads, sequential for small ones
    let use_parallel = total_combinations > PARALLEL_COMBINATIONS_THRESHOLD 
        || permutations.len() > PARALLEL_PERMUTATIONS_THRESHOLD;
    
    let matches = if use_parallel {
        tracing::debug!("Using parallel processing for large workload");
        generate_parallel(&terms, &permutations, frag_len, max_results)
    } else {
        tracing::debug!("Using sequential processing for small workload");
        generate_sequential(&terms, &permutations, frag_len, max_results)
    };

    tracing::info!("Generated {} valid apronyms (limit: {})", matches.len(), max_results);
    tracing::debug!("Valid apronyms: {:?}", matches.iter().map(|a| &a.text).collect::<Vec<_>>());
    
    matches
}

/// Generate apronyms using parallel processing for large workloads
fn generate_parallel(
    terms: &[String], 
    permutations: &[Vec<usize>], 
    frag_len: usize, 
    max_results: usize
) -> Vec<Apronym> {
    // Use a shared counter for early termination
    let found_count = std::sync::atomic::AtomicUsize::new(0);
    
    // Use parallel processing to handle permutations concurrently
    let all_matches: Vec<Vec<Apronym>> = permutations
        .par_iter()
        .enumerate()
        .filter_map(|(perm_idx, perm)| {
            // Check if we've already found enough results
            if found_count.load(std::sync::atomic::Ordering::Relaxed) >= max_results {
                return None;
            }
            
            tracing::trace!("Processing permutation {}/{}: {:?}", perm_idx + 1, permutations.len(), perm);
            
            let total_combinations = frag_len.pow(perm.len() as u32);
            
            // Safety check: prevent potential overflow or excessive computation
            if total_combinations > MAX_COMBINATIONS_PER_PERMUTATION {
                tracing::warn!("Skipping permutation with {} combinations (exceeds safety limit of {})", 
                    total_combinations, MAX_COMBINATIONS_PER_PERMUTATION);
                return None;
            }
            
            // Process this permutation and collect matches
            let perm_matches = process_permutation_with_limit(terms, perm, frag_len, total_combinations, max_results, &found_count);
            
            if !perm_matches.is_empty() {
                let new_count = found_count.fetch_add(perm_matches.len(), std::sync::atomic::Ordering::Relaxed);
                tracing::trace!("Found {} matches for permutation {:?} (total: {})", perm_matches.len(), perm, new_count + perm_matches.len());
                Some(perm_matches)
            } else {
                None
            }
        })
        .collect();

    // Flatten the results and take only up to max_results
    all_matches
        .into_iter()
        .flatten()
        .take(max_results)
        .collect()
}

/// Generate apronyms using sequential processing for small workloads
fn generate_sequential(
    terms: &[String], 
    permutations: &[Vec<usize>], 
    frag_len: usize, 
    max_results: usize
) -> Vec<Apronym> {
    let mut matches = Vec::new();
    
    for (perm_idx, perm) in permutations.iter().enumerate() {
        // Early exit if we have enough results
        if matches.len() >= max_results {
            break;
        }
        
        tracing::trace!("Processing permutation {}/{}: {:?}", perm_idx + 1, permutations.len(), perm);
        
        let total_combinations = frag_len.pow(perm.len() as u32);
        
        // Safety check: prevent potential overflow or excessive computation
        if total_combinations > MAX_COMBINATIONS_PER_PERMUTATION {
            tracing::warn!("Skipping permutation with {} combinations (exceeds safety limit of {})", 
                total_combinations, MAX_COMBINATIONS_PER_PERMUTATION);
            continue;
        }
        
        // Process this permutation sequentially
        for i in 0..total_combinations {
            // Early exit if we have enough results
            if matches.len() >= max_results {
                break;
            }
            
            let mut apronym = Apronym {
                text: String::new(),
                terms: Vec::new(),
            };

            // Build the apronym by extracting fragments from each selected term
            for j in 0..perm.len() {
                // Calculate fragment length for term at position j
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
                tracing::trace!("Found valid apronym: '{}' from terms {:?}", apronym.text, apronym.terms);
                matches.push(apronym);
            }
        }
    }
    
    matches
}

/// Process a single permutation to generate apronyms with early termination
/// 
/// This function handles the fragment length combinations for a specific permutation
/// and returns all valid apronyms found, with early exit when limit is reached.
fn process_permutation_with_limit(
    terms: &[String], 
    perm: &[usize], 
    frag_len: usize, 
    total_combinations: usize,
    max_results: usize,
    found_count: &std::sync::atomic::AtomicUsize
) -> Vec<Apronym> {
    let mut matches = Vec::new();
    
    for i in 0..total_combinations {
        // Check if we've already found enough results globally
        if found_count.load(std::sync::atomic::Ordering::Relaxed) >= max_results {
            break;
        }
        
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
            tracing::trace!("Found valid apronym: '{}' from terms {:?}", apronym.text, apronym.terms);
            matches.push(apronym);
        }
    }
    
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
