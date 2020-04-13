use legion::prelude::*;

use crate::engine::components::all::*;
use crate::engine::input_command::InputCommand;

pub fn pawn_input() -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_input_system")
        .with_query(<Write<ReceiveInput>>::query())
        .write_component::<Moving>()
        .build(|_, mut world, _, query| {
            for (entity, mut input) in query.iter_entities(&mut world) {
                if input.next_command != input.command {
                    match input.next_command {
                        Some(InputCommand::Move(loc, _attacking)) => {
                            if let Some(mut moving) = world.get_component_mut::<Moving>(entity) {
                                moving.target = MoveTarget::Location(loc);
                            }
                        }
                        Some(InputCommand::Attack(target_entity)) => println!("TODO: ATTACK"),
                        Some(InputCommand::MoveDelta(delta)) => println!("TODO: MOVEDELTA"),
                        Some(InputCommand::Recall) => println!("TODO: RECALL"),
                        Some(InputCommand::Stop) => {
                            if let Some(mut moving) = world.get_component_mut::<Moving>(entity) {
                                moving.target = MoveTarget::None;
                            }
                        }
                        Some(InputCommand::UseAbility(idx)) => println!("TODO: USEABILITY"),
                        Some(InputCommand::UseAimedAbility(idx, target_loc)) => {
                            println!("TODO: USEAIMEDABILITY")
                        }
                        Some(InputCommand::UseTargettedAbility(idx, target_entity)) => {
                            println!("TODO: USETARGETTEDABILITY")
                        }
                        None => (),
                    };
                }

                input.command = input.next_command;
                input.next_command = None;
            }
        })
}
