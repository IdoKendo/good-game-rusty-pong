pub mod ball;
pub mod game;
pub mod game_state;
pub mod ggrs_config;
pub mod helpers;
pub mod input;
pub mod lobby;
pub mod paddle;
pub mod screen_state;
pub mod traits;

pub const BALL_SIZE: f32 = 10.0;
pub const EDGE_BOTTOM: i32 = 1;
pub const EDGE_LEFT: i32 = 1;
pub const EDGE_RIGHT: i32 = 502;
pub const EDGE_TOP: i32 = 332;
pub const FONT_SIZE: u16 = 32;
pub const INITIAL_VELOCITY: i32 = 3;
pub const INPUT_LEFT_PADDLE_DOWN: u8 = 0b0010;
pub const INPUT_LEFT_PADDLE_UP: u8 = 0b0001;
pub const INPUT_RIGHT_PADDLE_DOWN: u8 = 0b0100;
pub const INPUT_RIGHT_PADDLE_UP: u8 = 0b1000;
pub const MATCHBOX_ADDR: &str = "wss://match.gschup.dev";
pub const MIDDLE_POS: [i32; 2] = [256, 171];
pub const PADDLE_BOTTOM_LIMIT: i32 = 1;
pub const PADDLE_HEIGHT: f32 = 50.0;
pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_TOP_LIMIT: i32 = 291;
pub const SCORE_MAX_VALUE: i32 = 5;
pub const SCORE_POS_X: f32 = 60.0;
pub const SCORE_POS_Y: f32 = 35.0;
pub const SCREEN_WIDTH: f32 = 512.0;
pub const SCREEN_HEIGHT: f32 = 342.0;
