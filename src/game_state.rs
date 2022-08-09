use ggrs::InputStatus;
use macroquad::prelude::*;

use crate::{
    ball::Ball, input::Input, paddle::Paddle, traits::Movable, INPUT_LEFT_PADDLE_DOWN,
    INPUT_LEFT_PADDLE_UP, INPUT_RIGHT_PADDLE_DOWN, INPUT_RIGHT_PADDLE_UP,
};

#[derive(Clone, Default)]
pub struct GameState {
    frame: i32,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub ball: Ball,
}

impl GameState {
    pub fn new() -> Self {
        let left_paddle = Paddle::new();
        let right_paddle = Paddle::new();
        let ball = Ball::new();

        Self {
            frame: 0,
            left_paddle,
            right_paddle,
            ball,
        }
    }

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

    pub fn advance(&mut self, inputs: (Input, InputStatus)) {
        self.frame += 1;

        let movables: Vec<&mut dyn Movable> = vec![
            &mut self.left_paddle,
            &mut self.right_paddle,
            &mut self.ball,
        ];
        for movable in movables {
            movable.perform_movement();
        }

        let input = match inputs.1 {
            InputStatus::Confirmed => inputs.0.inp,
            InputStatus::Predicted => inputs.0.inp,
            InputStatus::Disconnected => 0, // If disconnected, stop all movement
        };

        if input & INPUT_LEFT_PADDLE_UP != 0 && input & INPUT_LEFT_PADDLE_DOWN == 0 {
            self.left_paddle.vel = -1;
        }
        if input & INPUT_LEFT_PADDLE_UP == 0 && input & INPUT_LEFT_PADDLE_DOWN != 0 {
            self.left_paddle.vel = 1;
        }
        if input & INPUT_LEFT_PADDLE_UP == 0 && input & INPUT_LEFT_PADDLE_DOWN == 0 {
            self.left_paddle.vel = 0;
        }
        if input & INPUT_RIGHT_PADDLE_UP != 0 && input & INPUT_RIGHT_PADDLE_DOWN == 0 {
            self.right_paddle.vel = -1;
        }
        if input & INPUT_RIGHT_PADDLE_UP == 0 && input & INPUT_RIGHT_PADDLE_DOWN != 0 {
            self.right_paddle.vel = 1;
        }
        if input & INPUT_RIGHT_PADDLE_UP == 0 && input & INPUT_RIGHT_PADDLE_DOWN == 0 {
            self.right_paddle.vel = 0;
        }
    }
}
