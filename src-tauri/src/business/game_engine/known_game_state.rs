use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use crate::business::Color;

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

    pub fn get_known_constraints(&self, player: usize) -> Result<PlayerConstraint, EngineError> {
        let known_cards = if player == self.player_index as usize {
            self.player_state.hand.clone()
        } else if let Some(handful) = &self.shared_state.declared_handfuls[player] {
            handful.cards.clone()
        } else {
            HashSet::new()
        };
        let number_cards = 18
            - self.shared_state.played_tricks.len()
            - if self
                .shared_state
                .current_trick
                .map(|trick| trick.cards[player].is_some())
                .unwrap_or(false)
            {
                1
            } else {
                0
            };
        let mut highest_trump = 21;
        let mut voided_colors: HashSet<Color> = HashSet::new();
        for trick in &self.shared_state.played_tricks {
            if trick.did_not_have_color(player) {
                voided_colors.insert(trick.color());
            }
            if let Some(highest_trump_trick) = trick.did_not_have_trump_higher(player) {
                if highest_trump_trick < highest_trump {
                    highest_trump = highest_trump_trick;
                }
            }
        }
        Ok(PlayerConstraint {
            number_cards,
            highest_trump,
            voided_colors,
            known_cards,
        })
    }

    pub fn possible_random_full_state_v2(&self) -> Result<GameState, EngineError> {
        let constraints_per_player: [PlayerConstraint; 4] = [
            self.get_known_constraints(0)?,
            self.get_known_constraints(1)?,
            self.get_known_constraints(2)?,
            self.get_known_constraints(2)?,
        ];
        let left_to_play = self.shared_state.cards_left_to_play();
        let possible_cards: Vec<HashSet<Card>> = (0..4)
            .map(|i| self.possible_cards(&constraints_per_player, &left_to_play, i))
            .collect();
        let known_cards: Vec<HashSet<Card>> = constraints_per_player
            .iter()
            .map(|constraint| constraint.known_cards.clone())
            .collect();
        let remaining_cards: Vec<usize> = constraints_per_player
            .iter()
            .map(|constraint| constraint.number_cards)
            .collect();
        let (hands, kitty) = self.generate_hands(possible_cards, known_cards, remaining_cards)?;
        Ok(GameState::initialize(
            hands,
            kitty
                .iter()
                .map(|item| *item)
                .collect::<Vec<Card>>()
                .try_into()
                .map_err(|_| {
                    EngineError::RustError(String::from("Could not create array from set"))
                })?,
            self.shared_state.dealer,
        ))
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

    pub fn possible_cards(
        &self,
        constraints_per_player: &[PlayerConstraint; 4],
        left_to_play: &HashSet<Card>,
        index: usize,
    ) -> HashSet<Card> {
        let mut res: HashSet<Card> = HashSet::new();
        for card in left_to_play {
            if self
                .shared_state
                .taker
                .is_some_and(|taker| self.player_index != taker)
                && self.kitty.is_some_and(|kitty| kitty.contains(card))
            {
                continue;
            }
            if constraints_per_player[index]
                .voided_colors
                .contains(&card.color)
            {
                continue;
            }
            if constraints_per_player
                .iter()
                .any(|constraint| constraint.known_cards.contains(&card))
            {
                continue;
            }
            if card.color == Color::Trump
                && constraints_per_player[index].highest_trump < card.value
            {
                continue;
            }
            res.insert(card.clone());
        }
        res
    }

    fn generate_hands(
        &self,
        possible_cards: Vec<HashSet<Card>>,
        known_cards: Vec<HashSet<Card>>,
        remaining_cards: Vec<usize>,
    ) -> Result<([HashSet<Card>; 4], HashSet<Card>), EngineError> {
        let mut remaining = remaining_cards.clone();
        let mut rng = rand::rng();
        let mut hands: Vec<HashSet<Card>> = known_cards.clone();
        let mut kitty: HashSet<Card> = HashSet::new();
        for i in 0..4 {
            if hands[i].len() > remaining[i] {
                return Err(EngineError::HandGenerationNotPossible(
                    "Too many known cards".into(),
                ));
            }
            remaining[i] -= hands[i].len();
        }
        let mut deck: Vec<Card> = possible_cards
            .iter()
            .enumerate()
            .flat_map(|(p, set)| set.difference(&hands[p]).cloned())
            .collect();
        deck.shuffle(&mut rng);
        while let Some(card) = deck.pop() {
            let mut candidates: Vec<usize> = (0..4)
                .filter(|&p| remaining[p] > 0 && possible_cards[p].contains(&card))
                .collect();
            if candidates.is_empty() {
                kitty.insert(card);
                continue;
            }
            candidates.sort_by_key(|&p| remaining[p] - possible_cards[p].len());
            let best_score = remaining[candidates[0]] - possible_cards[candidates[0]].len();
            let tightest: Vec<usize> = candidates
                .into_iter()
                .filter(|&p| remaining[p] - possible_cards[p].len() == best_score)
                .collect();
            let chosen = *tightest.choose(&mut rng).unwrap();
            hands[chosen].insert(card);
            remaining[chosen] -= 1;
        }
        for i in 0..4 {
            if remaining[i] != 0 {
                return Err(EngineError::HandGenerationNotPossible(
                    "Unfilled hand".into(),
                ));
            }
        }
        Ok((hands.try_into().unwrap(), kitty))
    }
}

#[derive(Debug, Clone)]
struct PlayerConstraint {
    pub number_cards: usize,
    pub highest_trump: u8,
    pub voided_colors: HashSet<Color>,
    pub known_cards: HashSet<Card>,
}
