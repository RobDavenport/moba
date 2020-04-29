use glam::Vec2;
use legion::borrow::Ref;

use crate::engine::components::all::*;

pub fn within_attack_range(
    attacking: &Attacking,
    position: &Position,
    other_position: &Position,
    other_collision: Option<Ref<Collider>>,
) -> bool {
    let radius = if let Some(collider) = other_collision {
        collider.radius
    } else {
        0.
    };
    let distance = (position.0 - other_position.0).length() - radius;

    distance <= attacking.range
}

// Returns true if we reached the destination
pub fn move_to_location(
    position: &mut Position,
    location: Vec2,
    movement_speed: f32,
    tick_time: f32,
) -> bool {
    let distance = location - position.0;
    let travel_distance = movement_speed * tick_time;

    if distance.length() > travel_distance {
        let direction = distance.normalize();
        let travel_vec = direction * travel_distance;
        position.0 += travel_vec;
        false
    } else {
        //It's closer than we can travel in a frame, so just set the position
        position.0 = location;
        true
    }
}

pub fn look_at_location(rotation: &mut Rotation, position: &Vec2, target: &Vec2) {
    let target = *target - *position;
    rotation.0 = target.x().atan2(target.y()) * (180.0 / std::f32::consts::PI);
}
