use macroquad::{prelude::WHITE, shapes::draw_rectangle};
use serde::{Deserialize, Serialize};

use crate::{
    traits::{Drawable, Movable},
    BALL_SIZE, EDGE_BOTTOM, EDGE_LEFT, EDGE_RIGHT, EDGE_TOP, INITIAL_VELOCITY, MIDDLE_POS,
    PADDLE_HEIGHT,
};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Ball {
    /// Ball's X position
    pub pos_x: i32,
    /// Ball's Y position
    pub pos_y: i32,
    /// Ball's X velocity
    pub vel_x: i32,
    /// Ball's Y velocity
    pub vel_y: i32,
}

impl Ball {
    /// Return a new ball at the (0, 0) position, with velocity of [`INITIAL_VELOCITY`] in both x direction and y direction.
    /// # Examples
    /// ```
    /// # use good_game_rusty_pong::{INITIAL_VELOCITY, ball::Ball};
    /// let ball = Ball::new();
    /// assert_eq!(ball.pos_x, 0);
    /// assert_eq!(ball.pos_y, 0);
    /// assert_eq!(ball.vel_x, INITIAL_VELOCITY);
    /// assert_eq!(ball.vel_y, INITIAL_VELOCITY);
    /// ```
    pub fn new() -> Self {
        Ball {
            pos_x: 0,
            pos_y: 0,
            vel_x: INITIAL_VELOCITY,
            vel_y: INITIAL_VELOCITY,
        }
    }

    /// Return true if the ball's Y position is outside a paddle's Y positions.
    /// This takes the [`PADDLE_HEIGHT`] into consideration
    pub fn missed_paddle(&self, paddle_pos: i32) -> bool {
        self.pos_y < paddle_pos || self.pos_y > paddle_pos + (PADDLE_HEIGHT as i32)
    }

    /// Reset the ball's position to the [`MIDDLE_POS`]
    /// # Examples
    /// ```
    /// # use good_game_rusty_pong::{MIDDLE_POS, ball::Ball};
    /// # let mut ball = Ball::new();
    /// ball.reset_position();
    /// assert_eq!(ball.pos_x, MIDDLE_POS[0]);
    /// assert_eq!(ball.pos_y, MIDDLE_POS[1]);
    /// ```
    pub fn reset_position(&mut self) {
        self.pos_x = MIDDLE_POS[0];
        self.pos_y = MIDDLE_POS[1];
    }
}

impl Movable for Ball {
    /// Move the ball according to its [`Self::vel_x`] and [`Self::vel_y`]
    /// If the [`Self::pos_x`] exceeds [`EDGE_RIGHT`] or [`EDGE_LEFT`] the [`Self::vel_x`] will flip
    /// If the [`Self::pos_y`] exceeds [`EDGE_TOP`] or [`EDGE_BOTTOM`] the [`Self::vel_y`] will flip
    fn perform_movement(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        if self.pos_x > EDGE_RIGHT || self.pos_x < EDGE_LEFT {
            self.vel_x = -self.vel_x;
        }

        if self.pos_y > EDGE_TOP || self.pos_y < EDGE_BOTTOM {
            self.vel_y = -self.vel_y;
        }
    }
}

impl Drawable for Ball {
    /// Draw a rectangle using [`draw_rectangle()`] to represent the ball
    fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x, y, BALL_SIZE, BALL_SIZE, WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_ball() {
        let mut ball = Ball::new();
        let initial_x = ball.pos_x;
        let initial_y = ball.pos_y;
        let initial_vel_x = ball.vel_x;
        let initial_vel_y = ball.vel_y;
        ball.perform_movement();
        assert_eq!(ball.pos_x, initial_x + ball.vel_x);
        assert_eq!(ball.pos_y, initial_y + ball.vel_y);
        assert_eq!(ball.vel_x, initial_vel_x);
        assert_eq!(ball.vel_y, initial_vel_y);
    }

    #[test]
    fn change_vel_on_right_edge() {
        let mut ball = Ball::new();
        ball.pos_x = EDGE_RIGHT;
        let initial_x = ball.pos_x;
        let initial_y = ball.pos_y;
        let initial_vel_x = ball.vel_x;
        let initial_vel_y = ball.vel_y;
        ball.perform_movement();
        assert_eq!(ball.pos_x, initial_x - ball.vel_x);
        assert_eq!(ball.pos_y, initial_y + ball.vel_y);
        assert_eq!(ball.vel_x, -initial_vel_x);
        assert_eq!(ball.vel_y, initial_vel_y);
    }

    #[test]
    fn change_vel_on_left_edge() {
        let mut ball = Ball::new();
        ball.pos_x = EDGE_LEFT;
        ball.vel_x = -ball.vel_x;
        let initial_x = ball.pos_x;
        let initial_y = ball.pos_y;
        let initial_vel_x = ball.vel_x;
        let initial_vel_y = ball.vel_y;
        ball.perform_movement();
        assert_eq!(ball.pos_x, initial_x - ball.vel_x);
        assert_eq!(ball.pos_y, initial_y + ball.vel_y);
        assert_eq!(ball.vel_x, -initial_vel_x);
        assert_eq!(ball.vel_y, initial_vel_y);
    }

    #[test]
    fn change_vel_on_top_edge() {
        let mut ball = Ball::new();
        ball.pos_y = EDGE_TOP;
        let initial_x = ball.pos_x;
        let initial_y = ball.pos_y;
        let initial_vel_x = ball.vel_x;
        let initial_vel_y = ball.vel_y;
        ball.perform_movement();
        assert_eq!(ball.pos_x, initial_x + ball.vel_x);
        assert_eq!(ball.pos_y, initial_y - ball.vel_y);
        assert_eq!(ball.vel_x, initial_vel_x);
        assert_eq!(ball.vel_y, -initial_vel_y);
    }

    #[test]
    fn change_vel_on_bottom_edge() {
        let mut ball = Ball::new();
        ball.pos_x = EDGE_BOTTOM;
        ball.vel_y = -ball.vel_y;
        let initial_x = ball.pos_x;
        let initial_y = ball.pos_y;
        let initial_vel_x = ball.vel_x;
        let initial_vel_y = ball.vel_y;
        ball.perform_movement();
        assert_eq!(ball.pos_x, initial_x + ball.vel_x);
        assert_eq!(ball.pos_y, initial_y - ball.vel_y);
        assert_eq!(ball.vel_x, initial_vel_x);
        assert_eq!(ball.vel_y, -initial_vel_y);
    }

    #[test]
    fn above_paddle() {
        let ball = Ball::new();
        assert_eq!(ball.missed_paddle(ball.pos_y + 1), true);
    }

    #[test]
    fn below_paddle() {
        let mut ball = Ball::new();
        ball.pos_y = PADDLE_HEIGHT as i32;
        assert_eq!(ball.missed_paddle(-1), true);
    }

    #[test]
    fn hit_paddle() {
        let ball = Ball::new();
        assert_eq!(ball.missed_paddle(-1), false);
    }
}
