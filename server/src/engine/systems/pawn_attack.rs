use legion::prelude::*;

use super::helpers;
use crate::engine::components::all::*;

// Attacking(u32) - transform, attack data, datatarget entity
pub fn pawn_attack(tick_time: f32) -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_attacking_system")
        .with_query(<(Write<Position>, Write<Attacking>)>::query())
        .read_component::<Position>()
        .read_component::<Collider>()
        .build(move |_, mut world, _, query| {
            for (position, mut attacking) in query.iter(&mut world) {
                let valid_target = if let Some(other) = attacking.target {
                    if let Some(other_position) = world.get_component::<Position>(other) {
                        helpers::within_attack_range(
                            &attacking,
                            &position,
                            &other_position,
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
                        tick_ready(&mut attacking, tick_time);
                    }
                    (has_target, AttackingState::WindingUp) => {
                        tick_winding_up(&mut attacking, has_target, tick_time);
                    }
                    (has_target, AttackingState::Cooldown) => {
                        tick_cooldown(&mut attacking, has_target, tick_time);
                    }
                    (false, _) => (),
                };
            }
        })
}

fn tick_ready(attacking: &mut Attacking, dt: f32) {
    attacking.timer += dt;
    attacking.state = AttackingState::WindingUp;
}

fn tick_winding_up(attacking: &mut Attacking, has_target: bool, dt: f32) {
    if has_target {
        attacking.timer += dt;

        if attacking.timer >= attacking.wind_up_time {
            //TODO Do the attack
            match attacking.attacking_type {
                AttackingType::Instant => println!("Pow!"),
                AttackingType::Projectile => println!("Bang!"),
            }
            attacking.state = AttackingState::Cooldown;
            attacking.timer = -(attacking.timer + attacking.reload_time);
        }
    } else {
        attacking.state = AttackingState::Ready;
        attacking.timer = 0.;
    }
}

fn tick_cooldown(attacking: &mut Attacking, has_target: bool, dt: f32) {
    attacking.timer += dt;

    if attacking.timer >= 0. {
        if has_target {
            attacking.state = AttackingState::WindingUp;
            attacking.timer = attacking.timer.abs();
        } else {
            attacking.state = AttackingState::Ready;
            attacking.timer = 0.;
        }
    }
}
