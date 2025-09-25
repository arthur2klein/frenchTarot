use super::{
    analysis::analysis_error::AnalysisError,
    game_engine::{card::Card, game_type::GameType, known_game_state::KnownGameState},
};

pub trait Player {
    fn play_a_card(&mut self, game_state: &KnownGameState) -> Result<Card, AnalysisError>;

    fn bid(&self, _game_state: &KnownGameState) -> Result<Option<GameType>, AnalysisError> {
        Ok(Some(GameType::GardeSans { chelem: false }))
    }

    fn chose_aside(&self, game_state: &KnownGameState) -> Result<[Card; 6], AnalysisError> {
        let chosen: Vec<Card> = game_state
            .player_state
            .hand
            .iter()
            .cloned()
            .take(6)
            .collect();

        chosen.try_into().map_err(|_| AnalysisError::NoCardToPlay)
    }
}
