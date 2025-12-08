use graphics::*;

use crate::database::{ItemSpawnData, MapAttribute, WarpData};

impl MapAttribute {
    pub fn as_str<'a>(attribute: u32) -> &'a str {
        match attribute {
            0 => "Walkable",
            1 => "Blocked",
            2 => "NpcBlocked",
            3 => "Warp",
            4 => "Sign",
            5 => "Item",
            6 => "Storage",
            7 => "Shop",
            _ => "",
        }
    }

    pub fn as_map_str<'a>(attribute: &MapAttribute) -> &'a str {
        match attribute {
            MapAttribute::Blocked => "B",
            MapAttribute::NpcBlocked => "N",
            MapAttribute::Warp(_) => "W",
            MapAttribute::Sign(_) => "S",
            MapAttribute::ItemSpawn(_) => "I",
            MapAttribute::Storage => "S",
            MapAttribute::Shop(_) => "S",
            _ => "",
        }
    }

    pub fn get_color(attribute: &MapAttribute) -> Color {
        match attribute {
            MapAttribute::Blocked => Color::rgba(200, 10, 10, 100),
            MapAttribute::NpcBlocked => Color::rgba(200, 50, 10, 100),
            MapAttribute::Warp(_) => Color::rgba(10, 10, 200, 100),
            MapAttribute::Sign(_) => Color::rgba(10, 200, 10, 100),
            MapAttribute::ItemSpawn(_) => Color::rgba(180, 180, 180, 100),
            MapAttribute::Storage => Color::rgba(160, 170, 20, 255),
            MapAttribute::Shop(_) => Color::rgba(200, 50, 100, 255),
            _ => Color::rgba(0, 0, 0, 0),
        }
    }

    pub fn convert_to_plain_enum(attribute: u32) -> Self {
        match attribute {
            1 => MapAttribute::Blocked,
            2 => MapAttribute::NpcBlocked,
            3 => MapAttribute::Warp(WarpData::default()),
            4 => MapAttribute::Sign(String::new()),
            5 => MapAttribute::ItemSpawn(ItemSpawnData::default()),
            6 => MapAttribute::Storage,
            7 => MapAttribute::Shop(0),
            _ => MapAttribute::Walkable,
        }
    }

    pub fn convert_to_num(attribute: &MapAttribute) -> u32 {
        match attribute {
            MapAttribute::Blocked => 1,
            MapAttribute::NpcBlocked => 2,
            MapAttribute::Warp(_) => 3,
            MapAttribute::Sign(_) => 4,
            MapAttribute::ItemSpawn(_) => 5,
            MapAttribute::Storage => 6,
            MapAttribute::Shop(_) => 7,
            _ => 0,
        }
    }

    pub fn to_editor(&self) -> EditorMapAttribute {
        match self {
            MapAttribute::Blocked => EditorMapAttribute::Blocked,
            MapAttribute::NpcBlocked => EditorMapAttribute::NpcBlocked,
            MapAttribute::Warp(_) => EditorMapAttribute::Warp,
            MapAttribute::Sign(_) => EditorMapAttribute::Sign,
            MapAttribute::ItemSpawn(_) => EditorMapAttribute::ItemSpawn,
            MapAttribute::Storage => EditorMapAttribute::Storage,
            MapAttribute::Shop(_) => EditorMapAttribute::Shop,
            _ => EditorMapAttribute::Walkable,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorMapAttribute {
    Walkable,
    Blocked,
    NpcBlocked,
    Warp,
    Sign,
    ItemSpawn,
    Storage,
    Shop,
    Count,
}

impl EditorMapAttribute {
    pub fn convert_to_plain_enum(attribute: u32) -> Self {
        match attribute {
            1 => EditorMapAttribute::Blocked,
            2 => EditorMapAttribute::NpcBlocked,
            3 => EditorMapAttribute::Warp,
            4 => EditorMapAttribute::Sign,
            5 => EditorMapAttribute::ItemSpawn,
            6 => EditorMapAttribute::Storage,
            7 => EditorMapAttribute::Shop,
            _ => EditorMapAttribute::Walkable,
        }
    }
}
