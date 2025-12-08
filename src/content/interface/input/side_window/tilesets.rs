use graphics::*;

use crate::{
    content::{
        Content,
        interface::side_window::TilesetWindow,
        widget::{Tooltip, is_within_area},
    },
    data_types::*,
    renderer::SystemHolder,
};

impl TilesetWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        let in_scroll = self.scrollbar.in_scroll(mouse_pos);
        self.scrollbar.set_hover(systems, in_scroll);

        self.tile_list.hover_widget(systems, mouse_pos);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.scrollbar.set_hold(systems, false, mouse_pos);
        self.tile_list.reset_widget(systems, mouse_pos);
        self.selection.in_hold = false;
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.scrollbar.in_scroll(mouse_pos) {
            self.scrollbar.set_hold(systems, true, mouse_pos);
            return true;
        }

        if self.tile_list.list.visible && self.tile_list.list.scrollbar.in_scroll(mouse_pos) {
            self.tile_list
                .list
                .scrollbar
                .set_hold(systems, true, mouse_pos);
            return true;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }

        self.scrollbar.set_move_scroll(systems, mouse_pos);
        if self.scrollbar.in_hold {
            self.update_content(systems);
        }

        if self.tile_list.list.visible {
            self.tile_list
                .list
                .scrollbar
                .set_move_scroll(systems, mouse_pos);
            self.tile_list.list.update_list_scroll(systems);
        }
    }

    pub fn click_tile_list(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let in_button_area = self.tile_list.button.in_area(systems, mouse_pos);

        if self.tile_list.list.visible {
            let result = self
                .tile_list
                .list
                .select_list_by_pos(systems, mouse_pos, true);

            if let Some(index) = result {
                self.tile_list.update_label(systems, index);
                self.tile_list.list.set_visible(systems, false, false);
                return result;
            }
        }

        if in_button_area {
            self.tile_list.button.set_click(systems, true);
            self.tile_list
                .list
                .set_visible(systems, !self.tile_list.list.visible, false);
        }

        None
    }

    pub fn click_tilesheet(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.start_pos.y
                + ((self.area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y)
                + self.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        if self.tile_list.list.visible {
            return false;
        }

        if is_within_area(mouse_pos, self.start_pos, self.area_size)
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

            if self.selection.in_hold {
                self.selection.end_pos = tile_pos;
            } else {
                self.selection.in_hold = true;
                self.selection.start_pos = tile_pos;
                self.selection.end_pos = tile_pos;
            }

            self.select_tile(systems, self.selection.start_pos, self.selection.end_pos);
            return true;
        }
        false
    }
}

pub fn side_tileset_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.tilesets.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.tilesets;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if let Some(list_index) = gui.click_tile_list(systems, mouse_pos) {
        gui.change_tileset(systems, list_index);
        return true;
    }

    if gui.click_tilesheet(systems, mouse_pos) {
        return true;
    }

    false
}

pub fn side_tileset_clickdrag_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.tilesets.visible {
        return false;
    }

    if content
        .interface
        .side_window
        .tilesets
        .click_tilesheet(systems, mouse_pos)
    {
        return true;
    }

    false
}
