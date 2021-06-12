#[derive(thiserror::Error, Debug)]
pub enum PsoError {
    #[error("Failed to parse pso config")]
    InvalidPsoConfig,
}
