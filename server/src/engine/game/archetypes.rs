use glam::Vec2;
use legion::prelude::*;

use super::Game;
use super::InputQueue;
use crate::engine::components::all::*;
use crate::engine::events::timed_event::*;
use crate::engine::resources::map_data::*;

impl Game {
    pub fn insert_cores(&mut self, cores: Vec<(usize, CoreData)>) {
        let mut replication_ids = Vec::new();
        let component_set = cores
            .into_iter()
            .map(|(team_id, core)| {
                let replicated = Replicated::new_for_game(self, ReplicatedEntityType::Core);
                replication_ids.push(replicated.id.clone());
                (
                    Transform::new(core.pos, None, None),
                    Team::new(TeamId(team_id as u32)),
                    replicated,
                    //TODO
                    // Collider,
                    // Provides Vision
                    // Visible
                )
            })
            .collect::<Vec<_>>();

        let inserted_entities = self.world.insert((), component_set);
        for (id, entity) in replication_ids.into_iter().zip(inserted_entities) {
            self.replicated_entities.insert(id, *entity);
        }
    }

    pub fn insert_towers(&mut self, towers: Vec<(usize, TowerData)>) {
        let mut replication_ids = Vec::new();
        let component_set = towers
            .into_iter()
            .map(|(team_id, tower)| {
                let replicated = Replicated::new_for_game(self, ReplicatedEntityType::Tower);
                replication_ids.push(replicated.id.clone());
                (
                    Transform::new(tower.pos, None, None),
                    // TODO
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

        let inserted_entities = self.world.insert((), component_set);
        for (id, entity) in replication_ids.into_iter().zip(inserted_entities) {
            self.replicated_entities.insert(id, *entity);
        }
    }

    pub fn insert_spawners(&mut self, spawners: Vec<(usize, SpawnerData)>) {
        let component_set = spawners.into_iter().map(|(team_id, spawner_data)| {
            (
                Transform::new(spawner_data.pos, None, None),
                Team::new(TeamId(team_id as u32)),
                Waypoints::new(spawner_data.waypoints),
            )
        });
        // TODO:
        // Attach TimedEvents to these spawners
        // FOR TESTING ONLY!
        // self.timed_events.push(TimedEvent {
        //     event_type: TimedEventType::Repeating(60),
        //     execute_frame: 60,
        //     name: "Test Event".to_string(),
        //     owner: None,
        //     execute: |_unused: &mut Self| {
        //         println!("fast");
        //     },
        // });

        self.world.insert((), component_set);
    }

    pub fn insert_player(&mut self, player_id: PlayerId) -> Entity {
        let replicated = Replicated::new_for_game(self, ReplicatedEntityType::Character);
        let entity = self
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
                    Attacking {
                        attacking_type: AttackingType::Projectile,
                        range: 50.,
                        reload_time: 1.,
                        state: AttackingState::Ready,
                        target: None,
                        timer: 0.,
                        wind_up_time: 0.25,
                    },
                )),
            )
            .first()
            .unwrap();

        self.replicated_entities
            .insert(replicated.id.clone(), *entity);

        self.player_inputs.insert(player_id, InputQueue::new());

        *entity
    }
}
