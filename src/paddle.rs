use macroquad::{
    prelude::WHITE,
    shapes::draw_rectangle,
    text::{draw_text_ex, Font, TextParams},
};
use serde::{Deserialize, Serialize};

use crate::{
    traits::{Drawable, Movable},
    FONT_SIZE, PADDLE_BOTTOM_LIMIT, PADDLE_HEIGHT, PADDLE_TOP_LIMIT, PADDLE_WIDTH, SCORE_MAX_VALUE,
};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Paddle {
    /// Paddle's score
    pub score: i32,
    /// Paddle's position
    pub pos: i32,
    /// Paddle's velocity
    pub vel: i32,
}

impl Paddle {
    /// Return a new paddle with score of 0, velocity of 0, and y position of 1.
    /// The X position is defined by whether it's the left paddle or the right paddle outside of the struct.
    /// # Examples
    /// ```
    /// # use good_game_rusty_pong::paddle::Paddle;
    /// let paddle = Paddle::new();
    /// assert_eq!(paddle.score, 0);
    /// assert_eq!(paddle.pos, 1);
    /// assert_eq!(paddle.vel, 0);
    /// ```
    pub fn new() -> Self {
        Paddle {
            score: 0,
            pos: 1,
            vel: 0,
        }
    }

    /// Score a point, if paddle's score is [`SCORE_MAX_VALUE`] or greater, then returns true, otherwise false
    /// # Examples
    /// ```
    /// # use good_game_rusty_pong::{SCORE_MAX_VALUE, paddle::Paddle};
    /// # let mut paddle = Paddle::new();
    /// assert_eq!(paddle.score, 0);
    /// let is_max_value = paddle.score_point();
    /// assert_eq!(paddle.score, 1);
    /// assert_ne!(paddle.score, SCORE_MAX_VALUE);
    /// assert_eq!(is_max_value, false);
    /// ```
    pub fn score_point(&mut self) -> bool {
        self.score += 1;
        self.score >= SCORE_MAX_VALUE
    }

    /// Draw the score on the screen using [`draw_text_ex()`] function
    pub fn draw_score(&self, x: f32, y: f32, font: Font) {
        draw_text_ex(
            &self.score.to_string(),
            x,
            y,
            TextParams {
                font_size: FONT_SIZE,
                font,
                ..Default::default()
            },
        )
    }
}

impl Movable for Paddle {
    /// Move the ball according to its [`Self::vel`]
    /// In case the position is below [`PADDLE_TOP_LIMIT`] and positive velocity
    /// Or in case the position is above [`PADDLE_BOTTOM_LIMIT`] and negative velocity
    fn perform_movement(&mut self) {
        if (self.vel > 0 && self.pos < PADDLE_TOP_LIMIT)
            || (self.vel < 0 && self.pos >= PADDLE_BOTTOM_LIMIT)
        {
            self.pos += self.vel;
        }
    }
}

impl Drawable for Paddle {
    /// Draw the paddle on the screen using the [`draw_rectangle()`] function
    fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x, y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
    }
}
