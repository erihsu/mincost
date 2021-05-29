#[derive(thiserror::Error, Debug)]
pub enum SaError {
    #[error("Failed to parse sa config")]
    InvalidSaConfig {
        #[from]
        source: serde_yaml::Error,
    },
    #[error("Not found config yaml")]
    ConfigNotFound {
        #[from]
        source: std::io::Error,
    },
}
