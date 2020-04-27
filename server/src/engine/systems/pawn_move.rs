use glam::Vec2;
use legion::prelude::*;

use crate::engine::components::all::*;

// TODO:
// Casting(u32), - active ability
// Channelling(u32), - active ability
// Dead, - respawns

//TODO: check if i need to flush command buffers?

//FIRST CHECK HOSTILE,
//THEN DO MOVEMENT
// Moving(Vec2), - transform, move to, movement data, NOT SEARCH HOSTILE
// AttackMoving(Vec2), - transform, move to , movement data + search hostile
pub fn pawn_move(tick_time: f32) -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_move_system")
        .with_query(<(Write<Transform>, Write<Moving>)>::query())
        .build(move |_, mut world, _, query| {
            for (mut transform, mut moving) in query.iter(&mut world) {
                match moving.target {
                    MoveTarget::Location(location) => {
                        if move_to_location(&mut transform, location, moving.base_speed, tick_time)
                        {
                            moving.target = MoveTarget::None;
                        }
                    }
                    MoveTarget::Entity(_entity) => {
                        println!("TODO: PAWN MOVE TO ENTITY UNFINISHED!")
                    }
                    MoveTarget::None => (),
                }
            }
        })
}

// Returns true if we reached the destination
fn move_to_location(
    transform: &mut Transform,
    location: Vec2,
    movement_speed: f32,
    tick_time: f32,
) -> bool {
    let distance = location - transform.position;
    transform.rotation = distance.x().atan2(distance.y()) * (180.0 / std::f32::consts::PI);
    let travel_distance = movement_speed * tick_time;

    if distance.length() > travel_distance {
        let direction = distance.normalize();
        let travel_vec = direction * travel_distance;
        transform.position += travel_vec;
        false
    } else {
        //It's closer than we can travel in a frame, so just set the position
        transform.position = location;
        true
    }
}
