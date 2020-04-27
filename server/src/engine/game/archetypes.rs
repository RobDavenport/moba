use glam::Vec2;
use legion::prelude::*;

use super::Game;
use crate::engine::components::all::*;
use crate::engine::events::timed_event::*;
use crate::engine::resources::map_data::*;

impl Game {
    pub fn insert_cores(&mut self, cores: Vec<(usize, CoreData)>) {
        let component_set = cores
            .into_iter()
            .map(|(team_id, core)| {
                let replicated = Replicated::new_for_game(self);
                (
                    Transform::new(core.pos, None, None),
                    // Health
                    Team::new(TeamId(team_id as u32)),
                    replicated,
                    // Collider,
                    // Provides Vision
                    // Visible
                )
            })
            .collect::<Vec<_>>();

        self.world.insert((), component_set);
    }

    pub fn insert_towers(&mut self, towers: Vec<(usize, TowerData)>) {
        let component_set = towers
            .into_iter()
            .map(|(team_id, tower)| {
                let replicated = Replicated::new_for_game(self);
                (
                    Transform::new(tower.pos, None, None),
                    // Attacks
                    // Search Hostile
                    // Health
                    // Collider
                    // Provides Vision
                    // Visible
                    Team::new(TeamId(team_id as u32)),
                    replicated,
                )
            })
            .collect::<Vec<_>>();

        self.world.insert((), component_set);
    }

    pub fn insert_spawners(&mut self, spawners: Vec<(usize, SpawnerData)>) {
        let component_set = spawners.into_iter().map(|(team_id, spawner_data)| {
            (
                Transform::new(spawner_data.pos, None, None),
                Team::new(TeamId(team_id as u32)),
                //Waypoints
            )
        });
        // TODO:
        // Attach TimedEvents to these spawners
        // FOR TESTING ONLY!
        self.timed_events.push(TimedEvent {
            event_type: TimedEventType::Repeating(60),
            execute_frame: 60,
            name: "Test Event".to_string(),
            owner: None,
            execute: |_unused: &mut World| {
                println!("fast");
            },
        });

        self.world.insert((), component_set);
    }

    pub fn insert_player(&mut self, player_id: PlayerId) -> Entity {
        let replicated = Replicated::new_for_game(self);
        *self
            .world
            .insert(
                (),
                std::iter::once((
                    Transform::new(Vec2::new(1., 1.), None, None),
                    replicated,
                    PlayerControlled { id: player_id },
                    Moving {
                        base_speed: 125.,
                        target: MoveTarget::None,
                    },
                    ReceiveInput::new(),
                )),
            )
            .first()
            .unwrap()
    }
}
