//! HTTP layer: fetcher and response normaliser.

pub mod fetcher;
pub mod normalizer;

pub use fetcher::{Fetcher, FetcherConfig};
pub use normalizer::Normalizer;
