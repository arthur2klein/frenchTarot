use super::{
    analysis::analysis_error::AnalysisError,
    game_engine::{card::Card, game_type::GameType, known_game_state::KnownGameState},
};

pub trait Player {
    fn play_a_card(&self, game_state: KnownGameState) -> Result<Card, AnalysisError>;

    fn bid(&self, game_state: KnownGameState) -> Result<Option<GameType>, AnalysisError>;

    fn chose_aside(&self, game_state: KnownGameState) -> Result<[&Card; 6], AnalysisError>;
}
