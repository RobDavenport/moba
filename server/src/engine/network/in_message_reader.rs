use glam::Vec2;

use super::protobuf::ClientMessage::*;
use crate::engine::components::player_controlled::PlayerId;
use crate::engine::input_command::InputCommand;
use crate::engine::messaging::messages::GameMessage; //todo cut this in favor of reader?

//Move this to a seperate class?
pub fn handle_client_command(command_msg: Command, id: PlayerId) -> Option<GameMessage> {
    match command_msg.command {
        Some(Command_oneof_command::moveCommand(cmd)) => Some(GameMessage::InputCommand {
            id,
            command: InputCommand::Move(Vec2::new(cmd.x, cmd.y), cmd.isAttack),
        }),
        Some(Command_oneof_command::moveDelta(cmd)) => {
            println!("TODO: MoveDelta");
            None
        }
        Some(Command_oneof_command::ability(cmd)) => {
            println!("TODO: Ability");
            None
        }
        Some(Command_oneof_command::attack(cmd)) => {
            println!("TODO: Attack");
            None
        }
        None => None,
    }
}
