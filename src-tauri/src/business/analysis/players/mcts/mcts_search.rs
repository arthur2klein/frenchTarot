use crate::business::analysis::analysis_error::AnalysisError;
use crate::business::analysis::simulate_random_playout::simulate_random_playout;
use crate::business::game_engine::game_state::GameState;
use crate::business::Card;

use super::mcts_node::MCTSNode;

pub fn mcts_search(
    root_state: GameState,
    iterations: usize,
    c_param: f64,
) -> Result<Card, AnalysisError> {
    let mut root = MCTSNode::new(root_state);
    for _ in 0..iterations {
        let mut node = &mut root;
        // Selection
        while !node.children.is_empty() && !node.state.shared_state.finished() {
            let (_, child) = node.select_child(c_param);
            node = child;
        }
        // Expansion
        if !node.state.shared_state.finished() {
            let (card, mut new_node) = node.expand()?;
            let mut sim_state = new_node.state.clone();
            let result = simulate_random_playout(&mut sim_state)?;
            new_node.backpropagate(result);
            node.children.insert(card, Box::new(new_node));
        } else {
            // simulation on the leaf node
            let mut sim_state = node.state.clone();
            let result = simulate_random_playout(&mut sim_state)?;
            node.backpropagate(result);
        }
    }
    root.children
        .iter()
        .max_by_key(|(_, node)| (node.total_score as f64 / node.visits as f64 * 1000.0) as i64)
        .map(|(card, _)| *card)
        .ok_or(AnalysisError::NoCardToPlay)
}
