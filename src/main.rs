use good_game_rusty_pong::game::Game;

use macroquad::texture::{load_texture, Texture2D};

#[macroquad::main("GoodGameRustyPong")]
async fn main() {
    let logo: Texture2D = load_texture("assets/logo.png").await.unwrap();
    Game::new(logo).run().await;
}
