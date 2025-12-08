use graphics::OtherError;
use serde::{Deserialize, Serialize};
use snafu::Backtrace;
use speedy::{Endianness, Readable, Writable};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub mod data;
pub mod editor;

pub use data::*;
pub use editor::*;

use crate::data_types::{EditorError, Result};

#[derive(Clone, Debug, Serialize, Deserialize, Readable, Writable)]
pub struct MapData {
    pub position: MapPosition,
    pub tile: Vec<Tile>,
    pub dir_block: Vec<u8>,
    pub attribute: Vec<MapAttribute>,
    pub zonespawns: [Vec<(u16, u16)>; 5],
    pub zones: [(u64, [Option<u64>; 5]); 5],
    pub music: Option<String>,
    pub weather: Weather,
}

impl MapData {
    pub fn default(x: i32, y: i32, group: u64) -> Self {
        Self {
            position: MapPosition {
                x,
                y,
                group: group as i32,
            },
            tile: vec![Tile { id: vec![0; 1024] }; 9],
            dir_block: vec![0; 1024],
            attribute: vec![MapAttribute::Walkable; 1024],
            zonespawns: Default::default(),
            zones: Default::default(),
            music: None,
            weather: Weather::default(),
        }
    }

    pub fn save_file(&self) -> Result<()> {
        let name = format!(
            "./data/maps/{}_{}_{}.bin",
            self.position.x, self.position.y, self.position.group
        );

        let bytes = self.write_to_vec().unwrap();

        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&name)
        {
            Ok(mut file) => {
                if let Err(e) = file.write(bytes.as_slice()) {
                    Err(EditorError::Other {
                        source: OtherError::new(&format!("File Error Err {e:?}",)),
                        backtrace: Backtrace::new(),
                    })
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(EditorError::Other {
                source: OtherError::new(&format!("Failed to open {name}, Err {e:?}",)),
                backtrace: Backtrace::new(),
            }),
        }
    }

    pub fn save_temp_file(&self, exist: bool) -> Result<()> {
        let name = if exist {
            format!(
                "./temp/{}_{}_{}.bin",
                self.position.x, self.position.y, self.position.group
            )
        } else {
            "./temp/recovery.bin".to_string()
        };

        let bytes = self.write_to_vec().unwrap();

        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&name)
        {
            Ok(mut file) => {
                if let Err(e) = file.write(bytes.as_slice()) {
                    Err(EditorError::Other {
                        source: OtherError::new(&format!("File Error Err {e:?}",)),
                        backtrace: Backtrace::new(),
                    })
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(EditorError::Other {
                source: OtherError::new(&format!("Failed to open {name}, Err {e:?}",)),
                backtrace: Backtrace::new(),
            }),
        }
    }
}

pub fn save_temp_file(x: i32, y: i32, group: u64, data: &MapData, exist: bool) -> Result<()> {
    let name = if exist {
        format!("./temp/{x}_{y}_{group}.bin")
    } else {
        "./temp/recovery.bin".to_string()
    };

    let bytes = data.write_to_vec().unwrap();

    match OpenOptions::new().write(true).create_new(true).open(&name) {
        Ok(mut file) => {
            if let Err(e) = file.write(bytes.as_slice()) {
                Err(EditorError::Other {
                    source: OtherError::new(&format!("File Error Err {e:?}",)),
                    backtrace: Backtrace::new(),
                })
            } else {
                Ok(())
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => data.save_temp_file(exist),
        Err(e) => Err(EditorError::Other {
            source: OtherError::new(&format!("Failed to open {name}, Err {e:?}",)),
            backtrace: Backtrace::new(),
        }),
    }
}

pub fn create_map_file(x: i32, y: i32, group: u64, data: &MapData) -> Result<()> {
    let name = format!("./data/maps/{x}_{y}_{group}.bin");

    let bytes = data.write_to_vec().unwrap();

    match OpenOptions::new().write(true).create_new(true).open(&name) {
        Ok(mut file) => {
            if let Err(e) = file.write(bytes.as_slice()) {
                Err(EditorError::Other {
                    source: OtherError::new(&format!("File Error Err {e:?}",)),
                    backtrace: Backtrace::new(),
                })
            } else {
                Ok(())
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(EditorError::Other {
            source: OtherError::new(&format!("Failed to open {name}, Err {e:?}",)),
            backtrace: Backtrace::new(),
        }),
    }
}

pub fn load_map_file(x: i32, y: i32, group: u64, create_file: bool) -> Result<MapData> {
    if !is_map_exist(x, y, group) {
        let data = MapData::default(x, y, group);
        if create_file {
            match create_map_file(x, y, group, &MapData::default(x, y, group)) {
                Ok(()) => return Ok(data),
                Err(e) => return Err(e),
            }
        } else {
            return Ok(data);
        }
    }

    let name: String = format!("./data/maps/{x}_{y}_{group}.bin");
    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(MapData::read_from_buffer(&bytes).unwrap())
        }
        Err(_) => Ok(MapData::default(x, y, group)),
    }
}

pub fn load_temp_map_file(x: i32, y: i32, group: u64) -> Result<MapData> {
    if !is_temp_map_exist(x, y, group) {
        let data = MapData::default(x, y, group);
        return Ok(data);
    }

    let name: String = format!("./temp/{x}_{y}_{group}.bin");
    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(MapData::read_from_buffer(&bytes).unwrap())
        }
        Err(_) => Ok(MapData::default(x, y, group)),
    }
}

pub fn delete_temp_map_file(x: i32, y: i32, group: u64) -> Result<()> {
    let name: String = format!("./temp/{x}_{y}_{group}.bin");
    fs::remove_file(name)?;
    Ok(())
}

pub fn load_recovery_map_file() -> Result<MapData> {
    let name: String = "./temp/recovery.bin".to_string();

    if !Path::new(&name).exists() {
        let data = MapData::default(0, 0, 0);
        return Ok(data);
    }

    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(MapData::read_from_buffer(&bytes).unwrap())
        }
        Err(_) => Ok(MapData::default(0, 0, 0)),
    }
}

pub fn delete_recovery_map_file() -> Result<()> {
    let name: String = "./temp/recovery.bin".to_string();
    fs::remove_file(name)?;
    Ok(())
}

pub fn is_recovery_map_file_exist() -> bool {
    Path::new("./temp/recovery.bin").exists()
}

pub fn is_temp_map_exist(x: i32, y: i32, group: u64) -> bool {
    let name = format!("./temp/{x}_{y}_{group}.bin");
    Path::new(&name).exists()
}

pub fn is_map_exist(x: i32, y: i32, group: u64) -> bool {
    let name = format!("./data/maps/{x}_{y}_{group}.bin");
    Path::new(&name).exists()
}

pub fn save_and_clear_map(x: i32, y: i32, group: u64) -> Result<()> {
    if let Ok(data) = load_temp_map_file(x, y, group) {
        delete_temp_map_file(x, y, group)?;

        if !is_map_exist(x, y, group) {
            return create_map_file(x, y, group, &data);
        }

        data.save_file()?;
    }
    Ok(())
}
