use nalgebra::Vector2;

use super::protobuf::ClientMessage::*;
use crate::engine::components::player_controlled::PlayerId;
use crate::engine::input_command::InputCommand;
use crate::engine::messaging::messages::GameMessage; //todo cut this in favor of reader?

//Move this to a seperate class?
pub fn handle_client_command(command_msg: Command, id: PlayerId) -> Option<GameMessage> {
    match command_msg.commandType {
        Command_CommandType::NONE => None,
        Command_CommandType::MOVECOMMAND => {
            let move_command = command_msg.get_moveCommand();
            Some(GameMessage::InputCommand {
                id,
                command: InputCommand::Move(
                    Vector2::new(move_command.x, move_command.y),
                    move_command.isAttack,
                ),
            })
        }
        Command_CommandType::MOVEDELTA => {
            println!("TODO: MoveDelta");
            None
        }
        Command_CommandType::ATTACK => {
            println!("TODO: Attack");
            None
        }
        Command_CommandType::ABILITY => {
            println!("TODO: Ability");
            None
        }
    }
}
