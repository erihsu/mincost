#[derive(thiserror::Error, Debug)]
pub enum SaError {
    #[error("Failed to parse sa config")]
    InvalidSaConfig,
}
