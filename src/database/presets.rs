use graphics::OtherError;
use serde::{Deserialize, Serialize};
use snafu::Backtrace;
use speedy::{Endianness, Readable, Writable};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::data_types::{EditorError, MAX_PRESETS, Result};

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable, Default,
)]
pub enum PresetTypeList {
    #[default]
    Normal,
    Animated,
    AutoTile,
    AutotileAnimated,
}

impl PresetTypeList {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => PresetTypeList::Animated,
            2 => PresetTypeList::AutoTile,
            3 => PresetTypeList::AutotileAnimated,
            _ => PresetTypeList::Normal,
        }
    }
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable, Default,
)]
pub struct PresetPos {
    pub x: u16,
    pub y: u16,
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable, Default,
)]
pub struct PresetFrames {
    pub start: PresetPos,
    pub end: PresetPos,
    pub tileset: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable, Default)]
pub struct PresetData {
    pub name: String,
    pub draw_type: PresetTypeList,
    pub frames: [PresetFrames; 4],
}

pub struct Presets {
    pub data: Vec<PresetData>,
    pub selected_preset_tiles: Vec<usize>,
}

impl Presets {
    pub fn load_data() -> Result<Self> {
        let mut data = Vec::with_capacity(MAX_PRESETS);

        for i in 0..MAX_PRESETS {
            let name: String = format!("./mapeditor/data/presets/p{i}.bin");

            data.push(if !Path::new(&name).exists() {
                let pd_data = PresetData::default();

                let bytes = pd_data.write_to_vec().unwrap();

                match OpenOptions::new().write(true).create_new(true).open(&name) {
                    Ok(mut file) => {
                        if let Err(e) = file.write(bytes.as_slice()) {
                            return Err(EditorError::Other {
                                source: OtherError::new(&format!("File Error Err {e:?}",)),
                                backtrace: Backtrace::new(),
                            });
                        }
                    }
                    Err(e) => {
                        return Err(EditorError::Other {
                            source: OtherError::new(&format!("Failed to open {name}, Err {e:?}",)),
                            backtrace: Backtrace::new(),
                        });
                    }
                }

                pd_data
            } else {
                match OpenOptions::new().read(true).open(name) {
                    Ok(mut file) => {
                        let mut bytes = Vec::new();
                        file.read_to_end(&mut bytes)?;
                        PresetData::read_from_buffer(&bytes).unwrap()
                    }
                    Err(_) => PresetData::default(),
                }
            });
        }

        Ok(Presets {
            data,
            selected_preset_tiles: Vec::with_capacity(52),
        })
    }

    pub fn save_preset(&self, index: usize) -> Result<()> {
        let name: String = format!("./mapeditor/data/presets/p{index}.bin");

        let bytes = self.data[index].write_to_vec().unwrap();

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
