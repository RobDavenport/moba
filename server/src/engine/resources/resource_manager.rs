use std::collections::HashMap;
use std::fs;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json;
use tokio::sync::RwLock;

use super::character_data::CharacterData;
use super::map_data::MapData;

struct ResourceToLoad {
    pub name: String,
    pub resource_type: ResourceType,
}

impl ResourceToLoad {
    pub fn new(name: &str, resource_type: ResourceType) -> Self {
        Self {
            name: name.to_string(),
            resource_type,
        }
    }
}

enum ResourceType {
    ResourceList,
    Character,
    Map,
}

pub struct ResourceManager {
    maps: RwLock<HashMap<String, MapData>>,
    characters: RwLock<HashMap<String, CharacterData>>,
}

#[derive(Debug, Deserialize)]
struct ResourceList {
    pub maps: Vec<String>,
    pub characters: Vec<String>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let resources = load_resource::<ResourceList>(ResourceToLoad::new(
            "resourceList",
            ResourceType::ResourceList,
        ));

        let maps = resources
            .maps
            .into_iter()
            .map(|map| {
                (
                    map.clone(),
                    load_resource::<MapData>(ResourceToLoad::new(&map, ResourceType::Map)),
                )
            })
            .collect::<HashMap<String, MapData>>();

        let characters = resources
            .characters
            .into_iter()
            .map(|character| {
                (
                    character.clone(),
                    load_resource::<CharacterData>(ResourceToLoad::new(
                        &character,
                        ResourceType::Character,
                    )),
                )
            })
            .collect::<HashMap<String, CharacterData>>();

        Self {
            maps: RwLock::new(maps),
            characters: RwLock::new(characters),
        }
    }

    pub async fn get_map(&self, name: &str) -> Option<MapData> {
        match self.maps.read().await.get(name) {
            Some(data) => Some(data.clone()),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub async fn get_character(&self, name: &str) -> Option<CharacterData> {
        match self.characters.read().await.get(name) {
            Some(data) => Some(data.clone()),
            None => None,
        }
    }
}

fn load_resource<T: DeserializeOwned>(resource: ResourceToLoad) -> T {
    println!("Loading resource: {}", &resource.name);

    let path = match resource.resource_type {
        ResourceType::ResourceList => format!("./resources/{}.json", &resource.name),
        ResourceType::Character => format!(
            "./resources/characters/{}/{}.json",
            &resource.name, &resource.name
        ),
        ResourceType::Map => format!(
            "./resources/maps/{}/{}.json",
            &resource.name, &resource.name
        ),
    };

    let contents =
        fs::read_to_string(path).expect(&format!("FAILED TO READ FILE: {}", &resource.name));

    serde_json::from_str::<T>(&contents)
        .expect(&format!("FAILED TO PARSE JSON: {}", &resource.name))
}
