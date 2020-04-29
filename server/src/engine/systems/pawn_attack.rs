use legion::borrow::Ref;
use legion::prelude::*;

use super::helpers;
use crate::engine::components::all::*;

// Attacking(u32) - transform, attack data, datatarget entity
pub fn pawn_attack(tick_time: f32) -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_attacking_system")
        .with_query(<(Write<Transform>, Write<Attacking>)>::query())
        .read_component::<Transform>()
        .read_component::<Collider>()
        .build(move |_, mut world, _, query| {
            for (mut transform, mut attacking) in query.iter(&mut world) {
                let valid_target = if let Some(other) = attacking.target {
                    if let Some(other_transform) = world.get_component::<Transform>(other) {
                        helpers::look_at_location(&mut transform, other_transform.position);
                        helpers::within_attack_range(
                            &attacking,
                            &transform,
                            &other_transform,
                            world.get_component::<Collider>(other),
                        )
                    } else {
                        false
                    }
                } else {
                    false
                };

                match (valid_target, attacking.state) {
                    (true, AttackingState::Ready) => {
                        tick_ready(&mut attacking, &tick_time);
                    }
                    (has_target, AttackingState::WindingUp) => {
                        tick_winding_up(&mut attacking, has_target, &tick_time);
                    }
                    (has_target, AttackingState::Cooldown) => {
                        tick_cooldown(&mut attacking, has_target, &tick_time);
                    }
                    (false, _) => (),
                };
            }
        })
}

fn tick_ready(attacking: &mut Attacking, dt: &f32) {
    println!("ready!");
    attacking.timer += dt;
    attacking.state = AttackingState::WindingUp;
}

fn tick_winding_up(attacking: &mut Attacking, has_target: bool, dt: &f32) {
    if has_target {
        println!("Winding up!");
        attacking.timer += dt;

        if attacking.timer >= attacking.wind_up_time {
            //TODO Do the attack
            println!("POW!");
            attacking.state = AttackingState::Cooldown;
            attacking.timer = -attacking.timer;
        }
    } else {
        println!("Target lost!");
        attacking.state = AttackingState::Ready;
        attacking.timer = 0.;
    }
}

fn tick_cooldown(attacking: &mut Attacking, has_target: bool, dt: &f32) {
    println!("Cooldown");
    attacking.timer += dt;

    if attacking.timer >= 0. {
        if has_target {
            println!("reload!");
            attacking.state = AttackingState::WindingUp;
            attacking.timer = attacking.timer.abs();
        } else {
            println!("Find a new target");
            attacking.state = AttackingState::Ready;
            attacking.timer = 0.;
        }
    }
}
