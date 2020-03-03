use legion::prelude::*;
use legion::system::SubWorld;
use nalgebra::Vector2;

use crate::engine::components::all::*;

// pub fn pawn_input() -> Box<dyn Schedulable> {
//     SystemBuilder::new("pawn_input_system")
//         .read_resource::<f32>()
//         .with_query(<(Write<Transform>, Write<Moving>)>::query())
//         .build(|command_buffer, mut world, tick_time, query| {
//             for (entity, (mut transform, mut moving)) in query.iter_entities(&mut world) {
//                 if let Some(location) = moving.location {
//                     if move_to_location(*transform, &location, moving.base_speed, **tick_time) {
//                         moving.location = None;
//                     }
//                 }
//             }
//         })
// }

