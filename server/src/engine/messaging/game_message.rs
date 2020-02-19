use crate::engine::input_command::InputCommand;

//Messages which actually affect the game state in some way or another
#[derive(Debug)]
pub enum GameMessage {
    ClientConnected(u32),
    ClientDisconnected(u32),
    InputCommand { id: u32, command: InputCommand },
}
