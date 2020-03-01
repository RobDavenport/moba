use legion::prelude::*;
use legion::system::SubWorld;

use nalgebra::Vector2;

use crate::engine::components::all::*;

pub fn pawn_update_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_control_system")
        .read_resource::<f32>()
        .with_query(<Write<Pawn>>::query())
        .build(move |command_buffer, mut world, tick_time, query| {
            for (entity, mut pawn) in query.iter_entities(&mut world) {
                match pawn.current_state {
                    PawnState::Idle => on_idle(world, entity),
                    PawnState::Moving(location) => {
                        on_moving(world, entity, *pawn, **tick_time, location)
                    }
                    PawnState::Attacking(target) => {
                        on_attacking(world, entity, **tick_time, target)
                    }
                    PawnState::AttackMoving(location) => {
                        on_attack_moving(world, entity, **tick_time, location)
                    }
                    PawnState::Stopped => on_stopped(world, entity),
                    PawnState::Casting(ability) => on_casting(world, entity, **tick_time, ability),
                    PawnState::Channelling(ability) => {
                        on_channelling(world, entity, **tick_time, ability)
                    }
                    PawnState::Dead => on_dead(world, entity, **tick_time),
                }
            }
        })
}

fn on_idle(world: &mut SubWorld, entity: Entity) {
    //TODO
    //Check if something within range,
    //If so, start attacking, otherwise do nothing
}

fn on_moving(
    world: &mut SubWorld,
    entity: Entity,
    mut pawn: Pawn,
    tick_time: f32,
    location: Vector2<f32>,
) {
    //TODO
    //Move to position
    // if Position == target, change to idle

    if let Some(walks) = world.get_component::<Walks>(entity) {
        if let Some(transform) = world.get_component::<Transform>(entity) {
            move_to_location(pawn, *transform, *walks, tick_time, location);
        }
    }
}

fn on_attacking(world: &mut SubWorld, entity: Entity, tick_time: f32, target: u32) {
    //TODO
    //If it's out of range, move closer
    //if its within range, attack or continue attacking
}

fn on_attack_moving(world: &mut SubWorld, entity: Entity, tick_time: f32, location: Vector2<f32>) {
    //TODO
    //Check nearby hostile, if theres an ememy, start attacking it
    // Else, move to location
}

fn on_stopped(world: &mut SubWorld, entity: Entity) {
    //TODO
    //Do nothing
}

fn on_casting(world: &mut SubWorld, entity: Entity, tick_time: f32, ability: u32) {
    //TODO
}

fn on_channelling(world: &mut SubWorld, entity: Entity, tick_time: f32, ability: u32) {
    //TODO
}

fn on_dead(world: &mut SubWorld, entity: Entity, tick_time: f32) {
    //TODO
}

fn move_to_location(
    mut pawn: Pawn,
    mut transform: Transform,
    walks: Walks,
    tick_time: f32,
    location: Vector2<f32>,
) {
    let distance = (location - transform.position);
    //TODO collision checking?
    let direction = distance.normalize();
    let travel_distance = walks.speed * tick_time;

    if (direction.magnitude() > travel_distance) {
        let travel_vec = direction * travel_distance;
        transform.position += travel_vec;
    } else {
        //It's closer than we can travel in a frame, so just set the position
        transform.position.x = location.x;
        transform.position.y = location.y;
        pawn.next_state = Some(PawnState::Idle);
    }
}

fn check_nearby_hostile() -> Option<Entity> {
    None
}
