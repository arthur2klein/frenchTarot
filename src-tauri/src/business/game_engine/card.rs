use super::engine_error::EngineError;
use std::{collections::HashSet, fmt};

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

    pub fn all_possibles() -> HashSet<Card> {
        [
            Card {
                color: Color::Excuse,
                value: 0,
            },
            Card {
                color: Color::Diamond,
                value: 1,
            },
            Card {
                color: Color::Diamond,
                value: 2,
            },
            Card {
                color: Color::Diamond,
                value: 3,
            },
            Card {
                color: Color::Diamond,
                value: 4,
            },
            Card {
                color: Color::Diamond,
                value: 5,
            },
            Card {
                color: Color::Diamond,
                value: 6,
            },
            Card {
                color: Color::Diamond,
                value: 7,
            },
            Card {
                color: Color::Diamond,
                value: 8,
            },
            Card {
                color: Color::Diamond,
                value: 9,
            },
            Card {
                color: Color::Diamond,
                value: 10,
            },
            Card {
                color: Color::Diamond,
                value: 11,
            },
            Card {
                color: Color::Diamond,
                value: 12,
            },
            Card {
                color: Color::Diamond,
                value: 13,
            },
            Card {
                color: Color::Diamond,
                value: 14,
            },
            Card {
                color: Color::Spade,
                value: 1,
            },
            Card {
                color: Color::Spade,
                value: 2,
            },
            Card {
                color: Color::Spade,
                value: 3,
            },
            Card {
                color: Color::Spade,
                value: 4,
            },
            Card {
                color: Color::Spade,
                value: 5,
            },
            Card {
                color: Color::Spade,
                value: 6,
            },
            Card {
                color: Color::Spade,
                value: 7,
            },
            Card {
                color: Color::Spade,
                value: 8,
            },
            Card {
                color: Color::Spade,
                value: 9,
            },
            Card {
                color: Color::Spade,
                value: 10,
            },
            Card {
                color: Color::Spade,
                value: 11,
            },
            Card {
                color: Color::Spade,
                value: 12,
            },
            Card {
                color: Color::Spade,
                value: 13,
            },
            Card {
                color: Color::Spade,
                value: 14,
            },
            Card {
                color: Color::Club,
                value: 1,
            },
            Card {
                color: Color::Club,
                value: 2,
            },
            Card {
                color: Color::Club,
                value: 3,
            },
            Card {
                color: Color::Club,
                value: 4,
            },
            Card {
                color: Color::Club,
                value: 5,
            },
            Card {
                color: Color::Club,
                value: 6,
            },
            Card {
                color: Color::Club,
                value: 7,
            },
            Card {
                color: Color::Club,
                value: 8,
            },
            Card {
                color: Color::Club,
                value: 9,
            },
            Card {
                color: Color::Club,
                value: 10,
            },
            Card {
                color: Color::Club,
                value: 11,
            },
            Card {
                color: Color::Club,
                value: 12,
            },
            Card {
                color: Color::Club,
                value: 13,
            },
            Card {
                color: Color::Club,
                value: 14,
            },
            Card {
                color: Color::Heart,
                value: 1,
            },
            Card {
                color: Color::Heart,
                value: 2,
            },
            Card {
                color: Color::Heart,
                value: 3,
            },
            Card {
                color: Color::Heart,
                value: 4,
            },
            Card {
                color: Color::Heart,
                value: 5,
            },
            Card {
                color: Color::Heart,
                value: 6,
            },
            Card {
                color: Color::Heart,
                value: 7,
            },
            Card {
                color: Color::Heart,
                value: 8,
            },
            Card {
                color: Color::Heart,
                value: 9,
            },
            Card {
                color: Color::Heart,
                value: 10,
            },
            Card {
                color: Color::Heart,
                value: 11,
            },
            Card {
                color: Color::Heart,
                value: 12,
            },
            Card {
                color: Color::Heart,
                value: 13,
            },
            Card {
                color: Color::Heart,
                value: 14,
            },
            Card {
                color: Color::Trump,
                value: 1,
            },
            Card {
                color: Color::Trump,
                value: 2,
            },
            Card {
                color: Color::Trump,
                value: 3,
            },
            Card {
                color: Color::Trump,
                value: 4,
            },
            Card {
                color: Color::Trump,
                value: 5,
            },
            Card {
                color: Color::Trump,
                value: 6,
            },
            Card {
                color: Color::Trump,
                value: 7,
            },
            Card {
                color: Color::Trump,
                value: 8,
            },
            Card {
                color: Color::Trump,
                value: 9,
            },
            Card {
                color: Color::Trump,
                value: 10,
            },
            Card {
                color: Color::Trump,
                value: 11,
            },
            Card {
                color: Color::Trump,
                value: 12,
            },
            Card {
                color: Color::Trump,
                value: 13,
            },
            Card {
                color: Color::Trump,
                value: 14,
            },
            Card {
                color: Color::Trump,
                value: 15,
            },
            Card {
                color: Color::Trump,
                value: 16,
            },
            Card {
                color: Color::Trump,
                value: 17,
            },
            Card {
                color: Color::Trump,
                value: 18,
            },
            Card {
                color: Color::Trump,
                value: 19,
            },
            Card {
                color: Color::Trump,
                value: 20,
            },
            Card {
                color: Color::Trump,
                value: 21,
            },
        ]
        .into_iter()
        .collect()
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
