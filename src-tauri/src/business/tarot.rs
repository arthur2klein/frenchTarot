use super::{
    business_error::BusinessError,
    game_engine::{
        engine_error::EngineError, game_state::GameState, game_type::GameType,
        known_game_state::KnownGameState, trick::Trick,
    },
    player::Player,
};

pub struct Tarot {
    state: GameState,
    players: [Box<dyn Player>; 4],
}

impl Tarot {
    pub fn initialize(players: [Box<dyn Player>; 4]) -> Self {
        Self {
            state: GameState::random_init(),
            players,
        }
    }

    pub fn bid(&mut self) -> Result<(), BusinessError> {
        let mut current_player = (self.state.shared_state.dealer + 1) % 4;
        let mut current_pass_count = 0;
        while current_pass_count < 4 {
            let player_bid = self.players[current_player as usize]
                .bid(KnownGameState::from_omniscient(&self.state, current_player))?;
            if let Some(game_type) = player_bid {
                self.state
                    .shared_state
                    .bid(current_player, Some(game_type))?;
                current_pass_count = 0;
            } else {
                current_pass_count += 1;
            }
            current_player = (current_player + 1) % 4;
        }
        let game_type = self
            .state
            .shared_state
            .game_type
            .ok_or(BusinessError::EveryonePassed)?;
        let taker = self
            .state
            .shared_state
            .taker
            .ok_or(BusinessError::EveryonePassed)?;
        if game_type.kitty_should_be_revealed() {
            let aside = self.players[taker as usize]
                .chose_aside(KnownGameState::from_omniscient(&self.state, current_player))?;
            self.state.players_state[taker as usize].chose_aside(aside)?;
        }
        Ok(())
    }

    pub fn play_a_new_trick(&mut self) -> Result<(), BusinessError> {
        let mut new_trick = Trick::new(self.state.shared_state.player_to_lead());
        self.state.shared_state.current_trick = Some(new_trick);
        for _ in 0..4 {
            if let Some(player_index) = new_trick.next_to_play() {
                let card = self.players[player_index as usize]
                    .play_a_card(KnownGameState::from_omniscient(&self.state, player_index))?;
                self.state.players_state[player_index as usize].play_a_card(
                    &mut new_trick,
                    player_index,
                    &card,
                )?;
            } else {
                return Err(BusinessError::Engine(EngineError::FinishedHand));
            }
        }
        Ok(self.state.shared_state.finish_trick()?)
    }

    pub fn play(&mut self) -> Result<usize, BusinessError> {
        self.bid()?;
        for _ in 0..18 {
            self.play_a_new_trick()?;
        }
        Ok(self.state.shared_state.current_score())
    }
}
