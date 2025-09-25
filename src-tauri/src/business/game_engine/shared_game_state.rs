use std::{collections::HashSet, fmt};

use crate::business::Card;

use super::{
    engine_error::EngineError,
    game_type::GameType,
    handfuls::Handfuls,
    trick::{PlayedTrick, Trick},
};

#[derive(Debug, Clone)]
pub struct SharedGameState {
    pub dealer: u8,
    pub taker: Option<u8>,
    pub current_trick: Option<Trick>,
    pub played_tricks: Vec<PlayedTrick>,
    pub game_type: Option<GameType>,
    pub declared_handfuls: [Option<Handfuls>; 4],
}

impl fmt::Display for SharedGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current round's dealer is {}\n", self.dealer)?;
        if let Some(taker) = self.taker {
            writeln!(f, "Current round's taker is {}", taker)?;
        }
        if let Some(game_type) = self.game_type {
            writeln!(f, " with bid {}\n", game_type)?;
            writeln!(f, "Taker's score is {}", self.current_score())?;
        }

        if let Some(current_trick) = self.current_trick {
            writeln!(f, "current trick is as follow: {}", current_trick)?;
        }
        Ok(())
    }
}

impl SharedGameState {
    pub fn player_to_lead(&self) -> u8 {
        match self.played_tricks.last() {
            None => (self.dealer + 1) % 4,
            Some(trick) => trick.winner,
        }
    }

    pub fn finish_trick(&mut self) -> Result<(), EngineError> {
        if let Some(trick) = self.current_trick {
            self.played_tricks.push(trick.into_played()?);
            Ok(())
        } else {
            Err(EngineError::NotBegunHand)
        }
    }

    pub fn bid(&mut self, player: u8, bid: Option<GameType>) -> Result<(), EngineError> {
        match (self.game_type, bid) {
            (Some(previous_bid), Some(bid)) if (previous_bid > bid) => Err(EngineError::InvalidBid),
            (None, _) | (_, Some(_)) => {
                self.taker = Some(player);
                self.game_type = bid;
                Ok(())
            }
            (_, None) => Ok(()),
        }
    }

    pub fn initialize(dealer: u8) -> Self {
        Self {
            dealer,
            taker: None,
            current_trick: None,
            played_tricks: vec![],
            game_type: None,
            declared_handfuls: [None, None, None, None],
        }
    }

    pub fn current_score(&self) -> usize {
        let mut total: usize = 0;
        let Some(current_taker) = self.taker else {
            return total;
        };
        for trick in &self.played_tricks {
            if trick.winner == current_taker {
                total += trick.points();
            }
        }
        total
    }

    pub fn kitty_should_be_revealed(&self) -> bool {
        self.game_type
            .map(|game_type| game_type.kitty_should_be_revealed())
            .unwrap_or(false)
    }

    pub fn finished(&self) -> bool {
        self.played_tricks.len() == 18
    }

    pub fn next_to_play(&self) -> Option<u8> {
        (!self.finished()).then(|| {
            self.current_trick
                .map(|trick| trick.next_to_play())
                .flatten()
                .unwrap_or(self.player_to_lead())
        })
    }

    pub fn new_trick(&mut self) -> Trick {
        self.current_trick = Some(Trick::new(self.player_to_lead()));
        self.current_trick.unwrap()
    }

    pub fn cards_left_to_play(&self) -> HashSet<Card> {
        let mut all_cards = Card::all_possibles();
        for trick in &self.played_tricks {
            for card in trick.cards {
                all_cards.remove(&card);
            }
        }
        all_cards
    }
}
