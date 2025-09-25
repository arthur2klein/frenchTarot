use crate::business::{
    analysis::{analysis_error::AnalysisError, players::mcts::mcts_search::mcts_search},
    Card, KnownGameState, Player,
};

pub struct MCTS {
    pub n_iterations: usize,
    pub c_param: f64, //1.41
}

impl Player for MCTS {
    fn play_a_card(&mut self, game_state: &KnownGameState) -> Result<Card, AnalysisError> {
        mcts_search(
            game_state.possible_random_full_state().map_err(|_| {
                AnalysisError::Other(String::from("Could not generate random state"))
            })?,
            self.n_iterations,
            self.c_param,
        )
    }
}
