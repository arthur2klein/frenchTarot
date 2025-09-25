use std::collections::HashMap;

use crate::business::{
    analysis::analysis_error::AnalysisError, game_engine::game_state::GameState, Card,
};

pub struct MCTSNode {
    pub state: GameState,
    pub visits: usize,
    pub total_score: usize,
    pub children: HashMap<Card, Box<MCTSNode>>,
}

impl MCTSNode {
    pub fn select_child(&mut self, c_param: f64) -> (&Card, &mut MCTSNode) {
        let mut best: Option<(&Card, &mut MCTSNode)> = None;
        for (card, node) in self.children.iter_mut() {
            let uct = node.uct_value(self.visits, c_param);
            if let Some((_, best_node)) = &mut best {
                let best_uct = best_node.uct_value(self.visits, c_param);
                if uct > best_uct {
                    best = Some((card, node));
                }
            } else {
                best = Some((card, node));
            }
        }
        best.unwrap()
    }

    fn uct_value(&self, parent_visits: usize, c_param: f64) -> f64 {
        if self.visits == 0 {
            f64::INFINITY
        } else {
            (self.total_score as f64 / self.visits as f64)
                + c_param * ((parent_visits as f64).ln() / self.visits as f64).sqrt()
        }
    }

    pub fn expand(&mut self) -> Result<(Card, MCTSNode), AnalysisError> {
        let legal_moves = self.state.cards_allowed(
            self.state
                .shared_state
                .next_to_play()
                .unwrap_or((self.state.shared_state.dealer + 1) % 4),
        );
        for card in legal_moves {
            if !self.children.contains_key(&card) {
                let mut new_state = self.state.clone();
                new_state
                    .play_card(
                        self.state
                            .shared_state
                            .next_to_play()
                            .unwrap_or((self.state.shared_state.dealer + 1) % 4),
                        card,
                    )
                    .map_err(|_| AnalysisError::NoCardToPlay)?;
                return Ok((*card, MCTSNode::new(new_state)));
            }
        }
        Err(AnalysisError::AnalysisFinished)
    }

    pub fn backpropagate(&mut self, result: usize) {
        self.visits += 1;
        self.total_score += result;
    }

    pub fn new(state: GameState) -> Self {
        Self {
            state,
            visits: 0,
            total_score: 0,
            children: HashMap::new(),
        }
    }
}
