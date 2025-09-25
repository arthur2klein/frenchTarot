use rand::seq::SliceRandom;
use std::collections::HashSet;

use super::{
    card::Card, engine_error::EngineError, game_state::GameState,
    player_game_state::PlayerGameState, shared_game_state::SharedGameState,
};

#[derive(Debug, Clone)]
pub struct KnownGameState {
    pub player_state: PlayerGameState,
    pub player_index: u8,
    pub kitty: Option<[Card; 6]>,
    pub shared_state: SharedGameState,
}

impl KnownGameState {
    pub fn initialize(
        hand: HashSet<Card>,
        player_index: u8,
        kitty: Option<[Card; 6]>,
        dealer: u8,
    ) -> Self {
        Self {
            player_state: PlayerGameState { hand },
            player_index,
            kitty,
            shared_state: SharedGameState::initialize(dealer),
        }
    }

    pub fn from_omniscient(state: &GameState, player: u8) -> Self {
        Self {
            player_state: state.players_state[player as usize].clone(),
            player_index: player,
            kitty: if state.shared_state.kitty_should_be_revealed() {
                Some(state.kitty)
            } else {
                None
            },
            shared_state: state.shared_state.clone(),
        }
    }

    pub fn possible_random_full_state(&self) -> Result<GameState, EngineError> {
        let mut rng = rand::rng();
        let mut left_to_play = self.shared_state.cards_left_to_play();
        for card in &self.player_state.hand {
            left_to_play.remove(card);
        }
        if let Some(kitty) = self.kitty {
            for card in kitty {
                left_to_play.remove(&card);
            }
        }
        let mut cards: Vec<Card> = left_to_play.into_iter().collect();
        cards.shuffle(&mut rng);
        let cards_per_player = (0..4)
            .map(|player| {
                18 - self.shared_state.played_tricks.len()
                    - if self
                        .shared_state
                        .current_trick
                        .map(|trick| trick.cards[player].is_some())
                        .unwrap_or(false)
                    {
                        1
                    } else {
                        0
                    }
            })
            .collect::<Vec<usize>>();
        let hands: [HashSet<Card>; 4] = (0..4)
            .map(|player| {
                cards
                    .drain(0..(cards_per_player[player as usize]))
                    .collect::<HashSet<Card>>()
            })
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| EngineError::RustError(String::from("Could not create hashSet")))?;
        let kitty: [Card; 6] = self.kitty.unwrap_or(
            cards
                .drain(..)
                .collect::<Vec<Card>>()
                .try_into()
                .map_err(|_| EngineError::RustError(String::from("Could not unwrap")))?,
        );
        Ok(GameState::initialize(
            hands,
            kitty,
            self.shared_state.dealer,
        ))
    }
}
