use std::collections::VecDeque;

use graphics::*;
use indexmap::IndexSet;

use crate::{
    content::{Content, get_attribute_visual, get_tile_pos},
    data_types::*,
    database::{MapAttribute, MapData, MapPosition},
    renderer::SystemHolder,
};

pub enum ChangePlaceHolder {
    None,
    Tiles,
    Attributes,
}

#[derive(Clone)]
pub struct TileChangeData {
    pub x: u16,
    pub y: u16,
    pub layer: usize,

    pub from: usize,
    pub to: usize,
}

#[derive(Clone)]
pub struct AttrChangeData {
    pub x: u16,
    pub y: u16,

    pub from: MapAttribute,
    pub to: MapAttribute,
}

#[derive(Default, Clone)]
pub struct TileChanges {
    pub change: Vec<TileChangeData>,
}

#[derive(Default, Clone)]
pub struct AttrChanges {
    pub change: Vec<AttrChangeData>,
}

#[derive(Clone)]
pub enum EditorChange {
    Tile(TileChanges),
    Attr(AttrChanges),
}

pub struct EditorData {
    pub pos: Option<MapPosition>,
    pub mapdata: MapData,
    pub changed: bool,
    pub temp_saved: bool,
    pub undo: VecDeque<EditorChange>,
    pub redo: VecDeque<EditorChange>,

    pub last_pos: Vec<Vec2>,
    pub change_placeholder: ChangePlaceHolder,
    pub tile_placeholder: TileChanges,
    pub attr_placeholder: AttrChanges,
    pub unsaved_map: IndexSet<MapPosition>,
    pub exiting_save: bool,
}

impl EditorData {
    pub fn new() -> Self {
        EditorData {
            pos: None,
            mapdata: MapData::default(0, 0, 0),
            changed: false,
            temp_saved: true,
            undo: VecDeque::with_capacity(64),
            redo: VecDeque::with_capacity(64),

            last_pos: Vec::with_capacity(1024),
            change_placeholder: ChangePlaceHolder::None,
            tile_placeholder: TileChanges::default(),
            attr_placeholder: AttrChanges::default(),
            unsaved_map: IndexSet::default(),
            exiting_save: false,
        }
    }

    pub fn record_placeholder(&mut self) {
        match self.change_placeholder {
            ChangePlaceHolder::None => return,
            ChangePlaceHolder::Attributes => {
                self.undo
                    .push_back(EditorChange::Attr(self.attr_placeholder.clone()));
            }
            ChangePlaceHolder::Tiles => {
                self.undo
                    .push_back(EditorChange::Tile(self.tile_placeholder.clone()));
            }
        }

        if self.undo.len() > MAX_CHANGES {
            let _ = self.undo.pop_front();
        }

        self.last_pos.clear();
        self.redo.clear();
        self.change_placeholder = ChangePlaceHolder::None;
        self.tile_placeholder.change.clear();
        self.attr_placeholder.change.clear();
    }

    pub fn record_tile(&mut self, x: u16, y: u16, layer: usize, from_id: usize, to_id: usize) {
        let new_pos = Vec2::new(x as f32, y as f32);
        if self.last_pos.contains(&new_pos) {
            return;
        }
        self.last_pos.push(new_pos);

        self.change_placeholder = ChangePlaceHolder::Tiles;
        if !self.attr_placeholder.change.is_empty() {
            self.attr_placeholder.change.clear();
        }

        self.tile_placeholder.change.push(TileChangeData {
            x,
            y,
            layer,
            from: from_id,
            to: to_id,
        });
    }

    pub fn record_attr(&mut self, x: u16, y: u16, from: MapAttribute, to: MapAttribute) {
        let new_pos = Vec2::new(x as f32, y as f32);
        if self.last_pos.contains(&new_pos) {
            return;
        }
        self.last_pos.push(new_pos);

        self.change_placeholder = ChangePlaceHolder::Attributes;
        if !self.tile_placeholder.change.is_empty() {
            self.tile_placeholder.change.clear();
        }

        self.attr_placeholder
            .change
            .push(AttrChangeData { x, y, from, to });
    }
}

pub fn apply_undo(content: &mut Content, systems: &mut SystemHolder) {
    if content.data.undo.is_empty() {
        return;
    }

    if let Some(data) = content.data.undo.pop_back() {
        content.data.redo.push_back(data.clone());

        match data {
            EditorChange::Attr(attr) => {
                for changes in attr.change.iter() {
                    let tile_pos = get_tile_pos(changes.x as i32, changes.y as i32);
                    let (color, text) = get_attribute_visual(&changes.from);
                    content.data.mapdata.attribute[tile_pos] = changes.from.clone();

                    let view_attr = content.map_view.attribute[tile_pos];
                    systems
                        .gfx
                        .set_text(&mut systems.renderer, &view_attr.text, &text);
                    systems.gfx.center_text(&view_attr.text);
                    systems.gfx.set_color(&view_attr.bg, color);
                }
            }
            EditorChange::Tile(tile) => {
                for changes in tile.change.iter() {
                    content.map_view.map.set_tile(
                        UVec3::new(changes.x as u32, changes.y as u32, changes.layer as u32),
                        TileData {
                            id: changes.from,
                            color: Color::rgb(255, 255, 255),
                            anim_time: 250,
                        },
                    );

                    let tile_pos = get_tile_pos(changes.x as i32, changes.y as i32);
                    content.data.mapdata.tile[changes.layer].id[tile_pos] = changes.from as u32;
                }
            }
        }

        content.data.changed = true;
        content.data.temp_saved = false;
        if let Some(map_pos) = content.data.pos {
            content
                .interface
                .footer
                .set_map_pos(systems, map_pos, false);
        }
    }
}

pub fn apply_redo(content: &mut Content, systems: &mut SystemHolder) {
    if content.data.redo.is_empty() {
        return;
    }

    if let Some(data) = content.data.redo.pop_back() {
        content.data.undo.push_back(data.clone());

        match data {
            EditorChange::Attr(attr) => {
                for changes in attr.change.iter() {
                    let tile_pos = get_tile_pos(changes.x as i32, changes.y as i32);
                    let (color, text) = get_attribute_visual(&changes.to);
                    content.data.mapdata.attribute[tile_pos] = changes.to.clone();

                    let view_attr = content.map_view.attribute[tile_pos];
                    systems
                        .gfx
                        .set_text(&mut systems.renderer, &view_attr.text, &text);
                    systems.gfx.center_text(&view_attr.text);
                    systems.gfx.set_color(&view_attr.bg, color);
                }
            }
            EditorChange::Tile(tile) => {
                for changes in tile.change.iter() {
                    content.map_view.map.set_tile(
                        UVec3::new(changes.x as u32, changes.y as u32, changes.layer as u32),
                        TileData {
                            id: changes.to,
                            color: Color::rgb(255, 255, 255),
                            anim_time: 250,
                        },
                    );

                    let tile_pos = get_tile_pos(changes.x as i32, changes.y as i32);
                    content.data.mapdata.tile[changes.layer].id[tile_pos] = changes.to as u32;
                }
            }
        }

        content.data.changed = true;
        content.data.temp_saved = false;
        if let Some(map_pos) = content.data.pos {
            content
                .interface
                .footer
                .set_map_pos(systems, map_pos, false);
        }
    }
}
