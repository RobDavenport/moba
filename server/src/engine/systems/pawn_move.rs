use legion::prelude::*;
use legion::system::SubWorld;
use nalgebra::Vector2;

use crate::engine::components::all::*;

// TODO:
// Casting(u32), - active ability
// Channelling(u32), - active ability
// Dead, - respawns

//TODO: check if i need to flush command buffers?

//FIRST CHECK HOSTILE,
//THEN DO MOVEMENT
// Moving(Vector2<f32>), - transform, move to, movement data, NOT SEARCH HOSTILE
// AttackMoving(Vector2<f32>), - transform, move to , movement data + search hostile
pub fn pawn_move() -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_move_system")
        .read_resource::<f32>()
        .with_query(<(Write<Transform>, Read<MoveTo>, Read<MovementData>)>::query())
        .build(|command_buffer, mut world, tick_time, query| {
            for (entity, (mut transform, move_to, data)) in query.iter_entities(&mut world) {
                if move_to_location(*transform, *move_to, *data, **tick_time) {
                    //we already reached the destination, so stop moving
                    command_buffer.remove_component::<MoveTo>(entity);
                }
            }
        })
}

// Returns true if we reached the destination
fn move_to_location(
    mut transform: Transform,
    move_to: MoveTo,
    movement_data: MovementData,
    tick_time: f32,
) -> bool {
    let distance = (move_to.location - transform.position);
    let direction = distance.normalize();
    let travel_distance = movement_data.speed * tick_time;

    if (direction.magnitude() > travel_distance) {
        let travel_vec = direction * travel_distance;
        transform.position += travel_vec;
        false
    } else {
        //It's closer than we can travel in a frame, so just set the position
        transform.position = move_to.location;
        true
    }
}
