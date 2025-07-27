// Library exports for the apronymer backend
// This allows integration tests to access internal modules

pub mod routes;
pub mod validator;
pub mod generator;
pub mod dictionary;
pub mod rate_limiter;

// Re-export commonly used functions for easier access
pub use generator::{generate_apronyms, generate_apronyms_with_limit};
