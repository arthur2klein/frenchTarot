use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use std::collections::HashSet;

use crate::business::analysis::analysis_error::AnalysisError;
use crate::business::{Card, KnownGameState, Player};

struct Random {
    rng: ThreadRng,
}

impl Player for Random {
    fn play_a_card(&mut self, known: &KnownGameState) -> Result<Card, AnalysisError> {
        let allowed: HashSet<&Card> = known.player_state.cards_allowed(
            &known
                .shared_state
                .current_trick
                .ok_or(AnalysisError::NoCardToPlay)?,
        );
        let chosen = allowed.iter().choose(&mut self.rng);
        chosen.map(|card| **card).ok_or(AnalysisError::NoCardToPlay)
    }
}
