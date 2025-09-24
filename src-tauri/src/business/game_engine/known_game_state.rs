use std::collections::HashSet;

use super::{
    card::Card, game_state::GameState, player_game_state::PlayerGameState,
    shared_game_state::SharedGameState,
};

#[derive(Debug, Clone)]
pub struct KnownGameState {
    pub player_state: PlayerGameState,
    pub kitty: Option<[Card; 6]>,
    pub shared_state: SharedGameState,
}

impl KnownGameState {
    pub fn initialize(hand: HashSet<Card>, kitty: Option<[Card; 6]>, dealer: u8) -> Self {
        Self {
            player_state: PlayerGameState { hand },
            kitty,
            shared_state: SharedGameState::initialize(dealer),
        }
    }

    pub fn from_omniscient(state: &GameState, player: u8) -> Self {
        Self {
            player_state: state.players_state[player as usize].clone(),
            kitty: if state.shared_state.kitty_should_be_revealed() {
                Some(state.kitty)
            } else {
                None
            },
            shared_state: state.shared_state.clone(),
        }
    }
}
