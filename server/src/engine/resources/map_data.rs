use glam::Vec2;
use serde::Deserialize;

// use crate::engine::components::all::*;
// use crate::engine::game::Game;

//TODO: Add Neutrals?

#[derive(Debug, Deserialize, Clone)]
pub struct CoreData {
    pub pos: Vec2,
    // Health
}

#[derive(Debug, Deserialize, Clone)]
pub struct TowerData {
    pub pos: Vec2,
    // Health
    // Damage
    // etc
}

#[derive(Debug, Deserialize, Clone)]
pub struct MapData {
    pub teams: Vec<TeamData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TeamData {
    pub core: CoreData,
    pub towers: Vec<TowerData>,
    pub spawners: Vec<SpawnerData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpawnerData {
    pub pos: Vec2,
    pub waypoints: Vec<Vec2>,
}
