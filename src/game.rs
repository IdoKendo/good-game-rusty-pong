use async_executor::LocalExecutor;
use ggrs::{GGRSError, P2PSession, SessionBuilder, SessionState};
use instant::{Duration, Instant};
use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;
use macroquad::{
    text::{load_ttf_font_from_bytes, Font},
    texture::Texture2D,
    window::{clear_background, next_frame, request_new_screen_size, screen_width},
};
use matchbox_socket::WebRtcSocket;

use crate::ggrs_config::GGRSConfig;
use crate::{
    game_state::GameState, lobby::Lobby, screen_state::ScreenState, traits::Drawable, EDGE_LEFT,
    EDGE_RIGHT, MATCHBOX_ADDR, SCORE_POS_X, SCORE_POS_Y, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub struct Game<'a> {
    screen_state: ScreenState,
    game_state: GameState,
    executor: LocalExecutor<'a>,
    lobby: Lobby,
    socket: Option<WebRtcSocket>,
    session: Option<P2PSession<GGRSConfig>>,
    last_update: Instant,
    accumulator: Duration,
    sounds: Vec<Sound>,
}

impl<'a> Game<'a> {
    pub fn new(logo: Texture2D) -> Self {
        Self {
            screen_state: ScreenState::Lobby,
            executor: LocalExecutor::new(),
            game_state: GameState::new(),
            lobby: Lobby::new(logo),
            socket: None,
            session: None,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            sounds: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        let font =
            load_ttf_font_from_bytes(include_bytes!("../assets/FiraSans-Regular.ttf")).unwrap();
        self.sounds = vec![
            load_sound_from_bytes(include_bytes!("../assets/left.wav"))
                .await
                .unwrap(),
            load_sound_from_bytes(include_bytes!("../assets/right.wav"))
                .await
                .unwrap(),
        ];

        loop {
            clear_background(BLACK);

            match &mut self.screen_state {
                ScreenState::Lobby => self.run_lobby(font),
                ScreenState::Connecting => self.run_connecting(font),
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
            self.screen_state = ScreenState::Connecting;
        }
    }

    fn run_connecting(&mut self, font: Font) {
        let socket = self
            .socket
            .as_mut()
            .expect("Should only be in connecting state if there exists a socket.");

        self.executor.try_tick();
        socket.accept_new_connections();

        draw_text_ex(
            "Waiting for the other player...",
            15.0,
            35.0,
            TextParams {
                font_size: 30,
                font,
                ..Default::default()
            },
        );

        // if there is a connected peer
        if !socket.connected_peers().is_empty() {
            // create a new game
            info!("Starting new game...");
            self.game_state = GameState::new();
            self.screen_state = ScreenState::Game;

            // create a new ggrs session
            let mut sess_build = SessionBuilder::<GGRSConfig>::new()
                .with_num_players(2)
                .with_max_prediction_window(12)
                .with_fps(60.0 as usize)
                .expect("Invalid FPS")
                .with_input_delay(2);

            // add players
            for (i, player_type) in socket.players().iter().enumerate() {
                sess_build = sess_build
                    .add_player(player_type.clone(), i)
                    .expect("Invalid player added.");
            }

            // start the GGRS session
            let sess = sess_build
                .start_p2p_session(self.socket.take().unwrap())
                .expect("Session could not be created.");
            self.session = Some(sess);

            // reset time variables for frame ticks
            self.last_update = Instant::now();
            self.accumulator = Duration::ZERO;
        }

        // user can abort
        if is_key_pressed(KeyCode::Escape) {
            self.screen_state = ScreenState::Lobby;
            self.socket = None;
            self.executor = LocalExecutor::new();
        }
    }

    fn run_game(&mut self, font: Font) {
        let session = self
            .session
            .as_mut()
            .expect("Should only be in game state if there exists a session.");

        self.executor.try_tick();
        session.poll_remote_clients();
        self.executor.try_tick();

        let mut fps_delta = 1. / 60.0;
        if session.frames_ahead() > 0 {
            fps_delta *= 1.1;
        }

        let delta = Instant::now().duration_since(self.last_update);
        self.accumulator = self.accumulator.saturating_add(delta);
        self.last_update = Instant::now();

        while self.accumulator.as_secs_f64() > fps_delta {
            // decrease accumulator
            self.accumulator = self
                .accumulator
                .saturating_sub(Duration::from_secs_f64(fps_delta));

            // frames are only happening if the sessions are synchronized
            if session.current_state() == SessionState::Running {
                // add input for all local players
                for handle in session.local_player_handles() {
                    session
                        .add_local_input(handle, self.game_state.local_input())
                        .expect("Invalid player handle"); // we always call game.local_input(0) in order to get WASD inputs.
                }

                match session.advance_frame() {
                    Ok(requests) => {
                        self.game_state.handle_requests(requests, &self.sounds);
                    }
                    Err(GGRSError::PredictionThreshold) => {}
                    Err(e) => panic!(
                        "Unknown error happened during P2PSession::<_>::advance_frame(): {e}"
                    ),
                }
            }
        }

        self.render_game(font);
        self.executor.try_tick();
    }

    fn render_game(&mut self, font: Font) {
        request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
        clear_background(BLACK);

        if self.game_state.ball.pos_x > EDGE_RIGHT
            && self
                .game_state
                .ball
                .missed_paddle(self.game_state.right_paddle.pos)
        {
            if self.game_state.left_paddle.score_point() {
                self.game_state = GameState::new();
                self.screen_state = ScreenState::Lobby;
                println!("Left paddle wins!");
            }
            self.game_state.ball.reset_position();
        } else if self.game_state.ball.pos_x < EDGE_LEFT
            && self
                .game_state
                .ball
                .missed_paddle(self.game_state.left_paddle.pos)
        {
            if self.game_state.right_paddle.score_point() {
                self.game_state = GameState::new();
                self.screen_state = ScreenState::Lobby;
                println!("Right paddle wins!");
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
