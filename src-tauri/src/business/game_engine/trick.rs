use std::fmt;

use super::{
    card::{Card, Color},
    engine_error::EngineError,
};

#[derive(Debug, Clone, Copy)]
pub struct PlayedTrick {
    pub cards: [Card; 4],
    pub winner: u8,
}

impl PlayedTrick {
    pub fn points(&self) -> usize {
        self.cards[0].points()
            + self.cards[1].points()
            + self.cards[2].points()
            + self.cards[3].points()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Trick {
    pub cards: [Option<Card>; 4],
    pub leader: u8,
}

impl fmt::Display for Trick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..4 {
            let player = (self.leader + 1) % 4;
            if let Some(card) = self.cards[player as usize] {
                write!(f, "Player {} played {}\n", player, card)?;
            }
        }
        Ok(())
    }
}

impl Trick {
    pub fn new(leader: u8) -> Self {
        Self {
            cards: [None, None, None, None],
            leader,
        }
    }

    pub fn color(&self) -> Option<Color> {
        self.cards[self.leader as usize]
            .map(|leader_card| {
                if leader_card.color != Color::Excuse {
                    Some(leader_card.color)
                } else {
                    let next_player = (self.leader + 1) % 4;
                    self.cards[next_player as usize].map(|card| card.color)
                }
            })
            .flatten()
    }

    pub fn winner(&self) -> Result<u8, EngineError> {
        if let Some(mut winner_card) = self.cards[self.leader as usize] {
            let mut winner: u8 = self.leader;
            let mut color: Color = winner_card.color;
            for i in 0..4 {
                if let Some(compared_card) = self.cards[i as usize] {
                    if compared_card.win_against(&winner_card, &color) {
                        winner_card = compared_card;
                        winner = i;
                        if color == Color::Excuse {
                            color = winner_card.color;
                        }
                    }
                }
            }
            Ok(winner)
        } else {
            Err(EngineError::NotBegunHand)
        }
    }

    pub fn next_to_play(&self) -> Option<u8> {
        let mut result = self.leader;
        let mut left_to_play = 4;
        while self.cards[result as usize].is_some() && left_to_play > 0 {
            result = (result + 1) % 4;
            left_to_play -= 1;
        }
        if left_to_play == 0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn points(&self) -> usize {
        let mut total: usize = 0;
        for i in 0..4 {
            if let Some(card) = self.cards[i] {
                total += card.points();
            }
        }
        total
    }

    pub fn into_played(self) -> Result<PlayedTrick, EngineError> {
        let cards: [Card; 4] = [
            self.cards[0].ok_or(EngineError::UnfinishedHand)?,
            self.cards[1].ok_or(EngineError::UnfinishedHand)?,
            self.cards[2].ok_or(EngineError::UnfinishedHand)?,
            self.cards[3].ok_or(EngineError::UnfinishedHand)?,
        ];
        let winner: u8 = self.winner()?;
        Ok(PlayedTrick { cards, winner })
    }

    pub fn play_card(&mut self, player: u8, card: &Card) -> Result<(), EngineError> {
        match self.next_to_play() {
            None => Err(EngineError::FinishedHand),
            Some(p) if p == player => {
                self.cards[p as usize] = Some(card.clone());
                Ok(())
            }
            _ => Err(EngineError::OutOfOrderPlay),
        }
    }

    pub fn highest_trump(&self) -> u8 {
        let mut result = 0;
        for option_card in self.cards {
            if let Some(card) = option_card {
                if card.color == Color::Trump && card.value > result {
                    result = card.value
                }
            }
        }
        result
    }

    pub fn overtrumped_by(&self, card: &Card) -> bool {
        card.color == Color::Trump && card.value > self.highest_trump()
    }
}
