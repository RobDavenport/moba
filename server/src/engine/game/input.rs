use crate::engine::input_command::InputCommand;

use std::collections::HashMap;

use glam::Vec2;
use legion::prelude::*;

use super::Game;
use crate::engine::components::all::*;

use crate::engine::systems::helpers;

pub struct InputQueue {
    pub command: Option<InputCommand>,
    pub next_command: Option<InputCommand>,
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
            let entity = self.player_entities.get(&id).unwrap();
            if input.next_command != input.command {
                match input.next_command {
                    Some(InputCommand::Move(loc, attacking)) => {
                        on_move(&mut self.world, &entity, loc, attacking)
                    }
                    Some(InputCommand::Attack(target_id)) => on_attack(
                        &mut self.world,
                        entity,
                        target_id,
                        &self.replicated_entities,
                    ),
                    Some(InputCommand::MoveDelta(_delta)) => on_move_delta(&mut self.world, entity),
                    Some(InputCommand::Recall) => on_recall(&mut self.world, entity),
                    Some(InputCommand::Stop) => on_stop(&mut self.world, entity),
                    Some(InputCommand::UseAbility(_idx)) => on_ability(&mut self.world, entity),
                    Some(InputCommand::UseAimedAbility(_idx, _target_loc)) => {
                        on_aimed_ability(&mut self.world, entity)
                    }
                    Some(InputCommand::UseTargetedAbility(_idx, _target_entity)) => {
                        on_targeted_ability(&mut self.world, entity)
                    }
                    None => (),
                };
            }

            input.command = input.next_command;
            input.next_command = None;
        }
    }
}

pub fn on_move(world: &mut World, entity: &Entity, loc: Vec2, _attacking: bool) {
    //TODO: Hostile, search etc
    if let Some(mut moving) = world.get_component_mut::<Moving>(*entity) {
        moving.target = MoveTarget::Location(loc);
    }
}

pub fn on_attack(
    world: &mut World,
    entity: &Entity,
    target_id: u32,
    replicated_entities: &HashMap<ReplicationId, Entity>,
) {
    if let Some(target) = replicated_entities.get(&ReplicationId(target_id)) {
        if let Some(mut attacking) = world.get_component_mut::<Attacking>(*entity) {
            attacking.target = Some(*target);
        }
    }
}

pub fn on_move_delta(_world: &mut World, _entity: &Entity) {
    println!("TODO: MOVEDELTA")
}

pub fn on_recall(_world: &mut World, _entity: &Entity) {
    println!("TODO: RECALL")
}

pub fn on_stop(world: &mut World, entity: &Entity) {
    if let Some(mut moving) = world.get_component_mut::<Moving>(*entity) {
        moving.target = MoveTarget::None;
    }
}

pub fn on_ability(_world: &mut World, _entity: &Entity) {
    println!("TODO: USEABILITY")
}

pub fn on_aimed_ability(_world: &mut World, _entity: &Entity) {
    println!("TODO: USEAIMEDABILITY")
}

pub fn on_targeted_ability(_world: &mut World, _entity: &Entity) {
    println!("TODO: USETARGETTEDABILITY")
}
