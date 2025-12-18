use bit_op::{BitOp, bit_u8::*};
use graphics::*;

use crate::{
    content::{
        Content, place_autotile, switch_attributes, switch_tab,
        widget::{in_layer_area, in_view_screen},
    },
    data_types::*,
    database::{
        EditorMapAttribute, ItemSpawnData, MapAttribute, MapData, MapPosition, PresetTypeList,
        WarpData, delete_temp_map_file, is_temp_map_exist, load_map_file, save_temp_file,
    },
    renderer::SystemHolder,
    resource::TilePos,
};

pub fn get_tile_pos(x: i32, y: i32) -> usize {
    (x + (y * 32_i32)) as usize
}

pub fn get_link_map_pos(map_pos: MapPosition, id: usize) -> MapPosition {
    match id {
        1 => MapPosition {
            x: map_pos.x,
            y: map_pos.y + 1,
            group: map_pos.group,
        }, // Top
        2 => MapPosition {
            x: map_pos.x + 1,
            y: map_pos.y + 1,
            group: map_pos.group,
        }, // Top Right
        3 => MapPosition {
            x: map_pos.x - 1,
            y: map_pos.y,
            group: map_pos.group,
        }, // Left
        4 => MapPosition {
            x: map_pos.x + 1,
            y: map_pos.y,
            group: map_pos.group,
        }, // Right
        5 => MapPosition {
            x: map_pos.x - 1,
            y: map_pos.y - 1,
            group: map_pos.group,
        }, // Down Left
        6 => MapPosition {
            x: map_pos.x,
            y: map_pos.y - 1,
            group: map_pos.group,
        }, // Down
        7 => MapPosition {
            x: map_pos.x + 1,
            y: map_pos.y - 1,
            group: map_pos.group,
        }, // Down Right
        _ => MapPosition {
            x: map_pos.x - 1,
            y: map_pos.y + 1,
            group: map_pos.group,
        }, // Top Left
    }
}

pub fn apply_link_map(content: &mut Content, map_pos: MapPosition) {
    for (id, map) in content.map_view.linked_map.iter_mut().enumerate() {
        let check_pos = get_link_map_pos(map_pos, id);

        if let Ok(mapdata) = load_map_file(check_pos.x, check_pos.y, check_pos.group as u64, false)
        {
            (0..32).for_each(|x| {
                (0..32).for_each(|y| {
                    let tile_num = get_tile_pos(x, y);

                    (0..9).for_each(|i| {
                        let id = mapdata.tile[i].id[tile_num] as usize;

                        map.map.set_tile(
                            UVec3::new(x as u32, y as u32, i as u32),
                            if id > 0 {
                                TileData {
                                    id,
                                    color: Color::rgba(255, 255, 255, 255),
                                    anim_time: 250,
                                }
                            } else {
                                TileData::default()
                            },
                        );
                    });
                })
            })
        }
    }
}

pub fn apply_map_data(content: &mut Content, systems: &mut SystemHolder, mapdata: &MapData) {
    let tile_size = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    let attr_zoom_pos = Vec2::new(content.map_view.map.pos.x, content.map_view.map.pos.y);

    (0..32).for_each(|x| {
        (0..32).for_each(|y| {
            let tile_num = get_tile_pos(x, y);

            (0..9).for_each(|i| {
                let id = mapdata.tile[i].id[tile_num] as usize;

                content.map_view.map.set_tile(
                    UVec3::new(x as u32, y as u32, i as u32),
                    if id > 0 {
                        TileData {
                            id,
                            color: Color::rgba(255, 255, 255, 255),
                            anim_time: 250,
                        }
                    } else {
                        TileData::default()
                    },
                );
            });

            let (color, text) = get_attribute_visual(&mapdata.attribute[tile_num]);

            let pos = Vec2::new((tile_num % 32) as f32, (tile_num / 32) as f32);
            let tile_pos = Vec2::new(
                attr_zoom_pos.x + (tile_size.x * pos.x),
                attr_zoom_pos.y + (tile_size.y * pos.y),
            );

            let view_attr = content.map_view.attribute[tile_num];
            systems
                .gfx
                .set_text(&mut systems.renderer, &view_attr.text, &text);
            systems.gfx.set_color(&view_attr.bg, color);

            systems.gfx.set_pos(
                &view_attr.bg,
                Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG),
            );

            let text_size = Vec2::new(tile_size.x, 20.0);
            let text_pos = Vec2::new(
                tile_pos.x,
                tile_pos.y + ((tile_size.y - text_size.y) * 0.5).floor(),
            );
            systems.gfx.set_pos(
                &view_attr.text,
                Vec3::new(text_pos.x, text_pos.y, ORDER_TILE_BG),
            );
            systems.gfx.set_bound(
                &view_attr.text,
                Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                ),
            );
            systems.gfx.center_text(&view_attr.text);

            let dirblock = mapdata.dir_block[tile_num];
            let dirblock_uv = get_dirblock_uv(dirblock);

            let view_dirblock = content.map_view.dir_block[tile_num];
            systems.gfx.set_uv(
                &view_dirblock,
                Vec4::new(
                    20.0 * dirblock_uv.0 as f32,
                    20.0 * dirblock_uv.1 as f32,
                    20.0,
                    20.0,
                ),
            );
            systems.gfx.set_pos(
                &view_dirblock,
                Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG),
            );
        });
    });

    let cur_zone = content.interface.side_window.zone.cur_zone;

    let zone_data = mapdata.zones[cur_zone];

    content.interface.side_window.zone.textbox[0].set_text(systems, format!("{}", zone_data.0));

    for i in 0..5 {
        content.interface.side_window.zone.textbox[i + 1].set_text(
            systems,
            if let Some(data) = zone_data.1[i] {
                format!("{data}")
            } else {
                String::new()
            },
        );
    }

    for gfx in content.map_view.zones.iter() {
        systems.gfx.set_color(gfx, Color::rgba(0, 0, 0, 0));
    }

    for zones in mapdata.zonespawns[cur_zone].iter() {
        let tile_num = get_tile_pos(zones.0 as i32, zones.1 as i32);
        let gfx = content.map_view.zones[tile_num];

        let pos = Vec2::new((tile_num % 32) as f32, (tile_num / 32) as f32);
        let tile_pos = Vec2::new(
            attr_zoom_pos.x + (tile_size.x * pos.x),
            attr_zoom_pos.y + (tile_size.y * pos.y),
        );

        systems
            .gfx
            .set_pos(&gfx, Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG));
        systems.gfx.set_color(&gfx, Color::rgba(0, 0, 100, 150));
    }

    content
        .interface
        .side_window
        .weather
        .weather_list
        .list
        .set_select(systems, Some(mapdata.weather as usize), true);
    content
        .interface
        .side_window
        .weather
        .weather_list
        .update_label(systems, mapdata.weather as usize);

    let music_index = content
        .audio_collection
        .audio
        .iter()
        .position(|data| Some(data) == mapdata.music.as_ref())
        .unwrap_or(0);

    content
        .interface
        .side_window
        .music
        .music_list
        .set_select(systems, Some(music_index), true);
}

pub fn update_zone_visible(content: &mut Content, systems: &mut SystemHolder) {
    let tile_size = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    let attr_zoom_pos = Vec2::new(content.map_view.map.pos.x, content.map_view.map.pos.y);

    let cur_zone = content.interface.side_window.zone.cur_zone;

    for gfx in content.map_view.zones.iter() {
        systems.gfx.set_color(gfx, Color::rgba(0, 0, 0, 0));
    }

    for zones in content.data.mapdata.zonespawns[cur_zone].iter() {
        let tile_num = get_tile_pos(zones.0 as i32, zones.1 as i32);
        let gfx = content.map_view.zones[tile_num];

        let pos = Vec2::new((tile_num % 32) as f32, (tile_num / 32) as f32);
        let tile_pos = Vec2::new(
            attr_zoom_pos.x + (tile_size.x * pos.x),
            attr_zoom_pos.y + (tile_size.y * pos.y),
        );

        systems
            .gfx
            .set_pos(&gfx, Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG));
        systems.gfx.set_color(&gfx, Color::rgba(0, 0, 100, 150));
    }
}

pub fn update_map_tile(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let start_pos = content.interface.side_window.tilesets.selection.start_pos;
    let end_pos = content.interface.side_window.tilesets.selection.end_pos;

    let map_pos = content.map_view.tile.cur_pos;
    let cur_layer = match content.interface.tool.cur_layer {
        1..=4 => content.interface.tool.cur_layer + 2,
        5 | 6 => content.interface.tool.cur_layer - 4,
        _ => content.interface.tool.cur_layer,
    } as u32;

    let pos = Vec2::new(start_pos.x.min(end_pos.x), start_pos.y.min(end_pos.y));

    let tilesheet_pos = Vec2::new(pos.x, TILESET_COUNT_Y.saturating_sub(1) as f32 - pos.y);
    let tilesheet_size = if set {
        Vec2::new(
            (start_pos.x - end_pos.x).abs() + 1.0,
            (start_pos.y - end_pos.y).abs() + 1.0,
        )
    } else {
        Vec2::new(1.0, 1.0)
    };
    let (tile_size_x, tile_size_y) = (tilesheet_size.x as usize, tilesheet_size.y as usize);

    for x in 0..tile_size_x {
        for y in 0..tile_size_y {
            let tile_id = if set {
                systems
                    .resource
                    .tile_pos_loc
                    .get(&TilePos {
                        x: (tilesheet_pos.x as u32 + x as u32) * TEXTURE_SIZE,
                        y: (tilesheet_pos.y as u32 - y as u32) * TEXTURE_SIZE,
                        file: content.interface.side_window.tilesets.cur_tileset as u32,
                    })
                    .copied()
            } else {
                None
            };

            if let Some(id) = tile_id {
                let pos = UVec3::new(
                    map_pos.x as u32 + x as u32,
                    map_pos.y as u32 + y as u32,
                    cur_layer,
                );

                let cur_id = content.map_view.map.get_tile(pos);
                content.data.record_tile(
                    pos.x as u16,
                    pos.y as u16,
                    cur_layer as usize,
                    cur_id.id,
                    id,
                );

                content.map_view.map.set_tile(
                    pos,
                    TileData {
                        id,
                        color: Color::rgba(255, 255, 255, 255),
                        anim_time: 250,
                    },
                );

                let tile_pos = get_tile_pos(pos.x as i32, pos.y as i32);
                content.data.mapdata.tile[cur_layer as usize].id[tile_pos] = id as u32;
            } else {
                let pos = UVec3::new(map_pos.x as u32, map_pos.y as u32, cur_layer);

                let cur_id = content.map_view.map.get_tile(pos);
                content.data.record_tile(
                    pos.x as u16,
                    pos.y as u16,
                    cur_layer as usize,
                    cur_id.id,
                    0,
                );

                content.map_view.map.set_tile(pos, TileData::default());

                let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);
                content.data.mapdata.tile[cur_layer as usize].id[tile_pos] = 0;
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

pub fn update_map_attribute(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let map_pos = content.map_view.tile.cur_pos;
    let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);

    let e_attribute = content.interface.side_window.attributes.cur_attribute;
    let attribute = {
        if set {
            let gui = &content.interface.side_window.attributes;

            match e_attribute {
                EditorMapAttribute::Blocked => MapAttribute::Blocked,
                EditorMapAttribute::ItemSpawn => {
                    MapAttribute::ItemSpawn(gui.attr_itemspawn.get_value())
                }
                EditorMapAttribute::NpcBlocked => MapAttribute::NpcBlocked,
                EditorMapAttribute::Shop => MapAttribute::Shop(gui.attr_index.get_value::<u16>()),
                EditorMapAttribute::Sign => MapAttribute::Sign(gui.attr_sign.get_value()),
                EditorMapAttribute::Storage => MapAttribute::Storage,
                EditorMapAttribute::Warp => MapAttribute::Warp({
                    let pos = gui.attr_position.get_value();

                    WarpData {
                        map_x: pos.map.x,
                        map_y: pos.map.y,
                        map_group: pos.map.group as u64,
                        tile_x: pos.x as u32,
                        tile_y: pos.y as u32,
                    }
                }),
                EditorMapAttribute::Walkable | EditorMapAttribute::Count => MapAttribute::Walkable,
            }
        } else {
            MapAttribute::Walkable
        }
    };

    let (color, text) = get_attribute_visual(&attribute);

    let cur_attr = content.data.mapdata.attribute[tile_pos].clone();
    content.data.record_attr(
        map_pos.x as u16,
        map_pos.y as u16,
        cur_attr,
        attribute.clone(),
    );

    content.data.mapdata.attribute[tile_pos] = attribute;

    {
        let view_attr = content.map_view.attribute[tile_pos];
        systems
            .gfx
            .set_text(&mut systems.renderer, &view_attr.text, &text);
        systems.gfx.center_text(&view_attr.text);
        systems.gfx.set_color(&view_attr.bg, color);
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

pub fn update_map_zone(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let map_pos = content.map_view.tile.cur_pos;
    let cur_zone = content.interface.side_window.zone.cur_zone;
    let tile_num = get_tile_pos(map_pos.x as i32, map_pos.y as i32);
    let data = (map_pos.x as u16, map_pos.y as u16);

    if set {
        if content.data.mapdata.zonespawns[cur_zone].contains(&data) {
            return;
        }
        content.data.mapdata.zonespawns[cur_zone].push(data);
    } else if let Some(index) = content.data.mapdata.zonespawns[cur_zone]
        .iter()
        .position(|check| *check == data)
    {
        content.data.mapdata.zonespawns[cur_zone].remove(index);
    } else {
        return;
    }

    let tile_size = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
    let attr_zoom_pos = Vec2::new(content.map_view.map.pos.x, content.map_view.map.pos.y);

    let pos = Vec2::new((tile_num % 32) as f32, (tile_num / 32) as f32);
    let tile_pos = Vec2::new(
        attr_zoom_pos.x + (tile_size.x * pos.x),
        attr_zoom_pos.y + (tile_size.y * pos.y),
    );

    let gfx = content.map_view.zones[tile_num];

    systems
        .gfx
        .set_pos(&gfx, Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG));
    systems.gfx.set_color(
        &gfx,
        Color::rgba(0, 0, if set { 100 } else { 0 }, if set { 150 } else { 0 }),
    );

    content.data.changed = true;
    content.data.temp_saved = false;
    if let Some(map_pos) = content.data.pos {
        content
            .interface
            .footer
            .set_map_pos(systems, map_pos, false);
    }
}

pub fn update_map_dirblock(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let map_pos = content.map_view.tile.cur_pos;
    let tile_num = get_tile_pos(map_pos.x as i32, map_pos.y as i32);

    let mut dirblock = 0;
    if set {
        if content.interface.side_window.dirblocks.blocks[1].value {
            dirblock.set(B1);
        }
        if content.interface.side_window.dirblocks.blocks[2].value {
            dirblock.set(B2);
        }
        if content.interface.side_window.dirblocks.blocks[0].value {
            dirblock.set(B0);
        }
        if content.interface.side_window.dirblocks.blocks[3].value {
            dirblock.set(B3);
        }
    }

    content.data.mapdata.dir_block[tile_num] = dirblock;

    let dirblock_uv = get_dirblock_uv(dirblock);

    let view_dirblock = content.map_view.dir_block[tile_num];
    systems.gfx.set_uv(
        &view_dirblock,
        Vec4::new(
            20.0 * dirblock_uv.0 as f32,
            20.0 * dirblock_uv.1 as f32,
            20.0,
            20.0,
        ),
    );

    content.data.changed = true;
    content.data.temp_saved = false;
    if let Some(map_pos) = content.data.pos {
        content
            .interface
            .footer
            .set_map_pos(systems, map_pos, false);
    }
}

pub fn set_preset(content: &mut Content, systems: &mut SystemHolder) {
    let cur_preset = content.interface.side_window.presets.selected_index;
    let draw_type = content.preset.data[cur_preset].draw_type;
    let animated = matches!(
        draw_type,
        PresetTypeList::Animated | PresetTypeList::AutotileAnimated
    );
    let map_pos = content.map_view.tile.cur_pos;
    let cur_layer = match content.interface.tool.cur_layer {
        1..=4 => content.interface.tool.cur_layer + 2,
        5 | 6 => content.interface.tool.cur_layer - 4,
        _ => content.interface.tool.cur_layer,
    } as u32;
    let frames = content.preset.data[cur_preset].frames;

    match draw_type {
        PresetTypeList::Normal | PresetTypeList::Animated => {
            for (i, frame) in frames.iter().enumerate() {
                let set_layer = if animated {
                    MapLayers::Anim1 as usize + i
                } else {
                    if i > 0 {
                        break;
                    }
                    cur_layer as usize
                };

                let start_pos = Vec2::new(frame.start.x as f32, frame.start.y as f32);
                let end_pos = Vec2::new(frame.end.x as f32, frame.end.y as f32);

                let tilesheet_pos = Vec2::new(
                    start_pos.x.min(end_pos.x),
                    TILESET_COUNT_Y.saturating_sub(1) as f32 - start_pos.y.min(end_pos.y),
                );
                let tilesheet_size = Vec2::new(
                    (start_pos.x - end_pos.x).abs() + 1.0,
                    (start_pos.y - end_pos.y).abs() + 1.0,
                );
                let (tile_size_x, tile_size_y) =
                    (tilesheet_size.x as usize, tilesheet_size.y as usize);

                for x in 0..tile_size_x {
                    for y in 0..tile_size_y {
                        let tile_id = systems
                            .resource
                            .tile_pos_loc
                            .get(&TilePos {
                                x: (tilesheet_pos.x as u32 + x as u32) * TEXTURE_SIZE,
                                y: (tilesheet_pos.y as u32 - y as u32) * TEXTURE_SIZE,
                                file: frame.tileset as u32,
                            })
                            .copied();

                        if let Some(id) = tile_id {
                            let pos = UVec3::new(
                                map_pos.x as u32 + x as u32,
                                map_pos.y as u32 + y as u32,
                                set_layer as u32,
                            );

                            let cur_id = content.map_view.map.get_tile(pos);
                            content.data.record_tile(
                                pos.x as u16,
                                pos.y as u16,
                                set_layer,
                                cur_id.id,
                                id,
                            );

                            content.map_view.map.set_tile(
                                pos,
                                TileData {
                                    id,
                                    color: Color::rgba(255, 255, 255, 255),
                                    anim_time: 250,
                                },
                            );

                            let tile_pos = get_tile_pos(pos.x as i32, pos.y as i32);
                            content.data.mapdata.tile[set_layer].id[tile_pos] = id as u32;
                        } else {
                            let pos =
                                UVec3::new(map_pos.x as u32, map_pos.y as u32, set_layer as u32);

                            let cur_id = content.map_view.map.get_tile(pos);
                            content.data.record_tile(
                                pos.x as u16,
                                pos.y as u16,
                                set_layer,
                                cur_id.id,
                                0,
                            );

                            content.map_view.map.set_tile(pos, TileData::default());

                            let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);
                            content.data.mapdata.tile[set_layer].id[tile_pos] = 0;
                        }
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
        PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated => {
            place_autotile(content, systems, animated, map_pos, cur_layer, frames);
        }
    }
}

pub fn update_tile_fill(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let cur_layer = match content.interface.tool.cur_layer {
        1..=4 => content.interface.tool.cur_layer + 2,
        5 | 6 => content.interface.tool.cur_layer - 4,
        _ => content.interface.tool.cur_layer,
    } as u32;

    let tile_data = if set {
        let start_pos = content.interface.side_window.tilesets.selection.start_pos;
        let end_pos = content.interface.side_window.tilesets.selection.end_pos;

        let tilesheet_pos = Vec2::new(
            start_pos.x.min(end_pos.x),
            TILESET_COUNT_Y.saturating_sub(1) as f32 - start_pos.y.min(end_pos.y),
        );

        let tile_id = if let Some(id) = systems
            .resource
            .tile_pos_loc
            .get(&TilePos {
                x: tilesheet_pos.x as u32 * TEXTURE_SIZE,
                y: tilesheet_pos.y as u32 * TEXTURE_SIZE,
                file: content.interface.side_window.tilesets.cur_tileset as u32,
            })
            .copied()
        {
            id
        } else {
            return;
        };

        TileData {
            id: tile_id,
            color: Color::rgba(255, 255, 255, 255),
            anim_time: 250,
        }
    } else {
        TileData::default()
    };

    let map_pos = content.map_view.tile.cur_pos;

    let comparedata = content
        .map_view
        .map
        .get_tile(UVec3::new(map_pos.x as u32, map_pos.y as u32, cur_layer))
        .id;
    if comparedata == tile_data.id {
        return;
    }

    let mut paint_to_map: Vec<Vec2> = Vec::with_capacity(1024);

    paint_to_map.push(map_pos);

    while let Some(pos) = paint_to_map.pop() {
        let cur_id =
            content
                .map_view
                .map
                .get_tile(UVec3::new(pos.x as u32, pos.y as u32, cur_layer));
        content.data.record_tile(
            pos.x as u16,
            pos.y as u16,
            cur_layer as usize,
            cur_id.id,
            tile_data.id,
        );

        content
            .map_view
            .map
            .set_tile(UVec3::new(pos.x as u32, pos.y as u32, cur_layer), tile_data);

        let tile_pos = get_tile_pos(pos.x as i32, pos.y as i32);
        content.data.mapdata.tile[cur_layer as usize].id[tile_pos] = tile_data.id as u32;

        for dir in 0..4 {
            let mut adjust_pos = Vec2::new(0.0, 0.0);
            match dir {
                1 => {
                    adjust_pos.y = 1.0;
                } // Up
                2 => {
                    adjust_pos.x = -1.0;
                } // Left
                3 => {
                    adjust_pos.x = 1.0;
                } // Right
                _ => {
                    adjust_pos.y = -1.0;
                } // Down
            }
            let checkpos = pos + adjust_pos;

            if checkpos.x >= 0.0 && checkpos.x < 32.0 && checkpos.y >= 0.0 && checkpos.y < 32.0 {
                let check_data = content
                    .map_view
                    .map
                    .get_tile(UVec3::new(checkpos.x as u32, checkpos.y as u32, cur_layer))
                    .id;
                if check_data == comparedata {
                    paint_to_map.push(checkpos);
                }
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

pub fn update_attribute_fill(content: &mut Content, systems: &mut SystemHolder, set: bool) {
    let map_pos = content.map_view.tile.cur_pos;
    let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);

    let e_attribute = content.interface.side_window.attributes.cur_attribute;
    let attribute = {
        if set {
            let gui = &content.interface.side_window.attributes;

            match e_attribute {
                EditorMapAttribute::Blocked => MapAttribute::Blocked,
                EditorMapAttribute::ItemSpawn => {
                    MapAttribute::ItemSpawn(gui.attr_itemspawn.get_value())
                }
                EditorMapAttribute::NpcBlocked => MapAttribute::NpcBlocked,
                EditorMapAttribute::Shop => MapAttribute::Shop(gui.attr_index.get_value::<u16>()),
                EditorMapAttribute::Sign => MapAttribute::Sign(gui.attr_sign.get_value()),
                EditorMapAttribute::Storage => MapAttribute::Storage,
                EditorMapAttribute::Warp => MapAttribute::Warp({
                    let pos = gui.attr_position.get_value();

                    WarpData {
                        map_x: pos.map.x,
                        map_y: pos.map.y,
                        map_group: pos.map.group as u64,
                        tile_x: pos.x as u32,
                        tile_y: pos.y as u32,
                    }
                }),
                EditorMapAttribute::Walkable | EditorMapAttribute::Count => MapAttribute::Walkable,
            }
        } else {
            MapAttribute::Walkable
        }
    };

    let comparedata = content.data.mapdata.attribute[tile_pos].clone();
    if comparedata == attribute {
        return;
    }

    let mut paint_to_map: Vec<Vec2> = Vec::with_capacity(1024);

    paint_to_map.push(map_pos);

    while let Some(pos) = paint_to_map.pop() {
        let new_pos = get_tile_pos(pos.x as i32, pos.y as i32);

        let cur_attr = content.data.mapdata.attribute[new_pos].clone();
        content
            .data
            .record_attr(pos.x as u16, pos.y as u16, cur_attr, attribute.clone());

        content.data.mapdata.attribute[new_pos].clone_from(&attribute);

        let (color, text) = get_attribute_visual(&attribute);
        {
            let view_attr = content.map_view.attribute[new_pos];
            systems
                .gfx
                .set_text(&mut systems.renderer, &view_attr.text, &text);
            systems.gfx.center_text(&view_attr.text);
            systems.gfx.set_color(&view_attr.bg, color);
        }

        for dir in 0..4 {
            let mut adjust_pos = Vec2::new(0.0, 0.0);
            match dir {
                1 => {
                    adjust_pos.y = 1.0;
                } // Up
                2 => {
                    adjust_pos.x = -1.0;
                } // Left
                3 => {
                    adjust_pos.x = 1.0;
                } // Right
                _ => {
                    adjust_pos.y = -1.0;
                } // Down
            }
            let checkpos = pos + adjust_pos;

            if checkpos.x >= 0.0 && checkpos.x < 32.0 && checkpos.y >= 0.0 && checkpos.y < 32.0 {
                let check_pos = get_tile_pos(checkpos.x as i32, checkpos.y as i32);
                let check_data = content.data.mapdata.attribute[check_pos].clone();
                if check_data == comparedata {
                    paint_to_map.push(checkpos);
                }
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

pub fn picker_layer_update(content: &mut Content, systems: &mut SystemHolder) {
    switch_tab(content, systems, TabButton::Tileset);

    let cur_layer = match content.interface.tool.cur_layer {
        1..=4 => content.interface.tool.cur_layer + 2,
        5 | 6 => content.interface.tool.cur_layer - 4,
        _ => content.interface.tool.cur_layer,
    } as u32;

    let map_pos = content.map_view.tile.cur_pos;
    let tile_pos = UVec3::new(map_pos.x as u32, map_pos.y as u32, cur_layer);
    let tile_id = content.map_view.map.get_tile(tile_pos).id;

    let tilesheet_pos = if let Some(tilesheet_pos) = systems.resource.tile_index_loc.get(&tile_id) {
        *tilesheet_pos
    } else {
        return;
    };

    content
        .interface
        .side_window
        .tilesets
        .change_tileset(systems, tilesheet_pos.file as usize);

    let pos = Vec2::new(
        (tilesheet_pos.x / TEXTURE_SIZE) as f32,
        TILESET_COUNT_Y.saturating_sub(1) as f32 - (tilesheet_pos.y / TEXTURE_SIZE) as f32,
    );

    content
        .interface
        .side_window
        .tilesets
        .select_tile(systems, pos, pos);
}

pub fn picker_attribute_update(content: &mut Content, systems: &mut SystemHolder) {
    let map_pos = content.map_view.tile.cur_pos;
    let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);
    let attribute = content.data.mapdata.attribute[tile_pos].clone();

    if attribute == MapAttribute::Walkable {
        return;
    }

    switch_tab(content, systems, TabButton::Attributes);
    let mapattr = attribute.to_editor();
    switch_attributes(content, systems, mapattr, None);

    let gui = &mut content.interface.side_window.attributes;

    match attribute {
        MapAttribute::ItemSpawn(data) => {
            gui.attr_itemspawn.input_box[0]
                .textbox
                .set_text(systems, format!("{}", data.index));
            gui.attr_itemspawn.input_box[1]
                .textbox
                .set_text(systems, format!("{}", data.amount));
            gui.attr_itemspawn.input_box[2]
                .textbox
                .set_text(systems, format!("{}", data.timer));
        }
        MapAttribute::Shop(data) => {
            gui.attr_index
                .input_box
                .textbox
                .set_text(systems, format!("{data}"));
        }
        MapAttribute::Sign(data) => {
            gui.attr_sign
                .input_box
                .textbox
                .set_text(systems, data.to_string());
        }
        MapAttribute::Warp(data) => {
            gui.attr_position.input_box[0]
                .textbox
                .set_text(systems, format!("{}", data.tile_x));
            gui.attr_position.input_box[1]
                .textbox
                .set_text(systems, format!("{}", data.tile_y));
            gui.attr_position.input_box[2]
                .textbox
                .set_text(systems, format!("{}", data.map_x));
            gui.attr_position.input_box[3]
                .textbox
                .set_text(systems, format!("{}", data.map_y));
            gui.attr_position.input_box[4]
                .textbox
                .set_text(systems, format!("{}", data.map_group));
        }
        MapAttribute::Storage
        | MapAttribute::NpcBlocked
        | MapAttribute::Walkable
        | MapAttribute::Blocked
        | MapAttribute::Count => {}
    }
}

pub fn load_and_apply_map(
    systems: &mut SystemHolder,
    content: &mut Content,
    mappos: MapPosition,
    seconds: f32,
) -> Result<bool> {
    if let Ok(mapdata) = load_map_file(mappos.x, mappos.y, mappos.group as u64, true) {
        if content.data.changed && !content.data.temp_saved {
            if let Some(mappos) = content.data.pos {
                save_temp_file(
                    mappos.x,
                    mappos.y,
                    mappos.group.try_into().unwrap(),
                    &content.data.mapdata,
                    true,
                )?;

                content.interface.notification.add_msg(
                    systems,
                    format!(
                        "Temp Map [X: {} Y: {} Group: {}] Saved!",
                        mappos.x, mappos.y, mappos.group
                    ),
                    seconds,
                );

                let _ = content.data.unsaved_map.insert(mappos);
            } else {
                save_temp_file(0, 0, 0, &content.data.mapdata, false)?;

                content.interface.notification.add_msg(
                    systems,
                    "Temp recovery map file saved!".to_string(),
                    seconds,
                );
            }
        }

        apply_map_data(content, systems, &mapdata);
        apply_link_map(content, mappos);
        content.data.mapdata = mapdata;
        content.data.pos = Some(mappos);
        content.data.changed = false;
        content.data.temp_saved = true;
        content.interface.footer.set_map_pos(systems, mappos, true);

        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn save_map_change(content: &mut Content, mappos: MapPosition) -> Result<bool> {
    content.data.mapdata.position = mappos;

    let _ = content.data.unsaved_map.swap_remove(&mappos);

    match content.data.mapdata.save_file() {
        Ok(()) => {
            if is_temp_map_exist(mappos.x, mappos.y, mappos.group as u64) {
                delete_temp_map_file(mappos.x, mappos.y, mappos.group as u64)?;
            }

            content.data.pos = Some(mappos);
            content.data.changed = false;
            content.data.temp_saved = true;

            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

pub fn get_dirblock_uv(dirblock: u8) -> (u16, u16) {
    let blocked = [
        dirblock.get(B1) == 0b00000010, // Up
        dirblock.get(B2) == 0b00000100, // Left
        dirblock.get(B0) == 0b00000001, // Down
        dirblock.get(B3) == 0b00001000, // Right
    ];

    match blocked {
        // 1-direction
        [true, false, false, false] => (1, 0),
        [false, true, false, false] => (3, 0),
        [false, false, true, false] => (2, 0),
        [false, false, false, true] => (4, 0),

        // 2-direction
        [true, true, false, false] => (3, 1),
        [true, false, true, false] => (0, 2),
        [true, false, false, true] => (0, 1),
        [false, true, true, false] => (2, 1),
        [false, true, false, true] => (4, 1),
        [false, false, true, true] => (1, 1),

        // 3-direction
        [true, true, true, false] => (2, 2),
        [true, true, false, true] => (3, 2),
        [true, false, true, true] => (4, 2),
        [false, true, true, true] => (1, 2),

        // 4-direction
        [true, true, true, true] => (0, 3),

        _ => (0, 0), // All false
    }
}

pub fn get_attribute_visual(attribute: &MapAttribute) -> (Color, String) {
    match attribute {
        MapAttribute::Blocked => (Color::rgba(255, 0, 0, 120), "B".to_string()),
        MapAttribute::ItemSpawn(_) => (Color::rgba(0, 0, 0, 120), "I".to_string()),
        MapAttribute::NpcBlocked => (Color::rgba(0, 0, 0, 120), "N".to_string()),
        MapAttribute::Shop(_) => (Color::rgba(0, 0, 0, 120), "S".to_string()),
        MapAttribute::Sign(_) => (Color::rgba(0, 0, 0, 120), "S".to_string()),
        MapAttribute::Storage => (Color::rgba(0, 0, 0, 120), "S".to_string()),
        MapAttribute::Warp(_) => (Color::rgba(0, 0, 0, 120), "W".to_string()),
        MapAttribute::Walkable | MapAttribute::Count => (Color::rgba(0, 0, 0, 0), String::new()),
    }
}
