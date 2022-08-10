use good_game_rusty_pong::game::Game;

use macroquad::texture::Texture2D;

#[macroquad::main("GoodGameRustyPong")]
async fn main() {
    let logo = Texture2D::from_file_with_format(include_bytes!("../assets/logo.png"), None);
    Game::new(logo).run().await;
}
