use std::collections::HashSet;

use rand::seq::SliceRandom;

use super::{
    card::{Card, Color},
    player_game_state::PlayerGameState,
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

    pub fn random_init() -> GameState {
        let mut rng = rand::rng();
        let mut cards_num: Vec<usize> = (1..=78).collect();
        cards_num.shuffle(&mut rng);
        let mut cards: Vec<Card> = cards_num
            .into_iter()
            .map(|number| match number {
                1..=14 => Card {
                    color: Color::Spade,
                    value: number as u8,
                },
                15..=28 => Card {
                    color: Color::Club,
                    value: (number - 14) as u8,
                },
                29..=42 => Card {
                    color: Color::Heart,
                    value: (number - 28) as u8,
                },
                43..=56 => Card {
                    color: Color::Diamond,
                    value: (number - 42) as u8,
                },
                57..=77 => Card {
                    color: Color::Trump,
                    value: (number - 56) as u8,
                },
                _ => Card {
                    color: Color::Excuse,
                    value: 0,
                },
            })
            .collect();
        let hands: [HashSet<Card>; 4] = [
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
            cards.drain(0..18).collect(),
        ];
        let kitty: [Card; 6] = cards.drain(..).collect::<Vec<Card>>().try_into().unwrap();
        Self::initialize(hands, kitty, rand::random_range(0..4))
    }
}
