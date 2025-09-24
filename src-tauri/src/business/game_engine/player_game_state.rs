use std::{collections::HashSet, fmt};

use super::{
    card::{Card, Color},
    engine_error::EngineError,
    trick::Trick,
};

#[derive(Debug, Clone)]
pub struct PlayerGameState {
    pub hand: HashSet<Card>,
}

impl PlayerGameState {
    pub fn use_card(&mut self, card: &Card) -> Result<(), EngineError> {
        self.hand
            .remove(card)
            .then(|| ())
            .ok_or(EngineError::DoesNotHaveCard)
    }

    pub fn cards_allowed(&self, trick: &Trick) -> HashSet<&Card> {
        self.hand
            .iter()
            .filter(|card| self.allowed_to_play(card, trick).is_ok())
            .collect::<HashSet<&Card>>()
    }

    pub fn allowed_to_play(&self, card: &Card, trick: &Trick) -> Result<(), EngineError> {
        if !self.hand.contains(card) {
            return Err(EngineError::DoesNotHaveCard);
        }
        let Some(trick_color) = trick.color() else {
            return Ok(());
        };
        if trick_color != card.color && self.has_color(trick_color) {
            return Err(EngineError::HasToFollowSuit);
        }
        if !self.has_color(trick_color)
            && card.color != Color::Trump
            && self.has_color(Color::Trump)
        {
            return Err(EngineError::HasToTrump);
        }
        if !self.has_color(trick_color)
            && card.color == Color::Trump
            && self.can_overtrump(trick)
            && !trick.overtrumped_by(card)
        {
            return Err(EngineError::HasToOvertrump);
        }
        Ok(())
    }

    pub fn play_a_card(
        &mut self,
        trick: &mut Trick,
        player: u8,
        card: &Card,
    ) -> Result<(), EngineError> {
        if trick
            .next_to_play()
            .map(|next| next != player)
            .unwrap_or(false)
        {
            return Err(EngineError::OutOfOrderPlay);
        }
        self.allowed_to_play(card, trick)?;
        self.use_card(card)?;
        trick.play_card(player, card)
    }

    fn has_color(&self, color: Color) -> bool {
        self.hand.iter().any(|card| card.color == color)
    }

    fn can_overtrump(&self, trick: &Trick) -> bool {
        let highest_trump = trick.highest_trump();
        self.hand
            .iter()
            .any(|card| card.color == Color::Trump && card.value > highest_trump)
    }

    pub fn chose_aside(&mut self, aside: [&Card; 6]) -> Result<(), EngineError> {
        for card in aside {
            self.use_card(card)?;
        }
        Ok(())
    }
}

impl fmt::Display for PlayerGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.hand
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}
