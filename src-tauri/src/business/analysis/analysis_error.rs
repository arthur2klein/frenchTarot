use crate::business::game_engine::engine_error::EngineError;

#[derive(Debug)]
pub enum AnalysisError {
    NoCardToPlay,
    AnalysisFinished,
    Engine(EngineError),
    RustError(String),
    Other(String),
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::Engine(value) => {
                write!(f, "Got error with tarot game engine: {}", value)
            }
            AnalysisError::NoCardToPlay => {
                write!(f, "No card anymore",)
            }
            AnalysisError::Other(arg) => {
                write!(f, "Other: {}", arg)
            }
            AnalysisError::AnalysisFinished => {
                write!(f, "Analysis already finished",)
            }
            AnalysisError::RustError(arg) => write!(f, "Rust error: {}", arg),
        }
    }
}

impl std::error::Error for AnalysisError {}
