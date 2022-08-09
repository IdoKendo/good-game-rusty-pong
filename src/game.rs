use async_executor::LocalExecutor;
use ggrs::InputStatus;
use macroquad::prelude::*;
use macroquad::{
    text::{load_ttf_font, Font},
    texture::Texture2D,
    window::{clear_background, next_frame, request_new_screen_size, screen_width},
};
use matchbox_socket::WebRtcSocket;
use std::process;

use crate::{
    game_state::GameState, lobby::Lobby, screen_state::ScreenState, traits::Drawable, EDGE_LEFT,
    EDGE_RIGHT, FONT_PATH, MATCHBOX_ADDR, SCORE_POS_X, SCORE_POS_Y, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub struct Game<'a> {
    screen_state: ScreenState,
    game_state: GameState,
    executor: LocalExecutor<'a>,
    lobby: Lobby,
    socket: Option<WebRtcSocket>,
}

impl<'a> Game<'a> {
    pub fn new(logo: Texture2D) -> Self {
        Self {
            screen_state: ScreenState::Lobby,
            executor: LocalExecutor::new(),
            game_state: GameState::new(),
            lobby: Lobby::new(logo),
            socket: None,
        }
    }

    pub async fn run(&mut self) {
        let font = load_ttf_font(FONT_PATH).await.unwrap();

        loop {
            clear_background(BLACK);

            match &mut self.screen_state {
                ScreenState::Lobby => self.run_lobby(font),
                ScreenState::Connecting => self.run_connecting(),
                ScreenState::Game => self.run_game(font),
            }

            next_frame().await;
        }
    }

    fn run_lobby(&mut self, font: Font) {
        if let Some(room_id) = self.lobby.run(font) {
            info!("Constructing socket...");
            let room_url = format!("{MATCHBOX_ADDR}/{room_id}");
            let (socket, message_loop) = WebRtcSocket::new(room_url);
            self.socket = Some(socket);
            let task = self.executor.spawn(message_loop);
            task.detach();
            self.screen_state = ScreenState::Game; // TODO: Change to connecting after implementing netcode
        }
    }

    fn run_connecting(&mut self) {}

    fn run_game(&mut self, font: Font) {
        request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
        clear_background(BLACK);

        let inputs = self.game_state.local_input();
        self.game_state.advance((inputs, InputStatus::Confirmed));

        if self.game_state.ball.pos_x > EDGE_RIGHT
            && self
                .game_state
                .ball
                .missed_paddle(self.game_state.right_paddle.pos)
        {
            if self.game_state.left_paddle.score_point() {
                process::exit(0);
            }
            self.game_state.ball.reset_position();
        }

        if self.game_state.ball.pos_x < EDGE_LEFT
            && self
                .game_state
                .ball
                .missed_paddle(self.game_state.left_paddle.pos)
        {
            if self.game_state.right_paddle.score_point() {
                process::exit(0);
            }
            self.game_state.ball.reset_position();
        }

        self.game_state
            .left_paddle
            .draw(0.0, self.game_state.left_paddle.pos as f32);
        self.game_state
            .right_paddle
            .draw(SCREEN_WIDTH - 10.0, self.game_state.right_paddle.pos as f32);
        self.game_state.ball.draw(
            self.game_state.ball.pos_x as f32,
            self.game_state.ball.pos_y as f32,
        );
        self.game_state
            .left_paddle
            .draw_score(SCORE_POS_X, SCORE_POS_Y, font);
        self.game_state
            .right_paddle
            .draw_score(screen_width() - SCORE_POS_X, SCORE_POS_Y, font);
    }
}
