use crate::engine::input_command::*;

use glam::Vec2;
use legion::prelude::*;

use super::Game;
use crate::engine::components::all::*;

pub struct InputQueue {
    pub command: Option<InputCommandGame>,
    pub next_command: Option<InputCommandGame>,
}

impl InputQueue {
    pub fn new() -> Self {
        Self {
            command: None,
            next_command: None,
        }
    }
}

impl Game {
    pub fn input_tick(&mut self) {
        for (id, mut input) in self.player_inputs.iter_mut() {
            if let Some(entity) = self.player_entities.get(&id) {
                if input.next_command != input.command {
                    match input.next_command {
                        Some(InputCommandGame::Move(loc, attacking)) => {
                            on_move(&mut self.world, *entity, loc, attacking)
                        }
                        Some(InputCommandGame::Attack(target_id)) => {
                            on_attack(&mut self.world, *entity, target_id)
                        }
                        Some(InputCommandGame::MoveDelta(_delta)) => {
                            on_move_delta(&mut self.world, *entity)
                        }
                        Some(InputCommandGame::Recall) => on_recall(&mut self.world, *entity),
                        Some(InputCommandGame::Stop) => on_stop(&mut self.world, *entity),
                        Some(InputCommandGame::UseAbility(_idx)) => {
                            on_ability(&mut self.world, *entity)
                        }
                        Some(InputCommandGame::UseAimedAbility(_idx, _target_loc)) => {
                            on_aimed_ability(&mut self.world, *entity)
                        }
                        Some(InputCommandGame::UseTargetedAbility(_idx, _target_entity)) => {
                            on_targeted_ability(&mut self.world, *entity)
                        }
                        Some(InputCommandGame::Invalid) => println!("Got invalid input command..."),
                        None => (),
                    };
                }

                input.command = input.next_command;
                input.next_command = None;
            }
        }
    }
}

pub fn on_move(world: &mut World, entity: Entity, loc: Vec2, _attacking: bool) {
    //TODO: Hostile, search etc
    if let Some(mut moving) = world.get_component_mut::<Moving>(entity) {
        moving.target = MoveTarget::Location(loc);
    }

    if let Some(mut attacking) = world.get_component_mut::<Attacking>(entity) {
        attacking.target = None;
    }
}

pub fn on_attack(world: &mut World, entity: Entity, target: Entity) {
    if target != entity {
        if let Some(mut attacking) = world.get_component_mut::<Attacking>(entity) {
            attacking.target = Some(target);
        }
    }
}

pub fn on_move_delta(_world: &mut World, _entity: Entity) {
    println!("TODO: MOVEDELTA")
}

pub fn on_recall(_world: &mut World, _entity: Entity) {
    println!("TODO: RECALL")
}

pub fn on_stop(world: &mut World, entity: Entity) {
    if let Some(mut moving) = world.get_component_mut::<Moving>(entity) {
        moving.target = MoveTarget::None;
    }
}

pub fn on_ability(_world: &mut World, _entity: Entity) {
    println!("TODO: USEABILITY")
}

pub fn on_aimed_ability(_world: &mut World, _entity: Entity) {
    println!("TODO: USEAIMEDABILITY")
}

pub fn on_targeted_ability(_world: &mut World, _entity: Entity) {
    println!("TODO: USETARGETTEDABILITY")
}
