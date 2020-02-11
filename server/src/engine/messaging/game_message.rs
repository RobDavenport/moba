use crate::engine::input_command::InputCommand;

//Messages which actually affect the game state in some way or another
pub enum GameMessage {
    ClientConnected,
    ClientDisconnected,
    InputCommand {id: u32, command: InputCommand}
}
