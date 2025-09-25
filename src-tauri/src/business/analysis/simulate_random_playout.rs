use rand::{rng, seq::IteratorRandom};

use crate::business::{
    analysis::analysis_error::AnalysisError, game_engine::game_state::GameState,
};

pub fn simulate_random_playout(full: &mut GameState) -> Result<usize, AnalysisError> {
    let mut rng = rng();
    while let Some(player) = full.shared_state.next_to_play() {
        let card = full
            .cards_allowed(player)
            .iter()
            .choose(&mut rng)
            .cloned()
            .cloned()
            .ok_or(AnalysisError::NoCardToPlay)?;
        full.play_card(player, &card)
            .map_err(|_| AnalysisError::Other(String::from("Can not play a card")))?;
    }
    Ok(full.shared_state.current_score())
}
