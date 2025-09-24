use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub enum GameType {
    Petit { chelem: bool },
    Garde { chelem: bool },
    GardeSans { chelem: bool },
    GardeContre { chelem: bool },
}

impl GameType {
    pub fn kitty_should_be_revealed(&self) -> bool {
        match self {
            GameType::Petit { chelem: _ } | GameType::Garde { chelem: _ } => false,
            GameType::GardeSans { chelem: _ } | GameType::GardeContre { chelem: _ } => true,
        }
    }

    pub fn hand_points_multiplier(&self) -> u8 {
        match self {
            GameType::Petit { chelem: _ } => 1,
            GameType::Garde { chelem: _ } => 2,
            GameType::GardeSans { chelem: _ } => 4,
            GameType::GardeContre { chelem: _ } => 6,
        }
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameType::Petit { chelem: _ } => write!(f, "Petit"),
            GameType::Garde { chelem: _ } => write!(f, "Garde"),
            GameType::GardeSans { chelem: _ } => write!(f, "Garde sans"),
            GameType::GardeContre { chelem: _ } => write!(f, "Garde contre"),
        }
    }
}
