// ...existing code...
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Global dictionary loaded once at startup
static DICTIONARY: Lazy<RwLock<HashSet<String>>> = Lazy::new(|| {
    println!("🔄 Initializing dictionary...");
    let mut set = HashSet::new();
    
    // Use container path if it exists, otherwise use local development path
    let dict_path = if std::path::Path::new("/app/wordlist/words.txt").exists() {
        "/app/wordlist/words.txt"
    } else {
        "./wordlist/words.txt"
    };
    println!("📖 Loading dictionary from: {}", dict_path);
    
    match File::open(dict_path) {
        Ok(file) => {
            println!("✅ Dictionary file opened successfully");
            let reader = BufReader::new(file);
            let mut word_count = 0;
            
            for (line_num, line) in reader.lines().enumerate() {
                match line {
                    Ok(word) => {
                        let trimmed = word.trim().to_uppercase();
                        if !trimmed.is_empty() {
                            set.insert(trimmed);
                            word_count += 1;
                        }
                    }
                    Err(e) => {
                        println!("⚠️ Failed to read line {}: {}", line_num + 1, e);
                    }
                }
            }
            
            println!("✅ Dictionary loaded successfully with {} words", word_count);
            if word_count == 0 {
                println!("❌ Dictionary is empty! This will cause apronym generation to fail");
            }
        }
        Err(e) => {
            println!("❌ Failed to open dictionary file '{}': {}", dict_path, e);
            println!("💡 Current working directory: {:?}", std::env::current_dir());
            
            // Try to list the directory to see what's there
            if let Ok(entries) = std::fs::read_dir("/app") {
                println!("📁 Contents of /app directory:");
                for entry in entries.flatten() {
                    println!("  - {:?}", entry.path());
                }
            }
            
            if let Ok(entries) = std::fs::read_dir("/app/wordlist") {
                println!("📁 Contents of /app/wordlist directory:");
                for entry in entries.flatten() {
                    println!("  - {:?}", entry.path());
                }
            } else {
                println!("❌ /app/wordlist directory does not exist or cannot be read");
            }
            
            println!("🚨 Dictionary loading failed - apronym generation will not work!");
        }
    }
    
    RwLock::new(set)
});

/// Get dictionary statistics for debugging
pub fn get_dictionary_stats() -> (usize, bool) {
    match DICTIONARY.read() {
        Ok(dict) => {
            let size = dict.len();
            let has_words = size > 0;
            (size, has_words)
        }
        Err(e) => {
            tracing::error!("Failed to read dictionary stats: {}", e);
            (0, false)
        }
    }
}

/// Check if word exists in dictionary
/// Returns false if dictionary access fails (defensive programming)
pub fn is_valid_word(word: &str) -> bool {
    match DICTIONARY.read() {
        Ok(dict) => dict.contains(&word.to_uppercase()),
        Err(e) => {
            tracing::error!("Dictionary lock poisoned: {}", e);
            false // Fail safely - don't crash the server
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dictionary_loads_and_contains_word() {
        // The dictionary should load and contain at least one word (if words.txt is present)
        let dict = DICTIONARY.read().unwrap();
        // This test passes if the dictionary is not empty
        assert!(!dict.is_empty(), "Dictionary should not be empty if words.txt is present");
    }
}