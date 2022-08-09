use ggrs::{Frame, GGRSRequest, GameStateCell, InputStatus, NULL_FRAME};
use macroquad::{
    audio::{play_sound_once, Sound},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    ball::Ball, ggrs_config::GGRSConfig, helpers::fletcher16, input::Input, paddle::Paddle,
    traits::Movable, INPUT_LEFT_PADDLE_DOWN, INPUT_LEFT_PADDLE_UP, INPUT_RIGHT_PADDLE_DOWN,
    INPUT_RIGHT_PADDLE_UP,
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GameState {
    frame: i32,
    last_checksum: (Frame, u64),
    periodic_checksum: (Frame, u64),
    sound_played: usize,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub ball: Ball,
}

impl GameState {
    /// Return a new game state with an initial [`Paddle`] for both left and right paddles,
    /// an initial [`Ball`] for the ball, and frame 0
    pub fn new() -> Self {
        let left_paddle = Paddle::new();
        let right_paddle = Paddle::new();
        let ball = Ball::new();

        Self {
            frame: 0,
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            sound_played: 0,
            left_paddle,
            right_paddle,
            ball,
        }
    }

    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest<GGRSConfig>>, sounds: &[Sound]) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell, .. } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance(inputs, sounds.to_vec()),
            }
        }
    }

    /// Save the current game state and create a checksum
    fn save_game_state(&mut self, cell: GameStateCell<GameState>, frame: Frame) {
        assert_eq!(self.frame, frame);
        let buffer = bincode::serialize(&self).unwrap();
        let checksum = fletcher16(&buffer) as u128;
        cell.save(frame, Some(self.clone()), Some(checksum));
    }

    /// Load gamestate and overwrite self
    fn load_game_state(&mut self, cell: GameStateCell<GameState>) {
        let loaded_game_state = cell.load().expect("No data found.");
        self.clone_from(&loaded_game_state);
    }

    /// Check for local inputs from the player and return an [`Input`] object
    pub fn local_input(&self) -> Input {
        let mut inp: u8 = 0;

        if is_key_down(KeyCode::Up) {
            inp |= INPUT_RIGHT_PADDLE_UP;
        }
        if is_key_down(KeyCode::Down) {
            inp |= INPUT_RIGHT_PADDLE_DOWN;
        }
        if is_key_down(KeyCode::W) {
            inp |= INPUT_LEFT_PADDLE_UP;
        }
        if is_key_down(KeyCode::S) {
            inp |= INPUT_LEFT_PADDLE_DOWN;
        }

        Input { inp }
    }

    /// Advance the game state's by a single frame and handle the moveable objects according to the received [`Input`]  
    /// In case the inputs contain an [`InputStatus::Disconnected`] status, all inputs will be ignored
    pub fn advance(&mut self, inputs_vector: Vec<(Input, InputStatus)>, sounds: Vec<Sound>) {
        self.frame += 1;

        let movables: Vec<&mut dyn Movable> = vec![
            &mut self.left_paddle,
            &mut self.right_paddle,
            &mut self.ball,
        ];
        for movable in movables {
            movable.perform_movement();
        }

        if self.ball.changed_direction {
            play_sound_once(sounds[self.sound_played]);
            self.sound_played = if self.sound_played == 0 { 1 } else { 0 };
        }

        let mut detected_left = false;
        let mut detected_right = false;

        for inputs in inputs_vector {
            let input = match inputs.1 {
                InputStatus::Confirmed => inputs.0.inp,
                InputStatus::Predicted => inputs.0.inp,
                InputStatus::Disconnected => 0,
            };

            if input & INPUT_LEFT_PADDLE_UP != 0 && input & INPUT_LEFT_PADDLE_DOWN == 0 {
                self.left_paddle.vel = -2;
                detected_left = true;
            }
            if input & INPUT_LEFT_PADDLE_UP == 0 && input & INPUT_LEFT_PADDLE_DOWN != 0 {
                self.left_paddle.vel = 2;
                detected_left = true;
            }
            if input & INPUT_RIGHT_PADDLE_UP != 0 && input & INPUT_RIGHT_PADDLE_DOWN == 0 {
                self.right_paddle.vel = -2;
                detected_right = true;
            }
            if input & INPUT_RIGHT_PADDLE_UP == 0 && input & INPUT_RIGHT_PADDLE_DOWN != 0 {
                self.right_paddle.vel = 2;
                detected_right = true;
            }
        }

        if !detected_left {
            self.left_paddle.vel = 0;
        }

        if !detected_right {
            self.right_paddle.vel = 0;
        }
    }
}
