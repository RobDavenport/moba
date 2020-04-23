use std::iter::*;

use legion::prelude::*;

use crate::engine::resources::{map_data::*, map_loader};
use crate::engine::systems::*;

use super::Game;

impl Game {
    pub fn init_game(&mut self, map_name: &str) -> &mut Self {
        self.load_map(map_name);

        self
    }

    pub fn load_map(&mut self, map_name: &str) {
        let map_data = map_loader::load_map_data(map_name);

        self.init_entities_from_map_data(map_data);
    }

    fn init_entities_from_map_data(&mut self, map_data: MapData) {
        let mut cores = Vec::new();
        let mut towers = Vec::new();
        let mut spawners = Vec::new();

        map_data
            .teams
            .into_iter()
            .enumerate()
            .for_each(|(team_id, data)| {
                cores.push((team_id, data.core));
                data.towers
                    .into_iter()
                    .for_each(|tower| towers.push((team_id, tower)));
                data.spawners
                    .into_iter()
                    .for_each(|spawner| spawners.push((team_id, spawner)));
            });

        self.insert_cores(cores);
        self.insert_towers(towers);
        self.insert_spawners(spawners);
    }
}

pub fn init_systems(tick_time: f32) -> Vec<Box<dyn Schedulable>> {
    let mut out = Vec::new();

    println!("Initialized game systems with tick time of {}s", tick_time);

    out.push(pawn_input::pawn_input());
    out.push(pawn_move::pawn_move(tick_time));

    out
}
