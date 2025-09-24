use super::analysis::analysis_error::AnalysisError;
use super::game_engine::engine_error::EngineError;

#[derive(Debug)]
pub enum BusinessError {
    Analysis(AnalysisError),
    Engine(EngineError),
    EveryonePassed,
}

impl From<AnalysisError> for BusinessError {
    fn from(err: AnalysisError) -> Self {
        BusinessError::Analysis(err)
    }
}

impl From<EngineError> for BusinessError {
    fn from(err: EngineError) -> Self {
        BusinessError::Engine(err)
    }
}

impl std::fmt::Display for BusinessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessError::Analysis(error) => write!(f, "{}", error),
            BusinessError::Engine(error) => write!(f, "{}", error),
            BusinessError::EveryonePassed => {
                write!(f, "Could not start the game as everyone passed")
            }
        }
    }
}

impl std::error::Error for BusinessError {}
