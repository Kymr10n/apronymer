// Library exports for the apronymer backend
// This allows integration tests to access internal modules

pub mod dictionary;
pub mod generator;
pub mod rate_limiter;
pub mod routes;
pub mod validator;

// Re-export commonly used functions for easier access
pub use generator::{generate_apronyms, generate_apronyms_with_limit};
