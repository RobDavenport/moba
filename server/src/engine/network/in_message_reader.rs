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
        Some(Command_oneof_command::moveDelta(cmd)) => Some(GameMessage::InputCommand {
            id,
            command: InputCommand::MoveDelta(Vec2::new(cmd.x, cmd.y)),
        }),
        Some(Command_oneof_command::ability(_cmd)) => {
            // TODO: THIS
            println!("TODO: Ability");
            None
        }
        Some(Command_oneof_command::attack(_cmd)) => {
            // TODO: THIS
            println!("TODO: Attack");
            None
        }
        Some(Command_oneof_command::stop(_cmd)) => Some(GameMessage::InputCommand {
            id,
            command: InputCommand::Stop,
        }),
        Some(Command_oneof_command::recall(_cmd)) => Some(GameMessage::InputCommand {
            id,
            command: InputCommand::Recall,
        }),
        None => None,
    }
}
