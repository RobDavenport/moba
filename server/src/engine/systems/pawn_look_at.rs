use legion::prelude::*;

use super::helpers;
use crate::engine::components::all::*;

pub fn pawn_look_at() -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_look_at")
        .with_query(<(Write<Rotation>, Read<Position>)>::query())
        .read_component::<Moving>()
        .read_component::<Attacking>()
        .read_component::<Position>()
        .build(move |_, mut world, _, query| {
            for (entity, (mut rotation, position)) in query.iter_entities(&mut world) {
                if let Some(attacking) = world.get_component::<Attacking>(entity) {
                    if let Some(target) = attacking.target {
                        if let Some(other_position) = world.get_component::<Position>(target) {
                            helpers::look_at_location(
                                &mut rotation,
                                &position.0,
                                &other_position.0,
                            );
                            continue;
                        }
                    }
                }

                if let Some(moving) = world.get_component::<Moving>(entity) {
                    match moving.target {
                        MoveTarget::None => continue,
                        MoveTarget::Location(loc) => {
                            helpers::look_at_location(&mut rotation, &position.0, &loc)
                        }
                        MoveTarget::Entity(target) => {
                            if let Some(other_position) = world.get_component::<Position>(target) {
                                helpers::look_at_location(
                                    &mut rotation,
                                    &position.0,
                                    &other_position.0,
                                )
                            }
                        }
                    }
                }
            }
        })
}
