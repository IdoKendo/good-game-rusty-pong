use macroquad::prelude::{is_key_pressed, is_key_released, KeyCode};
use serde::{Deserialize, Serialize};

use crate::{ball::Ball, paddle::Paddle, traits::Movable};

#[derive(Clone, Serialize, Deserialize, Default)]
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

    pub fn advance(&mut self) {
        self.frame += 1;

        let movables: Vec<&mut dyn Movable> = vec![
            &mut self.left_paddle,
            &mut self.right_paddle,
            &mut self.ball,
        ];
        for movable in movables {
            movable.perform_movement();
        }

        if is_key_released(KeyCode::Up) || is_key_released(KeyCode::Down) {
            self.right_paddle.vel = 0;
        } else if is_key_released(KeyCode::W) || is_key_released(KeyCode::S) {
            self.left_paddle.vel = 0;
        }

        if is_key_pressed(KeyCode::Up) {
            self.right_paddle.vel = -1;
        } else if is_key_pressed(KeyCode::Down) {
            self.right_paddle.vel = 1;
        } else if is_key_pressed(KeyCode::W) {
            self.left_paddle.vel = -1;
        } else if is_key_pressed(KeyCode::S) {
            self.left_paddle.vel = 1;
        }
    }
}
