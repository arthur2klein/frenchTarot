#[derive(Debug)]
pub enum AnalysisError {
    NoCardToPlay,
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::NoCardToPlay => {
                write!(f, "No card anymore",)
            }
        }
    }
}

impl std::error::Error for AnalysisError {}
