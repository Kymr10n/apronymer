use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Global dictionary loaded once at startup
static DICTIONARY: Lazy<RwLock<HashSet<String>>> = Lazy::new(|| {
    let mut set = HashSet::new();
    if let Ok(file) = File::open("wordlist/words.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                set.insert(word.trim().to_uppercase());
            }
        }
    }
    RwLock::new(set)
});

/// Check if word exists in dictionary
pub fn is_valid_word(word: &str) -> bool {
    DICTIONARY
        .read()
        .unwrap()
        .contains(&word.to_uppercase())
}

/// Expose this for testing / debug if needed
pub fn word_count() -> usize {
    DICTIONARY.read().unwrap().len()
}