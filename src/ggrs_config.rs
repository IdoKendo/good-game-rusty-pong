use ggrs::Config;

use crate::{game_state::GameState, input::Input};

/// `GGRSConfig` holds all type parameters for GGRS Sessions
#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Input;
    type State = GameState;
    type Address = String;
}
