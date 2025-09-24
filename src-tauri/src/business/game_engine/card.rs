use super::engine_error::EngineError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Spade,
    Heart,
    Diamond,
    Club,
    Trump,
    Excuse,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Card {
    pub color: Color,
    pub value: u8,
}

impl Card {
    pub fn new(color: Color, value: u8) -> Result<Card, EngineError> {
        let res = Card { color, value };
        res.validate()?;
        Ok(res)
    }

    pub fn validate(&self) -> Result<(), EngineError> {
        match self.color {
            Color::Excuse => Ok(()),
            Color::Trump => {
                if (1..=21).contains(&self.value) {
                    Ok(())
                } else {
                    Err(EngineError::InvalidCardValue(self.value))
                }
            }
            _ => {
                if (1..=14).contains(&self.value) {
                    Ok(())
                } else {
                    Err(EngineError::InvalidCardValue(self.value))
                }
            }
        }
    }

    pub fn points(self: &Self) -> usize {
        match self {
            Card {
                color: Color::Trump,
                value,
            } => {
                if *value == 1 || *value == 21 {
                    9
                } else {
                    1
                }
            }
            Card {
                color: Color::Excuse,
                value: _,
            } => 9,
            Card { color: _, value } => {
                if *value < 11 {
                    1
                } else {
                    ((*value as usize) - 10) * 2 + 1
                }
            }
        }
    }

    pub fn win_against(&self, other: &Self, lead_color: &Color) -> bool {
        match (self.color, other.color) {
            (color1, color2) if color1 == color2 => self.value > other.value,
            (Color::Trump, _) => true,
            (_, Color::Trump) => false,
            (color, _) if color == *lead_color => true,
            (_, _) => false,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Excuse => write!(f, "Excuse"),
            Color::Trump => write!(f, "{} of Trump", self.value),
            color => {
                let value_str = match self.value {
                    14 => "King",
                    13 => "Queen",
                    12 => "Knight",
                    11 => "Jack",
                    other => &other.to_string(),
                };
                write!(f, "{} of {}", value_str, color)
            }
        }
    }
}
