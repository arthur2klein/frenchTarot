use std::collections::HashSet;

use rand::seq::SliceRandom;

use super::{
    card::Card, engine_error::EngineError, player_game_state::PlayerGameState,
    shared_game_state::SharedGameState,
};

#[derive(Debug, Clone)]
pub struct GameState {
    pub players_state: [PlayerGameState; 4],
    pub kitty: [Card; 6],
    pub shared_state: SharedGameState,
}

impl GameState {
    pub fn initialize(hands: [HashSet<Card>; 4], kitty: [Card; 6], dealer: u8) -> Self {
        Self {
            players_state: hands.map(|hand| PlayerGameState { hand }),
            kitty,
            shared_state: SharedGameState::initialize(dealer),
        }
    }

    pub fn random_init() -> Result<GameState, EngineError> {
        let mut rng = rand::rng();
        let mut cards: Vec<Card> = Card::all_possibles().into_iter().collect();
        cards.shuffle(&mut rng);
        let hands: [HashSet<Card>; 4] = [
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
        ];
        let kitty: [Card; 6] = cards
            .drain(..)
            .collect::<Vec<Card>>()
            .try_into()
            .map_err(|_| EngineError::RustError(String::from("Could not unwrap")))?;
        Ok(Self::initialize(hands, kitty, rand::random_range(0..4)))
    }

    pub fn play_card(&mut self, player_index: u8, card: &Card) -> Result<(), EngineError> {
        let mut current_trick = self
            .shared_state
            .current_trick
            .unwrap_or(self.shared_state.new_trick());
        self.players_state[player_index as usize].play_a_card(
            &mut current_trick,
            player_index,
            card,
        )?;
        if current_trick.next_to_play().is_none() {
            self.shared_state.finish_trick()?;
        }
        Ok(())
    }

    pub fn cards_allowed(&self, player: u8) -> HashSet<&Card> {
        self.shared_state
            .current_trick
            .map(|trick| self.players_state[player as usize].cards_allowed(&trick))
            .unwrap_or(HashSet::new())
    }
}
