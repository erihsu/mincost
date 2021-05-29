#[derive(thiserror::Error, Debug)]
pub enum GaError {
    #[error("Population must be ranked before selection")]
    SelectionBeforeRank,
    #[error("Population must be selected before breed")]
    BreedBeforeSelection,
    #[error("Population must be breed before mutate")]
    MutateBeforeBreed,
    #[error("Best Individuals Not generated")]
    BestIndividualNotReady,
    #[error("Failed to parse ga config")]
    InvalidGaConfig {
        #[from]
        source: serde_yaml::Error,
    },
    #[error("Not found config yaml")]
    ConfigNotFound {
        #[from]
        source: std::io::Error,
    },
}
