use legion::prelude::*;

use super::helpers;
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
        .with_query(<(Write<Position>, Write<Moving>)>::query())
        .build(move |_, mut world, _, query| {
            for (mut position, mut moving) in query.iter(&mut world) {
                match moving.target {
                    MoveTarget::Location(location) => {
                        if helpers::move_to_location(
                            &mut position,
                            location,
                            moving.base_speed,
                            tick_time,
                        ) {
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
