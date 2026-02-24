use camera::controls::{FlatControls, FlatSettings};
use graphics::*;
use input::{Key, Named};
use winit::event_loop::ActiveEventLoop;

use crate::{
    content::{
        Content, MapPosInputType, apply_redo, apply_undo, get_link_map_pos, get_tile_pos,
        interface_input, load_and_apply_map, map_view, picker_attribute_update,
        picker_layer_update, save_map_change, set_preset, update_attribute_fill,
        update_map_attribute, update_map_dirblock, update_map_tile, update_map_zone,
        update_tile_fill,
        widget::{
            Alert, AlertBuilder, AlertIndex, Tooltip, in_drawing_area, in_layer_area,
            in_view_screen,
        },
    },
    data_types::{MouseInputType, Result, SelectedTextbox, TEXTURE_SIZE, TabButton, ToolType},
    database::{EditorMapAttribute, is_temp_map_exist},
    renderer::{Graphics, SystemHolder},
};

#[allow(clippy::too_many_arguments)]
pub fn handle_input(
    systems: &mut SystemHolder,
    graphics: &mut Graphics<FlatControls>,
    inputtype: MouseInputType,
    screen_pos: Vec2,
    content: &mut Content,
    tooltip: &mut Tooltip,
    alert: &mut Alert,
    elwt: &ActiveEventLoop,
    seconds: f32,
) -> Result<()> {
    // We convert the mouse position to render position as the y pos increase upward
    let mouse_pos = Vec2::new(screen_pos.x, systems.size.height - screen_pos.y);

    if interface_input(
        systems, graphics, inputtype, mouse_pos, content, tooltip, alert, elwt, seconds,
    )? {
        content.map_view.last_camera_pos = content.map_view.camera_pos;
        return Ok(());
    }

    if in_view_screen(systems, mouse_pos) && !in_layer_area(systems, mouse_pos) {
        let cur_tool = content.interface.tool.cur_tool;
        let cur_tab = content.interface.side_window.cur_tab;

        if cur_tab == TabButton::Attributes {
            content.map_view.attr_preview.clear_attr(systems, seconds);

            match inputtype {
                MouseInputType::LeftDown
                | MouseInputType::MiddleDown
                | MouseInputType::RightDown => {
                    content.map_view.attr_preview.in_drag = true;
                }
                MouseInputType::Move => {
                    let start_pos = (content.map_view.map.pos * systems.config.zoom).round()
                        + content.map_view.camera_pos;
                    let selecting_pos = mouse_pos - start_pos;
                    let tile_size = (TEXTURE_SIZE as f32 * systems.config.zoom).round();
                    let map_pos = Vec2::new(
                        (selecting_pos.x / tile_size).floor().min(31.0),
                        (selecting_pos.y / tile_size).floor().min(31.0),
                    );

                    let tile_pos = get_tile_pos(map_pos.x as i32, map_pos.y as i32);
                    if let Some(attr) = content.data.mapdata.attribute.get(tile_pos) {
                        content
                            .map_view
                            .attr_preview
                            .show_attr(systems, mouse_pos, attr, seconds);
                    }
                }
                MouseInputType::Release => {
                    content.map_view.attr_preview.in_drag = false;
                    content.map_view.attr_preview.update_pos(systems, mouse_pos);
                }
                _ => {}
            }
        }

        match inputtype {
            MouseInputType::LeftDown => {
                if cur_tool == ToolType::Move {
                    content.map_view.set_map_drag(mouse_pos)
                }
            }
            MouseInputType::LeftDownMove => {
                if cur_tool == ToolType::Move {
                    content.map_view.last_camera_pos =
                        content.map_view.update_map_drag(graphics, mouse_pos);
                }
            }
            MouseInputType::Move => {
                let old_link_map = content.map_view.hover_linked_map;

                if let Some(link_index) = content.map_view.in_linked_area(systems, mouse_pos) {
                    if Some(link_index) != old_link_map {
                        content.map_view.hover_linked_map = Some(link_index);
                        systems.gfx.set_color(
                            &content.map_view.linked_map[link_index].bg,
                            Color::rgba(40, 40, 40, 150),
                        );
                    }
                } else {
                    content.map_view.hover_linked_map = None;
                }

                if old_link_map != content.map_view.hover_linked_map
                    && let Some(index) = old_link_map
                {
                    systems.gfx.set_color(
                        &content.map_view.linked_map[index].bg,
                        Color::rgba(0, 0, 0, 150),
                    );
                }
            }
            MouseInputType::DoubleLeftDown => {
                if let Some(link_index) = content.map_view.in_linked_area(systems, mouse_pos)
                    && let Some(check_pos) = content.data.pos
                {
                    let mappos = get_link_map_pos(check_pos, link_index);

                    if is_temp_map_exist(mappos.x, mappos.y, mappos.group as u64) {
                        alert.show_alert(
                            systems,
                            AlertBuilder::new_confirm(
                                "Temp File",
                                "Temporary file found! Would you like to load this file?",
                            )
                            .with_index(AlertIndex::LoadTempFile(mappos)),
                        );
                    } else if load_and_apply_map(systems, content, mappos, seconds)? {
                        content.interface.notification.add_msg(
                            systems,
                            format!(
                                "Map [X: {} Y: {} Group: {}] Loaded!",
                                mappos.x, mappos.y, mappos.group
                            ),
                            seconds,
                        );
                    } else {
                        alert.show_alert(
                            systems,
                            &AlertBuilder::new_info("Error", "Failed to load map"),
                        );
                    }

                    return Ok(());
                }
            }
            MouseInputType::MiddleDown => content.map_view.set_map_drag(mouse_pos),
            MouseInputType::MiddleDownMove => {
                content.map_view.last_camera_pos =
                    content.map_view.update_map_drag(graphics, mouse_pos);
            }
            _ => {}
        }

        if in_drawing_area(content, systems, mouse_pos) {
            match inputtype {
                MouseInputType::LeftDown => {
                    content.map_view.last_camera_pos = content.map_view.camera_pos;
                    match cur_tool {
                        ToolType::Paint => match cur_tab {
                            TabButton::Tileset => update_map_tile(content, systems, true),
                            TabButton::Attributes => update_map_attribute(content, systems, true),
                            TabButton::CustomTiles => set_preset(content, systems),
                            TabButton::DirBlock => update_map_dirblock(content, systems, true),
                            TabButton::Zones => update_map_zone(content, systems, true),
                            _ => {}
                        },
                        ToolType::Eraser => match cur_tab {
                            TabButton::Tileset | TabButton::CustomTiles => {
                                update_map_tile(content, systems, false)
                            }
                            TabButton::Attributes => update_map_attribute(content, systems, false),
                            TabButton::DirBlock => update_map_dirblock(content, systems, false),
                            TabButton::Zones => update_map_zone(content, systems, false),
                            _ => {}
                        },
                        ToolType::Fill => match cur_tab {
                            TabButton::Tileset => update_tile_fill(content, systems, true),
                            TabButton::Attributes => update_attribute_fill(content, systems, true),
                            _ => {}
                        },
                        ToolType::Picker => match cur_tab {
                            TabButton::Tileset => picker_layer_update(content, systems),
                            TabButton::Attributes => picker_attribute_update(content, systems),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                MouseInputType::LeftDownMove => {
                    content.map_view.hover_tile(systems, mouse_pos);
                    match cur_tool {
                        ToolType::Paint => match cur_tab {
                            TabButton::Tileset => update_map_tile(content, systems, true),
                            TabButton::Attributes => update_map_attribute(content, systems, true),
                            TabButton::CustomTiles => set_preset(content, systems),
                            TabButton::DirBlock => update_map_dirblock(content, systems, true),
                            TabButton::Zones => update_map_zone(content, systems, true),
                            _ => {}
                        },
                        ToolType::Eraser => match cur_tab {
                            TabButton::CustomTiles | TabButton::Tileset => {
                                update_map_tile(content, systems, false)
                            }
                            TabButton::Attributes => update_map_attribute(content, systems, false),
                            TabButton::DirBlock => update_map_dirblock(content, systems, false),
                            TabButton::Zones => update_map_zone(content, systems, false),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                MouseInputType::RightDownMove => {
                    content.map_view.hover_tile(systems, mouse_pos);
                    match cur_tool {
                        ToolType::Paint | ToolType::Eraser => match cur_tab {
                            TabButton::CustomTiles | TabButton::Tileset => {
                                update_map_tile(content, systems, false)
                            }
                            TabButton::Attributes => update_map_attribute(content, systems, false),
                            TabButton::DirBlock => update_map_dirblock(content, systems, false),
                            TabButton::Zones => update_map_zone(content, systems, false),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                MouseInputType::RightDown => {
                    content.map_view.last_camera_pos = content.map_view.camera_pos;
                    match cur_tool {
                        ToolType::Paint | ToolType::Eraser => match cur_tab {
                            TabButton::CustomTiles | TabButton::Tileset => {
                                update_map_tile(content, systems, false)
                            }
                            TabButton::Attributes => update_map_attribute(content, systems, false),
                            TabButton::DirBlock => update_map_dirblock(content, systems, false),
                            TabButton::Zones => update_map_zone(content, systems, false),
                            _ => {}
                        },
                        ToolType::Fill => match cur_tab {
                            TabButton::Tileset => update_tile_fill(content, systems, false),
                            TabButton::Attributes => update_attribute_fill(content, systems, false),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                MouseInputType::Move => match cur_tool {
                    ToolType::Paint | ToolType::Eraser | ToolType::Fill | ToolType::Picker => {
                        content.map_view.hover_tile(systems, mouse_pos);
                    }
                    _ => {}
                },
                MouseInputType::MiddleDown
                | MouseInputType::MiddleDownMove
                | MouseInputType::Release
                | MouseInputType::DoubleLeftDown
                | MouseInputType::DoubleRightDown => {}
            }
        }

        content.interface.footer.set_tile_pos(
            systems,
            (
                content.map_view.tile.cur_pos.x as u32,
                content.map_view.tile.cur_pos.y as u32,
            ),
        );
    }

    if let MouseInputType::Release = inputtype {
        content.map_view.clear_map_drag();
        content.data.record_placeholder();

        content.map_view.camera_pos = content.map_view.last_camera_pos;
    }
    Ok(())
}

pub fn handle_key_input(
    key: &Key,
    pressed: bool,
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    elwt: &ActiveEventLoop,
    seconds: f32,
) -> Result<()> {
    if alert.visible {
        if alert.alert_key_input(content, systems, elwt, seconds, key, pressed)? {
            return Ok(());
        }

        if !alert.allow_action {
            return Ok(());
        }
    }

    match content.interface.selected_textbox {
        SelectedTextbox::SampleTextbox => {
            /*content
            .interface
            .sample
            .sample_textbox
            .enter_text(systems, key, pressed, false);*/
        }
        SelectedTextbox::AttrContent => {
            match content.interface.side_window.attributes.cur_attribute {
                EditorMapAttribute::Warp => {
                    if let Some(index) = content
                        .interface
                        .side_window
                        .attributes
                        .attr_position
                        .cur_textbox
                    {
                        content
                            .interface
                            .side_window
                            .attributes
                            .attr_position
                            .input_box[index]
                            .textbox
                            .enter_text(systems, key, pressed, true);
                    }
                }
                EditorMapAttribute::ItemSpawn => {
                    if let Some(index) = content
                        .interface
                        .side_window
                        .attributes
                        .attr_itemspawn
                        .cur_textbox
                    {
                        content
                            .interface
                            .side_window
                            .attributes
                            .attr_itemspawn
                            .input_box[index]
                            .textbox
                            .enter_text(systems, key, pressed, true);
                    }
                }
                EditorMapAttribute::Shop => {
                    content
                        .interface
                        .side_window
                        .attributes
                        .attr_index
                        .input_box
                        .textbox
                        .enter_text(systems, key, pressed, true);
                }
                EditorMapAttribute::Sign => {
                    content
                        .interface
                        .side_window
                        .attributes
                        .attr_sign
                        .input_box
                        .textbox
                        .enter_text(systems, key, pressed, false);
                }
                _ => {}
            }
        }
        SelectedTextbox::ZoneTextbox => {
            if let Some(index) = content.interface.side_window.zone.cur_textbox {
                content.interface.side_window.zone.textbox[index]
                    .enter_text(systems, key, pressed, true);

                let result = content.interface.side_window.zone.textbox[index]
                    .text
                    .parse::<u64>()
                    .ok();

                if index == 0 {
                    if let Some(val) = result {
                        content.data.mapdata.zones[content.interface.side_window.zone.cur_zone].0 =
                            val;
                    }
                } else {
                    let id = index.saturating_sub(1);
                    content.data.mapdata.zones[content.interface.side_window.zone.cur_zone].1[id] =
                        result;
                }
            }
        }
        SelectedTextbox::MapPosTextbox => {
            if let Some(index) = content.interface.mappos_input.cur_textbox {
                content.interface.mappos_input.textbox[index]
                    .enter_text(systems, key, pressed, true);
            }
        }
        SelectedTextbox::None => match key {
            Key::Named(Named::Control) => content.input.ctrl_down = pressed,
            Key::Named(Named::Shift) => content.input.shift_down = pressed,
            Key::Character('o') | Key::Character('O') => {
                if pressed && content.input.ctrl_down {
                    content
                        .interface
                        .mappos_input
                        .open(systems, MapPosInputType::LoadMap);
                }
            }
            Key::Character('s') | Key::Character('S') => {
                if pressed && content.input.ctrl_down {
                    if !content.input.shift_down
                        && let Some(mappos) = content.data.pos
                    {
                        if save_map_change(content, mappos)? {
                            content.interface.notification.add_msg(
                                systems,
                                format!(
                                    "Map [X: {} Y: {} Group: {}] Saved!",
                                    mappos.x, mappos.y, mappos.group
                                ),
                                seconds,
                            );
                        } else {
                            alert.show_alert(
                                systems,
                                &AlertBuilder::new_info("Error", "Failed to save map"),
                            );
                        }
                    } else {
                        content
                            .interface
                            .mappos_input
                            .open(systems, MapPosInputType::SaveMap);
                    }
                }
            }
            Key::Character('z') => {
                if pressed && content.input.ctrl_down {
                    apply_undo(content, systems);
                }
            }
            Key::Character('y') => {
                if pressed && content.input.ctrl_down {
                    apply_redo(content, systems);
                }
            }
            _ => {}
        },
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_wheel(
    systems: &mut SystemHolder,
    graphics: &mut Graphics<FlatControls>,
    amount: f32,
    screen_pos: Vec2,
    content: &mut Content,
) {
    // We convert the mouse position to render position as the y pos increase upward
    let mouse_pos = Vec2::new(screen_pos.x, systems.size.height - screen_pos.y);

    if in_layer_area(systems, mouse_pos) {
        let mut cur_layer = content.interface.tool.cur_layer;
        if amount > 0.0 {
            cur_layer = cur_layer
                .saturating_add(1)
                .min(MapLayers::Count as usize - 1);
        } else {
            cur_layer = cur_layer.saturating_sub(1);
        }

        content.interface.tool.layer_button[content.interface.tool.cur_layer]
            .set_disable(systems, false);
        content.interface.tool.cur_layer = cur_layer;
        content.interface.tool.layer_button[content.interface.tool.cur_layer]
            .set_disable(systems, true);
    } else if in_view_screen(systems, mouse_pos) {
        let zoom_value = content.interface.tool.zoom_scroll.value;

        if amount > 0.0 {
            content
                .interface
                .tool
                .zoom_scroll
                .set_value(systems, zoom_value.saturating_add(1));
        } else {
            content
                .interface
                .tool
                .zoom_scroll
                .set_value(systems, zoom_value.saturating_sub(1));
        }

        let new_zoom_value = content.interface.tool.zoom_scroll.value;
        let zoom_level = 100 + (10 * new_zoom_value);
        systems.gfx.set_text(
            &mut systems.renderer,
            &content.interface.tool.zoom_label,
            &format!("{zoom_level}%"),
        );

        let set_zoom = zoom_level as f32 * 0.01;
        let rounded_value = (set_zoom * 10.0).round() / 10.0;
        content
            .map_view
            .adjust_map_by_zoom(systems, graphics, rounded_value);
        graphics.system.controls_mut().settings_mut().zoom = rounded_value;
        systems.config.zoom = rounded_value;
    }
}
