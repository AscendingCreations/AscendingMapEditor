use graphics::Vec2;

use crate::{
    content::{
        Content, apply_redo, apply_undo,
        interface::{map_pos_input::MapPosInputType, menu_bar::MenuBar},
        load_and_apply_map, save_map_change,
        widget::{Alert, AlertBuilder, AlertIndex},
    },
    data_types::Result,
    database::is_temp_map_exist,
    renderer::SystemHolder,
};

impl MenuBar {
    pub fn hover_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        for button in self.button.iter_mut() {
            let in_hover = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_hover);
        }

        self.file_menu.hover_list(systems, mouse_pos);
        self.edit_menu.hover_list(systems, mouse_pos);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, _mouse_pos: Vec2) {
        for button in self.button.iter_mut() {
            button.set_click(systems, false);
        }
    }

    pub fn hold_scrollbar(&mut self, _systems: &mut SystemHolder, _mouse_pos: Vec2) -> bool {
        false
    }

    pub fn hold_move_scrollbar(&mut self, _systems: &mut SystemHolder, _mouse_pos: Vec2) {}

    pub fn click_buttons(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        for (index, button) in self.button.iter_mut().enumerate() {
            if button.in_area(systems, mouse_pos) {
                button.set_click(systems, true);
                return Some(index);
            }
        }

        None
    }
}

pub fn menu_bar_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    mouse_pos: Vec2,
    seconds: f32,
) -> Result<bool> {
    {
        let gui = &mut content.interface.menu_bar;

        if gui.hold_scrollbar(systems, mouse_pos) {
            return Ok(true);
        }

        if let Some(index) = gui.click_buttons(systems, mouse_pos) {
            match index {
                0 => {
                    gui.edit_menu.set_visible(systems, false, true);
                    gui.file_menu
                        .set_visible(systems, !gui.file_menu.visible, true);
                }
                1 => {
                    gui.file_menu.set_visible(systems, false, true);
                    gui.edit_menu
                        .set_visible(systems, !gui.edit_menu.visible, true);
                }
                2 => {
                    gui.file_menu.set_visible(systems, false, true);
                    gui.edit_menu.set_visible(systems, false, true);
                }
                _ => {}
            }
            return Ok(true);
        }
    }

    if let Some(index) = content
        .interface
        .menu_bar
        .file_menu
        .select_list_by_pos(systems, mouse_pos, false)
    {
        match index {
            0 => {
                content
                    .interface
                    .mappos_input
                    .open(systems, MapPosInputType::LoadMap);
            } // Open Map
            1 => {
                if let Some(mappos) = content.data.pos {
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
            } // Save
            2 => {
                content
                    .interface
                    .mappos_input
                    .open(systems, MapPosInputType::SaveMap);
            } // Save As
            3 => {
                if let Some(mappos) = content.data.pos {
                    if load_and_apply_map(systems, content, mappos, seconds)? {
                        content.interface.notification.add_msg(
                            systems,
                            format!(
                                "Map [X: {} Y: {} Group: {}] Reloaded!",
                                mappos.x, mappos.y, mappos.group
                            ),
                            seconds,
                        );
                    } else {
                        alert.show_alert(
                            systems,
                            &AlertBuilder::new_info("Error", "Failed to reload map"),
                        );
                    }
                } else {
                    alert.show_alert(
                        systems,
                        &AlertBuilder::new_info("Error", "No loaded map to reload"),
                    );
                }
            } // Reload Map
            _ => {}
        }
        content
            .interface
            .menu_bar
            .file_menu
            .set_visible(systems, false, true);
        return Ok(true);
    }

    if let Some(index) = content
        .interface
        .menu_bar
        .edit_menu
        .select_list_by_pos(systems, mouse_pos, false)
    {
        match index {
            0 => apply_undo(content, systems), // Undo
            1 => apply_redo(content, systems), // Redo
            _ => {}
        }
        content
            .interface
            .menu_bar
            .edit_menu
            .set_visible(systems, false, true);
        return Ok(true);
    }

    {
        let gui = &mut content.interface;
        gui.menu_bar.file_menu.set_visible(systems, false, true);
        gui.menu_bar.edit_menu.set_visible(systems, false, true);
    }

    Ok(false)
}
