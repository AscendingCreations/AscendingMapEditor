use graphics::*;

use crate::{
    content::{Content, get_tile_pos},
    data_types::{TEXTURE_SIZE, TILESET_COUNT_Y},
    database::{PresetFrames, PresetTypeList},
    renderer::SystemHolder,
    resource::TilePos,
};

pub const AUTO_UL: (u32, u32) = (0, 2); // Up Left
pub const AUTO_U: (u32, u32) = (1, 2); // Up
pub const AUTO_UR: (u32, u32) = (2, 2); // Up Right
pub const AUTO_L: (u32, u32) = (0, 1); // Left
pub const AUTO_C: (u32, u32) = (1, 1); // Center
pub const AUTO_R: (u32, u32) = (2, 1); // Right
pub const AUTO_DL: (u32, u32) = (0, 0); // Down Left
pub const AUTO_D: (u32, u32) = (1, 0); // Down
pub const AUTO_DR: (u32, u32) = (2, 0); // Down Right
pub const AUTO_CUL: (u32, u32) = (3, 2); // Corner Up Left
pub const AUTO_CUR: (u32, u32) = (4, 2); // Corner Up Right
pub const AUTO_CDL: (u32, u32) = (3, 1); // Corner Down Left
pub const AUTO_CDR: (u32, u32) = (4, 1); // Corner Down Right

pub fn check_match_direction(content: &Content, map_pos: Vec2, layer: usize) -> [bool; 8] {
    let pos = (map_pos.x as u32, map_pos.y as u32);

    let top = if pos.1 < 31 {
        is_tile_same(content, Vec2::new(pos.0 as f32, (pos.1 + 1) as f32), layer)
    } else {
        true
    };
    let left = if pos.0 > 0 {
        is_tile_same(content, Vec2::new((pos.0 - 1) as f32, pos.1 as f32), layer)
    } else {
        true
    };
    let right = if pos.0 < 31 {
        is_tile_same(content, Vec2::new((pos.0 + 1) as f32, pos.1 as f32), layer)
    } else {
        true
    };
    let down = if pos.1 > 0 {
        is_tile_same(content, Vec2::new(pos.0 as f32, (pos.1 - 1) as f32), layer)
    } else {
        true
    };
    let top_right = if pos.0 < 31 && pos.1 < 31 {
        is_tile_same(
            content,
            Vec2::new((pos.0 + 1) as f32, (pos.1 + 1) as f32),
            layer,
        )
    } else {
        true
    };
    let top_left = if pos.0 > 0 && pos.1 < 31 {
        is_tile_same(
            content,
            Vec2::new((pos.0 - 1) as f32, (pos.1 + 1) as f32),
            layer,
        )
    } else {
        true
    };
    let down_left = if pos.0 > 0 && pos.1 > 0 {
        is_tile_same(
            content,
            Vec2::new((pos.0 - 1) as f32, (pos.1 - 1) as f32),
            layer,
        )
    } else {
        true
    };
    let down_right = if pos.0 < 31 && pos.1 > 0 {
        is_tile_same(
            content,
            Vec2::new((pos.0 + 1) as f32, (pos.1 - 1) as f32),
            layer,
        )
    } else {
        true
    };

    [
        top_left, top, top_right, left, right, down_left, down, down_right,
    ]
}

pub fn is_tile_same(content: &Content, map_pos: Vec2, layer: usize) -> bool {
    let tile = content
        .map_view
        .map
        .get_tile(UVec3::new(map_pos.x as u32, map_pos.y as u32, layer as u32))
        .id;
    content.preset.selected_preset_tiles.contains(&tile)
}

pub fn place_autotile(
    content: &mut Content,
    systems: &mut SystemHolder,
    animated: bool,
    map_pos: Vec2,
    cur_layer: u32,
    frames: [PresetFrames; 4],
) {
    let mut check_pos = Vec::with_capacity(9);
    check_pos.push(map_pos);
    for x in -1..=1 {
        for y in -1..=1 {
            let newpos = map_pos + Vec2::new(x as f32, y as f32);
            if newpos == map_pos
                || newpos.x < 0.0
                || newpos.y < 0.0
                || newpos.x > 31.0
                || newpos.y > 31.0
            {
                continue;
            }
            check_pos.push(newpos);
        }
    }

    for (i, frame) in frames.iter().enumerate() {
        let set_layer = if animated {
            MapLayers::Anim1 as usize + i
        } else {
            if i > 0 {
                break;
            }
            cur_layer as usize
        };

        for &pos in check_pos.iter() {
            if pos != map_pos && !is_tile_same(content, pos, set_layer) {
                continue;
            }

            let start_pos = Vec2::new(frame.start.x as f32, frame.start.y as f32);
            let end_pos = Vec2::new(frame.end.x as f32, frame.end.y as f32);
            let tilesheet_pos = Vec2::new(
                start_pos.x.min(end_pos.x),
                TILESET_COUNT_Y.saturating_sub(1) as f32 - start_pos.y.min(end_pos.y),
            );

            let key = check_match_direction(content, pos, set_layer);
            // TL, T, TR, L, R, DL, D, DR
            let sheet_pos = match key {
                [_, false, _, false, true, _, true, _] => AUTO_UL,
                [_, false, _, true, true, _, true, _] => AUTO_U,
                [_, false, _, true, false, _, true, _] => AUTO_UR,
                [_, true, _, false, true, _, true, _] => AUTO_L,
                [_, true, _, true, false, _, true, _] => AUTO_R,
                [_, true, _, false, true, _, false, _] => AUTO_DL,
                [_, true, _, true, true, _, false, _] => AUTO_D,
                [_, true, _, true, false, _, false, _] => AUTO_DR,
                [true, true, _, true, true, _, true, false] => AUTO_CUL,
                [_, true, true, true, true, false, true, _] => AUTO_CUR,
                [_, true, false, true, true, true, true, _] => AUTO_CDL,
                [false, true, _, true, true, _, true, true] => AUTO_CDR,
                _ => AUTO_C,
            };

            let tile_id = systems
                .resource
                .tile_pos_loc
                .get(&TilePos {
                    x: (tilesheet_pos.x as u32 + sheet_pos.0) * TEXTURE_SIZE,
                    y: (tilesheet_pos.y as u32 - sheet_pos.1) * TEXTURE_SIZE,
                    file: frame.tileset as u32,
                })
                .copied();

            if let Some(id) = tile_id {
                let t_pos = UVec3::new(pos.x as u32, pos.y as u32, set_layer as u32);

                let cur_id = content.map_view.map.get_tile(t_pos);
                content
                    .data
                    .record_tile(t_pos.x as u16, t_pos.y as u16, set_layer, cur_id.id, id);

                content.map_view.map.set_tile(
                    t_pos,
                    TileData {
                        id,
                        color: Color::rgba(255, 255, 255, 255),
                        anim_time: 250,
                    },
                );

                let tile_pos = get_tile_pos(t_pos.x as i32, t_pos.y as i32);
                content.data.mapdata.tile[set_layer].id[tile_pos] = id as u32;
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
