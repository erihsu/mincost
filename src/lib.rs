/// A collection of modern heuristic optimization algorithm written in Rust.
// #[cfg(feature = "ga")]
pub mod ga;
#[cfg(feature = "sa")]
pub mod sa;

#[cfg(feature = "ga")]
pub use ga::*;

#[cfg(feature = "sa")]
pub use sa::*;
