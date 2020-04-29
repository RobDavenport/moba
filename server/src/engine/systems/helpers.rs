use glam::Vec2;
use legion::borrow::Ref;
use legion::prelude::*;
use legion::system::SubWorld;

use crate::engine::components::all::*;

pub fn within_attack_range(
    attacking: &Attacking,
    transform: &Transform,
    other_transform: &Transform,
    other_collision: Option<Ref<Collider>>,
) -> bool {
    let radius = if let Some(collider) = other_collision {
        collider.radius
    } else {
        0.
    };
    let distance = (transform.position - other_transform.position).length() - radius;

    distance <= attacking.range
}

// Returns true if we reached the destination
pub fn move_to_location(
    transform: &mut Transform,
    location: Vec2,
    movement_speed: f32,
    tick_time: f32,
) -> bool {
    let distance = location - transform.position;
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

pub fn look_at_location(transform: &mut Transform, location: Vec2) {
    let target = location - transform.position;
    transform.rotation = target.x().atan2(target.y()) * (180.0 / std::f32::consts::PI);
}
