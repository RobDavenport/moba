use std::iter::*;

use legion::prelude::*;

use crate::engine::resources::character_data::*;
use crate::engine::resources::map_data::*;
use crate::engine::systems::*;

use super::Game;

impl Game {
    pub fn init_game(&mut self, map_data: MapData, characters: Vec<CharacterData>) -> &mut Self {
        self.init_map(map_data);
        self.init_characters(characters);

        self
    }

    fn init_map(&mut self, map_data: MapData) {
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

    fn init_characters(&mut self, character_data: Vec<CharacterData>) {
        //TODO: this func
    }
}

pub fn init_systems(tick_time: f32) -> Vec<Box<dyn Schedulable>> {
    let mut out = Vec::new();

    println!("Initialized game systems with tick time of {}s", tick_time);

    out.push(pawn_input::pawn_input());
    out.push(pawn_move::pawn_move(tick_time));

    out
}
