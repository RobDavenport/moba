use crate::engine::components::player_controlled::PlayerId;
use crate::engine::input_command::InputCommand;

//Messages which actually affect the game state in some way or another
#[derive(Debug)]
pub enum GameMessage {
    ClientConnected(PlayerId),
    ClientDisconnected(PlayerId),
    InputCommand { id: PlayerId, command: InputCommand },
    Ack { id: PlayerId, new_baseline: u32 },
}
