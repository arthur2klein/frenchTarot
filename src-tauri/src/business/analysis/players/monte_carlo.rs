use rayon::prelude::*;
use std::collections::HashSet;

use crate::business::analysis::analysis_error::AnalysisError;
use crate::business::analysis::simulate_random_playout::simulate_random_playout;
use crate::business::{Card, KnownGameState, Player};

struct MonteCarlo {
    sims_per_candidate: usize,
}

impl Player for MonteCarlo {
    fn play_a_card(&mut self, known: &KnownGameState) -> Result<Card, AnalysisError> {
        let allowed: HashSet<&Card> = known.player_state.cards_allowed(
            &known
                .shared_state
                .current_trick
                .ok_or(AnalysisError::NoCardToPlay)?,
        );
        let results: Vec<(Card, usize)> = allowed
            .into_iter()
            .filter_map(|card| {
                let candidate = *card;
                let total_score: Result<usize, AnalysisError> = (0..self.sims_per_candidate)
                    .into_par_iter()
                    .try_fold(
                        || 0,
                        |acc, _| {
                            let mut full = known.possible_random_full_state().map_err(|_| {
                                AnalysisError::Other(String::from(
                                    "Can not generate a random full state",
                                ))
                            })?;
                            full.play_card(known.player_index, &candidate)
                                .map_err(|_| {
                                    AnalysisError::Other(String::from(
                                        "Can not generate play a card",
                                    ))
                                })?;
                            let val: usize = simulate_random_playout(&mut full)?;
                            Ok(acc + val)
                        },
                    )
                    .try_reduce(|| 0, |a, b| Ok(a + b));

                total_score
                    .map(|value| (candidate, value / self.sims_per_candidate))
                    .ok()
            })
            .collect();

        // pick best
        let best = results
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or(AnalysisError::NoCardToPlay)?;

        Ok(best.0)
    }
}
