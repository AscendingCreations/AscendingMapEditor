use graphics::*;

use serde::{Deserialize, Serialize};
use snafu::Backtrace;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;

use winit::{event::*, keyboard::*};

use crate::data_types::{EditorError, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub hide_fps: bool,
    pub zoom: f32,
}

impl ConfigData {
    pub fn default() -> Self {
        Self {
            hide_fps: false,
            zoom: 1.0,
        }
    }

    pub fn save_config(&self) -> Result<()> {
        let name = "./map_config.json".to_string();

        match OpenOptions::new().truncate(true).write(true).open(&name) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(&file, self) {
                    Err(EditorError::Other {
                        source: OtherError::new(&format!("Serdes File Error Err {e:?}",)),
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

    pub fn reset_config(&mut self) {
        let default_config = ConfigData::default();
        *self = default_config;
    }

    pub fn set_data(&mut self, data: ConfigData) {
        *self = data;
    }
}

pub fn create_config(data: &ConfigData) -> Result<()> {
    let name = "./map_config.json".to_string();

    match OpenOptions::new().write(true).create_new(true).open(&name) {
        Ok(file) => {
            if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                Err(EditorError::Other {
                    source: OtherError::new(&format!("Serdes File Error Err {e:?}",)),
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

pub fn load_config() -> ConfigData {
    if !is_config_exist() {
        let data = ConfigData::default();
        match create_config(&ConfigData::default()) {
            Ok(()) => return data,
            Err(_) => return ConfigData::default(),
        }
    }

    match OpenOptions::new().read(true).open("./map_config.json") {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error {e:?}");
                    ConfigData::default()
                }
            }
        }
        Err(_) => ConfigData::default(),
    }
}

pub fn is_config_exist() -> bool {
    let name = "./map_config.json".to_string();
    Path::new(&name).exists()
}
