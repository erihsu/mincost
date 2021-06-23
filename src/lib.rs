//! A collection of modern heuristic optimization toolkit.
//!
//! There are tree common optimization methods in the crate currently.
//! 1.generic algorithm
//! 2.simluated annealing
//! 3.particle swarm optimization
//!
//! You can fit any of these methods into your project by enabling relavant features

//! To use generic algorithm
//! ```toml
//! [dependencies]
//! mincost = { version = "0.1.1", features = ["ga"] }
//! ```

//! To use simulated annealing algorithm
//! ```toml
//! [dependencies]
//! mincost = { version = "0.1.1", features = ["sa"] }
//! ```

//! To use particle swarm optimization
//! ```toml
//! [dependencies]
//! mincost = { version = "0.1.1", features = ["pso"] }
//! ```

#[cfg(feature = "ga")]
pub mod ga;
#[cfg(feature = "pso")]
pub mod pso;
#[cfg(feature = "sa")]
pub mod sa;

#[cfg(feature = "ga")]
pub use ga::*;

#[cfg(feature = "sa")]
pub use sa::*;

#[cfg(feature = "pso")]
pub use pso::*;
