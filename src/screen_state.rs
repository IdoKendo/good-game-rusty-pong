/// Represent which screen is displayed
pub enum ScreenState {
    /// Main screen
    Lobby,
    /// Loading screen while connection is being established
    Connecting,
    /// Main game
    Game,
}
