use graphics::MapLayers;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};

#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, Readable, Writable, PartialEq, Eq)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub group: i32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Readable, Writable)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: MapPosition,
}

#[derive(Clone, Debug, Serialize, Deserialize, Readable, Writable)]
pub struct Tile {
    pub id: Vec<u32>,
}

#[derive(
    Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Debug, Readable, Writable,
)]
pub enum Weather {
    #[default]
    None,
    Rain,
    Snow,
    Sunny,
    Storm,
    Blizzard,
    Heat,
    Hail,
    SandStorm,
    Windy,
}

impl Weather {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Weather::Rain,
            2 => Weather::Snow,
            3 => Weather::Sunny,
            4 => Weather::Storm,
            5 => Weather::Blizzard,
            6 => Weather::Heat,
            7 => Weather::Hail,
            8 => Weather::SandStorm,
            9 => Weather::Windy,
            _ => Weather::None,
        }
    }

    pub fn convert_to_string(&self) -> String {
        match self {
            Weather::None => "None".to_string(),
            Weather::Rain => "Rain".to_string(),
            Weather::Snow => "Snow".to_string(),
            Weather::Sunny => "Sunny".to_string(),
            Weather::Storm => "Storm".to_string(),
            Weather::Blizzard => "Blizzard".to_string(),
            Weather::Heat => "Heat".to_string(),
            Weather::Hail => "Hail".to_string(),
            Weather::SandStorm => "SandStorm".to_string(),
            Weather::Windy => "Windy".to_string(),
        }
    }

    pub fn to_vec_string() -> Vec<String> {
        vec![
            "None".to_string(),
            "Rain".to_string(),
            "Snow".to_string(),
            "Sunny".to_string(),
            "Storm".to_string(),
            "Blizzard".to_string(),
            "Heat".to_string(),
            "Hail".to_string(),
            "SandStorm".to_string(),
            "Windy".to_string(),
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable, Default)]
pub enum MapAttribute {
    #[default]
    Walkable,
    Blocked,
    NpcBlocked,
    Warp(WarpData),
    Sign(String),
    ItemSpawn(ItemSpawnData),
    Storage,
    Shop(u16),
    Count,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Readable, Writable)]
pub struct WarpData {
    pub map_x: i32,
    pub map_y: i32,
    pub map_group: u64,
    pub tile_x: u32,
    pub tile_y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Readable, Writable)]
pub struct ItemSpawnData {
    pub index: u32,
    pub amount: u16,
    pub timer: u64,
}
