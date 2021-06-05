#[derive(thiserror::Error, Debug)]
pub enum GaError {
    #[error("Population must be ranked before selection")]
    SelectionBeforeRank,
    #[error("Population must be selected before breed")]
    BreedBeforeSelection,
    #[error("Population must be breed before mutate")]
    MutateBeforeBreed,
    #[error("Best DeciIndividuals Not generated")]
    BestDeciIndividualNotReady,
}
