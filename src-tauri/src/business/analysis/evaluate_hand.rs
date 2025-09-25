use crate::business::{Color, GameType, PlayerGameState};

pub fn evaluate_hand(state: &PlayerGameState) -> Option<GameType> {
    if count_points(state, GameType::Petit { chelem: false })
        < points_threshold(GameType::Petit { chelem: false })
    {
        None
    } else if count_points(state, GameType::Garde { chelem: false })
        < points_threshold(GameType::Garde { chelem: false })
    {
        Some(GameType::Petit { chelem: false })
    } else if count_points(state, GameType::GardeSans { chelem: false })
        < points_threshold(GameType::GardeSans { chelem: false })
    {
        Some(GameType::Garde { chelem: false })
    } else if count_points(state, GameType::GardeContre { chelem: false })
        < points_threshold(GameType::GardeContre { chelem: false })
    {
        Some(GameType::GardeSans { chelem: false })
    } else {
        Some(GameType::GardeContre { chelem: false })
    }
}

fn color_to_index(color: &Color) -> usize {
    match color {
        Color::Spade => 0,
        Color::Club => 1,
        Color::Heart => 2,
        Color::Diamond => 3,
        Color::Trump => 4,
        Color::Excuse => 5,
    }
}

fn count_points(state: &PlayerGameState, game_type: GameType) -> usize {
    let count_low_suits = matches!(
        game_type,
        GameType::GardeSans { chelem: _ } | GameType::GardeContre { chelem: _ }
    );
    let mut total = 0;
    let mut has_petit = false;
    let mut number_per_color = [0; 6];
    let mut has_honour = [[false; 4]; 4];
    let mut high_trumps = [false; 6];
    for card in &state.hand {
        number_per_color[color_to_index(&card.color)] += 1;
        if card.color == Color::Trump {
            // 10 points for the 21
            if card.value == 21 {
                total += 10;
            }
            if card.value == 1 {
                has_petit = true;
            }
            // 2 points per high trump
            if card.value > 15 {
                total += 2;
                high_trumps[(card.value - 16) as usize] = true;
            }
        } else if card.color == Color::Excuse {
            // 8 points for the excuse
            total += 8;
        } else {
            if card.value > 10 {
                has_honour[color_to_index(&card.color)][(card.value - 11) as usize] = true;
            }
        }
    }
    // 1 point per high trump in a sequence
    let mut current_suit = 0;
    for has in high_trumps {
        if has {
            current_suit += 1;
        } else {
            if current_suit > 1 {
                total += current_suit;
            }
            current_suit = 0;
        }
    }
    if current_suit > 1 {
        total += current_suit;
    }
    let number_of_trumps = number_per_color[color_to_index(&Color::Trump)];
    // 0-9 points for the petit depending of the number of trumps to protect it
    if has_petit {
        total += match number_of_trumps {
            1..4 => 0,
            4 => 5,
            5 => 7,
            _ => 9,
        }
    }
    // 2 points per trump
    if number_of_trumps > 3 {
        total += number_of_trumps * 2;
    }
    for i in 0..4 {
        // 0-9 points for high suits
        // if garde sans or garde contre, 3-6 points if few cards in a suit
        total += match number_per_color[i] {
            0 => {
                if count_low_suits {
                    6
                } else {
                    0
                }
            }
            1 => {
                if count_low_suits {
                    3
                } else {
                    0
                }
            }
            2..5 => 0,
            5 => 5,
            6 => 7,
            _ => 9,
        };
        // 0-13 points for honours
        total += match (has_honour[i][3], has_honour[i][2]) {
            (true, true) => 10,
            (true, false) => 6,
            (false, true) => 3,
            (false, false) => 0,
        };
        if has_honour[i][1] {
            total += 2;
        }
        if has_honour[i][0] {
            total += 1;
        }
    }
    total
}

fn points_threshold(game_type: GameType) -> usize {
    match game_type {
        GameType::Petit { chelem: _ } => 40,
        GameType::Garde { chelem: _ } => 55,
        GameType::GardeSans { chelem: _ } => 71,
        GameType::GardeContre { chelem: _ } => 80,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::business::Card;

    use super::*;

    #[test]
    fn test_point_threshold() {
        assert_eq!(points_threshold(GameType::Petit { chelem: false }), 40)
    }

    #[test]
    fn test_thresholds() {
        let mut hand: HashSet<Card> = HashSet::new();
        hand.insert(Card::new(Color::Diamond, 10).unwrap());
        hand.insert(Card::new(Color::Spade, 7).unwrap());
        hand.insert(Card::new(Color::Spade, 14).unwrap());
        hand.insert(Card::new(Color::Trump, 1).unwrap());
        hand.insert(Card::new(Color::Trump, 21).unwrap());
        hand.insert(Card::new(Color::Club, 11).unwrap());
        hand.insert(Card::new(Color::Trump, 20).unwrap());
        hand.insert(Card::new(Color::Trump, 14).unwrap());
        hand.insert(Card::new(Color::Trump, 15).unwrap());
        hand.insert(Card::new(Color::Diamond, 13).unwrap());
        hand.insert(Card::new(Color::Heart, 12).unwrap());
        hand.insert(Card::new(Color::Trump, 18).unwrap());
        hand.insert(Card::new(Color::Heart, 14).unwrap());
        hand.insert(Card::new(Color::Club, 14).unwrap());
        hand.insert(Card::new(Color::Diamond, 8).unwrap());
        hand.insert(Card::new(Color::Diamond, 12).unwrap());
        hand.insert(Card::new(Color::Trump, 19).unwrap());
        hand.insert(Card::new(Color::Excuse, 0).unwrap());
        assert_eq!(
            count_points(&PlayerGameState { hand }, GameType::Petit { chelem: false }),
            79
        )
    }
}
