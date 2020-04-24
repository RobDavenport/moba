use std::fs;

use serde_json;

use super::map_data::MapData;

pub fn load_map_data(map_name: &str) -> MapData {
    println!("Loading map: {}", &map_name);

    let contents = fs::read_to_string(format!("./resources/maps/{}/{}.json", &map_name, &map_name))
        .expect(&format!("FAILED TO READ FILE: {}", &map_name));
    let data = serde_json::from_str::<MapData>(&contents)
        .expect(&format!("FAILED TO READ JSON: {}", &map_name));

    println!("Map {}/mapData.json loaded successfully.", &map_name);

    data
}
