use indexmap::IndexMap;
use std::fs;
use std::io;
use std::path::Path;

use graphics::*;

use crate::data_types::Result;
use crate::data_types::TEXTURE_SIZE;

pub enum GuiTexture {
    VerticalArrow,
    ToolIcon,
    LayerIcon,
    LayerTopIcon,
    AnimLayerIcon,
    TileSelect,
    TabIcon,
    PresetPreview,
    TilesheetSelect,
    DirBlock,
    PreviewBlocker,
}

pub struct TilesheetData {
    pub name: String,
    pub img: usize,
    pub tile: TileSheet,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
    pub file: u32,
}

pub struct TextureAllocation {
    pub interface: Vec<usize>,
    pub tilesheet: Vec<TilesheetData>,
    // This will be used for eyedropper tool
    pub tile_index_loc: IndexMap<usize, TilePos, ahash::RandomState>,
    pub tile_pos_loc: IndexMap<TilePos, usize, ahash::RandomState>,
}

impl TextureAllocation {
    pub fn new(
        img_atlases: &mut AtlasSet,
        map_atlases: &mut AtlasSet,
        renderer: &GpuRenderer,
    ) -> Result<Self> {
        // This is how we load a image into a atlas/Texture. It returns the location of the image
        // within the texture. its x, y, w, h.  Texture loads the file. group_uploads sends it to the Texture
        // renderer is used to upload it to the GPU when done.
        let paths = [
            "mapeditor/images/vertical_arrow.png",
            "mapeditor/images/tool_icon.png",
            "mapeditor/images/layer_button.png",
            "mapeditor/images/layer_button_top.png",
            "mapeditor/images/anim_layer_button.png",
            "mapeditor/images/tile_select.png",
            "mapeditor/images/tab_icon.png",
            "mapeditor/images/preset_preview.png",
            "mapeditor/images/tilesheet_select.png",
            "mapeditor/images/dir_block.png",
            "mapeditor/images/preview_blocker.png",
        ];

        let mut interface = Vec::with_capacity(paths.len());
        for path in paths {
            interface.push(
                Texture::from_file(path)?
                    .upload(img_atlases, renderer)
                    .ok_or_else(|| OtherError::new("failed to upload image"))?,
            )
        }

        let mut tile_index_loc = IndexMap::default();
        let mut tile_pos_loc = IndexMap::default();
        let mut tilesheet = Vec::new();
        let mut count = 0;
        let mut path_found = true;
        while path_found {
            let path = format!("./images/tiles/t{count}.png");
            if Path::new(&path).exists() {
                let res = TilesheetData {
                    name: format!("t{count}.png"),
                    img: Texture::from_file(path)?
                        .upload(img_atlases, renderer)
                        .ok_or_else(|| OtherError::new("failed to upload image"))?,
                    tile: Texture::from_file(format!("images/tiles/t{count}.png"))?
                        .new_tilesheet(map_atlases, renderer, TEXTURE_SIZE)
                        .ok_or_else(|| OtherError::new("failed to upload tiles"))?,
                };

                // Store the tile location
                for tile in &res.tile.tiles {
                    if tile.tex_id > 0 {
                        let tilepos = TilePos {
                            x: tile.x,
                            y: tile.y,
                            file: count,
                        };

                        tile_index_loc.insert(tile.tex_id, tilepos);
                        tile_pos_loc.insert(tilepos, tile.tex_id);
                    }
                }

                tilesheet.push(res);

                count += 1;
            } else {
                path_found = false;
            }
        }

        // Complete! We can now pass the result
        Ok(Self {
            interface,
            tilesheet,
            tile_index_loc,
            tile_pos_loc,
        })
    }
}
