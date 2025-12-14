use graphics::*;

use crate::{
    content::{
        Content,
        interface::side_window::{PresetWindow, PresetWindowType},
        widget::{Alert, AlertBuilder, AlertIndex, Tooltip, is_within_area},
    },
    data_types::*,
    database::{PresetFrames, PresetPos, PresetTypeList},
    renderer::SystemHolder,
    resource::TilePos,
};

impl PresetWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        match self.window_type {
            PresetWindowType::Base => {
                let in_hover = self.base.edit_button.in_area(systems, mouse_pos);
                self.base.edit_button.set_hover(systems, in_hover);
                self.base.preset_list.hover_list(systems, mouse_pos);
                self.base.preset_list.hover_scrollbar(systems, mouse_pos);
            }
            PresetWindowType::Editor => {
                let in_scroll = self.editor.scrollbar.in_scroll(mouse_pos);
                self.editor.scrollbar.set_hover(systems, in_scroll);
                let in_scroll = self.editor.frame_scroll.in_scroll(mouse_pos);
                self.editor.frame_scroll.set_hover(systems, in_scroll);
                self.editor.tile_list.hover_widget(systems, mouse_pos);
                let in_hover = self.editor.save_button.in_area(systems, mouse_pos);
                self.editor.save_button.set_hover(systems, in_hover);
                let in_hover = self.editor.cancel_button.in_area(systems, mouse_pos);
                self.editor.cancel_button.set_hover(systems, in_hover);
                self.editor.type_list.hover_widget(systems, mouse_pos);
            }
        }
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.base.edit_button.set_click(systems, false);
        self.base
            .preset_list
            .scrollbar
            .set_hold(systems, false, mouse_pos);

        self.editor.scrollbar.set_hold(systems, false, mouse_pos);
        self.editor.frame_scroll.set_hold(systems, false, mouse_pos);
        self.editor.tile_list.reset_widget(systems, mouse_pos);
        self.editor.save_button.set_click(systems, false);
        self.editor.cancel_button.set_click(systems, false);
        self.editor.type_list.reset_widget(systems, mouse_pos);
        self.editor.selection.in_hold = false;
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        match self.window_type {
            PresetWindowType::Base => {
                if self.base.preset_list.scrollbar.in_scroll(mouse_pos) {
                    self.base
                        .preset_list
                        .scrollbar
                        .set_hold(systems, true, mouse_pos);
                    return true;
                }
            }
            PresetWindowType::Editor => {
                if self.editor.scrollbar.in_scroll(mouse_pos) {
                    self.editor.scrollbar.set_hold(systems, true, mouse_pos);
                    return true;
                }
                if self.editor.frame_scroll.in_scroll(mouse_pos) {
                    self.editor.frame_scroll.set_hold(systems, true, mouse_pos);
                    return true;
                }

                if self.editor.tile_list.list.visible
                    && self.editor.tile_list.list.scrollbar.in_scroll(mouse_pos)
                {
                    self.editor
                        .tile_list
                        .list
                        .scrollbar
                        .set_hold(systems, true, mouse_pos);
                    return true;
                }

                if self.editor.type_list.list.visible
                    && self.editor.type_list.list.scrollbar.in_scroll(mouse_pos)
                {
                    self.editor
                        .type_list
                        .list
                        .scrollbar
                        .set_hold(systems, true, mouse_pos);
                    return true;
                }
            }
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }

        match self.window_type {
            PresetWindowType::Base => {
                if self.base.preset_list.visible {
                    self.base
                        .preset_list
                        .scrollbar
                        .set_move_scroll(systems, mouse_pos);
                    self.base.preset_list.update_list_scroll(systems);
                }
            }
            PresetWindowType::Editor => {
                self.editor.scrollbar.set_move_scroll(systems, mouse_pos);
                if self.editor.scrollbar.in_hold {
                    self.update_editor_content(systems);
                }

                self.editor.frame_scroll.set_move_scroll(systems, mouse_pos);
                if self.editor.frame_scroll.in_hold {
                    let id = self.editor.frame_scroll.value;
                    systems.gfx.set_text(
                        &mut systems.renderer,
                        &self.editor.frame_label,
                        &format!("Frm: {}", id + 1),
                    );
                    self.editor.selection.start_pos = Vec2::new(
                        self.editor.frames[id].start.x as f32,
                        self.editor.frames[id].start.y as f32,
                    );
                    self.editor.selection.end_pos = Vec2::new(
                        self.editor.frames[id].end.x as f32,
                        self.editor.frames[id].end.y as f32,
                    );
                    let end_pos = if matches!(
                        self.editor.cur_type,
                        PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
                    ) {
                        self.editor.selection.start_pos + Vec2::new(4.0, 2.0)
                    } else {
                        self.editor.selection.end_pos
                    };
                    self.select_tile(systems, self.editor.selection.start_pos, end_pos);
                    let tileset = self.editor.frames[id].tileset as usize;
                    self.change_tileset(systems, tileset);
                    self.editor
                        .tile_list
                        .list
                        .set_select(systems, Some(tileset), true);
                    self.editor.tile_list.update_label(systems, tileset);
                }

                if self.editor.tile_list.list.visible {
                    self.editor
                        .tile_list
                        .list
                        .scrollbar
                        .set_move_scroll(systems, mouse_pos);
                    self.editor.tile_list.list.update_list_scroll(systems);
                }

                if self.editor.type_list.list.visible {
                    self.editor
                        .type_list
                        .list
                        .scrollbar
                        .set_move_scroll(systems, mouse_pos);
                    self.editor.type_list.list.update_list_scroll(systems);
                }
            }
        }
    }

    pub fn click_edit_button(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let in_area = self.base.edit_button.in_area(systems, mouse_pos);
        if in_area {
            self.base.edit_button.set_click(systems, true);
        }
        in_area
    }

    pub fn click_save_button(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let in_area = self.editor.save_button.in_area(systems, mouse_pos);
        if in_area {
            self.editor.save_button.set_click(systems, true);
        }
        in_area
    }

    pub fn click_cancel_button(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let in_area = self.editor.cancel_button.in_area(systems, mouse_pos);
        if in_area {
            self.editor.cancel_button.set_click(systems, true);
        }
        in_area
    }

    pub fn click_tile_list(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let in_button_area = self.editor.tile_list.button.in_area(systems, mouse_pos);

        if self.editor.tile_list.list.visible {
            let result = self
                .editor
                .tile_list
                .list
                .select_list_by_pos(systems, mouse_pos, true);

            if let Some(index) = result {
                self.editor.tile_list.update_label(systems, index);
                self.editor
                    .tile_list
                    .list
                    .set_visible(systems, false, false);
                return result;
            }
        }

        if in_button_area {
            self.editor.tile_list.button.set_click(systems, true);
            self.editor.tile_list.list.set_visible(
                systems,
                !self.editor.tile_list.list.visible,
                false,
            );
        }

        None
    }

    pub fn click_type_list(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let in_button_area = self.editor.type_list.button.in_area(systems, mouse_pos);

        if self.editor.type_list.list.visible {
            let result = self
                .editor
                .type_list
                .list
                .select_list_by_pos(systems, mouse_pos, true);

            if let Some(index) = result {
                self.editor.type_list.update_label(systems, index);
                self.editor
                    .type_list
                    .list
                    .set_visible(systems, false, false);
                return result;
            }
        }

        if in_button_area {
            self.editor.type_list.button.set_click(systems, true);
            self.editor.type_list.list.set_visible(
                systems,
                !self.editor.type_list.list.visible,
                false,
            );
        }

        None
    }

    pub fn click_tilesheet(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.editor.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.editor.start_pos.y
                + ((self.editor.area_size.y - (34.0 * systems.scale as f32).floor())
                    - tileset_size.y)
                + self.editor.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        if self.editor.tile_list.list.visible {
            return false;
        }

        if is_within_area(mouse_pos, self.editor.start_pos, self.editor.area_size)
            && is_within_area(
                mouse_pos,
                Vec2::new(tileset_pos.x, tileset_pos.y),
                tileset_size,
            )
        {
            let selecting_pos = mouse_pos - Vec2::new(tileset_pos.x, tileset_pos.y);
            let tile_size = (20.0 * systems.scale as f32).floor();
            let tile_pos = Vec2::new(
                (selecting_pos.x / tile_size)
                    .floor()
                    .min(TILESET_COUNT_X.saturating_sub(1) as f32),
                (selecting_pos.y / tile_size)
                    .floor()
                    .min(TILESET_COUNT_Y.saturating_sub(1) as f32),
            );

            if self.editor.selection.in_hold {
                let limit = Vec2::new(2.0, 2.0);
                self.editor.selection.end_pos = tile_pos.clamp(
                    self.editor.selection.start_pos - limit,
                    self.editor.selection.start_pos + limit,
                );
            } else {
                self.editor.selection.in_hold = true;
                self.editor.selection.start_pos = tile_pos;
                self.editor.selection.end_pos = tile_pos;
            }

            let end_pos = if matches!(
                self.editor.cur_type,
                PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
            ) {
                self.editor.selection.start_pos = self.editor.selection.start_pos.min(Vec2::new(
                    TILESET_COUNT_X.saturating_sub(5) as f32,
                    TILESET_COUNT_Y.saturating_sub(3) as f32,
                ));
                self.editor.selection.end_pos = self.editor.selection.start_pos;

                self.editor.selection.start_pos + Vec2::new(4.0, 2.0)
            } else {
                self.editor.selection.end_pos
            };

            self.editor.frames[self.editor.frame_scroll.value].start = PresetPos {
                x: self.editor.selection.start_pos.x as u16,
                y: self.editor.selection.start_pos.y as u16,
            };
            self.editor.frames[self.editor.frame_scroll.value].end = PresetPos {
                x: self.editor.selection.end_pos.x as u16,
                y: self.editor.selection.end_pos.y as u16,
            };
            self.editor.frames[self.editor.frame_scroll.value].tileset =
                self.editor.cur_tileset as u16;

            self.select_tile(systems, self.editor.selection.start_pos, end_pos);
            return true;
        }
        false
    }
}

pub fn side_preset_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    mouse_pos: Vec2,
) -> Result<bool> {
    if !content.interface.side_window.presets.visible {
        return Ok(false);
    }

    let gui = &mut content.interface.side_window.presets;

    match gui.window_type {
        PresetWindowType::Base => {
            if gui.hold_scrollbar(systems, mouse_pos) {
                return Ok(true);
            }

            if gui.click_edit_button(systems, mouse_pos) {
                gui.switch_state(systems, PresetWindowType::Editor);
                preset_update_editor(content, systems);
                return Ok(true);
            }

            if let Some(index) = gui
                .base
                .preset_list
                .select_list_by_pos(systems, mouse_pos, true)
            {
                if gui.selected_index != index {
                    gui.selected_index = index;
                    preset_update_preview(content, systems);
                }
                return Ok(true);
            }
        }
        PresetWindowType::Editor => {
            if gui.hold_scrollbar(systems, mouse_pos) {
                return Ok(true);
            }

            if let Some(list_index) = gui.click_type_list(systems, mouse_pos) {
                gui.editor.cur_type = PresetTypeList::from_index(list_index);
                gui.editor.frame_scroll.set_value(systems, 0);
                gui.editor.frame_scroll.set_max_value(
                    systems,
                    if matches!(
                        gui.editor.cur_type,
                        PresetTypeList::Normal | PresetTypeList::AutoTile
                    ) {
                        0
                    } else {
                        3
                    },
                );
                gui.editor.frames = [PresetFrames::default(); 4];
                gui.editor.selection.start_pos = Vec2::new(0.0, 0.0);
                gui.editor.selection.end_pos = Vec2::new(0.0, 0.0);
                let end_pos = if matches!(
                    gui.editor.cur_type,
                    PresetTypeList::AutotileAnimated | PresetTypeList::AutoTile
                ) {
                    Vec2::new(4.0, 2.0)
                } else {
                    Vec2::new(0.0, 0.0)
                };
                gui.select_tile(systems, Vec2::new(0.0, 0.0), end_pos);

                return Ok(true);
            }

            if let Some(list_index) = gui.click_tile_list(systems, mouse_pos) {
                gui.change_tileset(systems, list_index);
                return Ok(true);
            }

            if gui.click_save_button(systems, mouse_pos) {
                alert.show_alert(
                    systems,
                    AlertBuilder::new_txt_input("Preset Name").with_index(AlertIndex::SavePreset),
                );
                return Ok(true);
            }

            if gui.click_cancel_button(systems, mouse_pos) {
                gui.switch_state(systems, PresetWindowType::Base);
                return Ok(true);
            }

            if gui.click_tilesheet(systems, mouse_pos) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

pub fn side_preset_clickdrag_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.presets.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.presets;

    match gui.window_type {
        PresetWindowType::Base => {}
        PresetWindowType::Editor => {
            if matches!(
                gui.editor.cur_type,
                PresetTypeList::AutotileAnimated | PresetTypeList::AutoTile
            ) {
                return false;
            }

            if gui.click_tilesheet(systems, mouse_pos) {
                return true;
            }
        }
    }

    false
}

pub fn preset_update_list(content: &mut Content, systems: &mut SystemHolder) {
    let mut list = Vec::with_capacity(MAX_PRESETS);
    for (i, data) in content.preset.data.iter().enumerate() {
        list.push(format!("{}: {}", i + 1, &data.name));
    }
    content
        .interface
        .side_window
        .presets
        .base
        .preset_list
        .update_list(
            systems,
            list,
            Some(content.interface.side_window.presets.selected_index),
        );

    preset_update_preview(content, systems);
}

pub fn preset_update_preview(content: &mut Content, systems: &mut SystemHolder) {
    let selected_index = content.interface.side_window.presets.selected_index;

    let gui = &mut content.interface.side_window.presets;

    systems.gfx.set_text(
        &mut systems.renderer,
        &gui.base.preview_name,
        &content.preset.data[selected_index].name,
    );
    systems.gfx.center_text(&gui.base.preview_name);

    systems.gfx.set_text(
        &mut systems.renderer,
        &gui.base.preview_info,
        match content.preset.data[selected_index].draw_type {
            PresetTypeList::Normal => "Normal",
            PresetTypeList::Animated => "Animated",
            PresetTypeList::AutoTile => "AutoTile",
            PresetTypeList::AutotileAnimated => "Animated AutoTile",
        },
    );
    systems.gfx.center_text(&gui.base.preview_info);

    gui.base.frames = content.preset.data[selected_index].frames;
    gui.base.preset_type = content.preset.data[selected_index].draw_type;

    let p_type = gui.base.preset_type;
    let is_autotile = matches!(
        p_type,
        PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
    );
    let is_animated = matches!(
        p_type,
        PresetTypeList::Animated | PresetTypeList::AutotileAnimated
    );

    let tile_size = (20.0 * systems.scale as f32).floor();
    let mut selected_preset_tiles = Vec::with_capacity(52);
    for (i, gfx) in gui.base.preview.iter_mut().enumerate() {
        systems.gfx.remove_gfx(&mut systems.renderer, gfx);

        let frame = gui.base.frames[i];

        if i > 0 && !is_animated {
            continue;
        }

        let size = if is_autotile {
            Vec2::new(5.0, 3.0)
        } else {
            Vec2::new(
                frame.start.x.abs_diff(frame.end.x) as f32 + 1.0,
                frame.start.y.abs_diff(frame.end.y) as f32 + 1.0,
            )
        };
        let pos = Vec2::new(
            if frame.start.x > frame.end.x {
                frame.end.x
            } else {
                frame.start.x
            } as f32,
            if frame.start.y > frame.end.y {
                frame.end.y
            } else {
                frame.start.y
            } as f32,
        );

        let preview_pos = systems.gfx.get_pos(&gui.base.preview_bg);
        let offset_pos = {
            let preview_size = (size - Vec2::ONE).max(Vec2::ONE);
            (Vec2::new(3.0, 2.0) - preview_size).max(Vec2::ZERO)
        };

        let img = Image::new(
            Some(systems.resource.tilesheet[frame.tileset as usize].img),
            &mut systems.renderer,
            Vec3::new(
                preview_pos.x + (2.0 * systems.scale as f32).floor() + (offset_pos.x * tile_size),
                preview_pos.y + (2.0 * systems.scale as f32).floor() + (offset_pos.y * tile_size),
                ORDER_WINDOW_CONTENT,
            ),
            (Vec2::new(size.x * 20.0, size.y * 20.0) * systems.scale as f32).floor(),
            Vec4::new(
                pos.x * 20.0,
                (TILESET_COUNT_Y as f32 - (pos.y + size.y)) * 20.0,
                size.x * 20.0,
                size.y * 20.0,
            ),
            2,
        );
        *gfx = systems.gfx.add_image(
            img,
            RENDER_GUI3,
            "Preset Tiles",
            gui.visible && gui.window_type == PresetWindowType::Base && i == 0,
            CameraView::SubView1,
        );

        let start_pos = Vec2::new(frame.start.x as f32, frame.start.y as f32);
        let end_pos = Vec2::new(frame.end.x as f32, frame.end.y as f32);
        let tilesheet_pos = Vec2::new(
            start_pos.x.min(end_pos.x),
            TILESET_COUNT_Y.saturating_sub(1) as f32 - start_pos.y.min(end_pos.y),
        );
        let tile_size = (size.x as u32, size.y as u32);
        for x in 0..tile_size.0 {
            for y in 0..tile_size.1 {
                if is_autotile && y == 0 && x > 2 {
                    continue;
                }

                if let Some(id) = systems
                    .resource
                    .tile_pos_loc
                    .get(&TilePos {
                        x: (tilesheet_pos.x as u32 + x) * TEXTURE_SIZE,
                        y: (tilesheet_pos.y as u32 - y) * TEXTURE_SIZE,
                        file: frame.tileset as u32,
                    })
                    .copied()
                {
                    selected_preset_tiles.push(id);
                }
            }
        }
    }

    content.preset.selected_preset_tiles = selected_preset_tiles;
}

pub fn preset_update_editor(content: &mut Content, systems: &mut SystemHolder) {
    if !content.interface.side_window.presets.visible {
        return;
    }

    let gui = &mut content.interface.side_window.presets;

    gui.editor.cur_type = content.preset.data[gui.selected_index].draw_type;
    gui.editor.frame_scroll.set_value(systems, 0);
    gui.editor.frame_scroll.set_max_value(
        systems,
        if matches!(
            gui.editor.cur_type,
            PresetTypeList::Normal | PresetTypeList::AutoTile
        ) {
            0
        } else {
            3
        },
    );
    systems
        .gfx
        .set_text(&mut systems.renderer, &gui.editor.frame_label, "Frm: 1");
    gui.editor
        .type_list
        .list
        .set_select(systems, Some(gui.editor.cur_type as usize), true);
    gui.editor
        .type_list
        .update_label(systems, gui.editor.cur_type as usize);
    gui.editor.frames = content.preset.data[gui.selected_index].frames;
    gui.editor.selection.start_pos = Vec2::new(
        gui.editor.frames[0].start.x as f32,
        gui.editor.frames[0].start.y as f32,
    );
    gui.editor.selection.end_pos = Vec2::new(
        gui.editor.frames[0].end.x as f32,
        gui.editor.frames[0].end.y as f32,
    );
    let end_pos = if matches!(
        gui.editor.cur_type,
        PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
    ) {
        gui.editor.selection.start_pos + Vec2::new(4.0, 2.0)
    } else {
        gui.editor.selection.end_pos
    };
    gui.select_tile(systems, gui.editor.selection.start_pos, end_pos);

    let tileset = gui.editor.frames[0].tileset as usize;
    gui.change_tileset(systems, tileset);
    gui.editor
        .tile_list
        .list
        .set_select(systems, Some(tileset), true);
    gui.editor.tile_list.update_label(systems, tileset);
}

pub fn save_preset(content: &mut Content, systems: &mut SystemHolder, name: String) -> Result<()> {
    let gui = &mut content.interface.side_window.presets;
    content.preset.data[gui.selected_index]
        .name
        .clone_from(&name);
    content.preset.data[gui.selected_index].draw_type = gui.editor.cur_type;
    content.preset.data[gui.selected_index].frames = gui.editor.frames;
    content.preset.save_preset(gui.selected_index)?;
    gui.switch_state(systems, PresetWindowType::Base);
    preset_update_list(content, systems);
    Ok(())
}
