pub enum GameMessage {
    ClientConnected,
    MoveCommand { x: f32, y: f32 },
}
