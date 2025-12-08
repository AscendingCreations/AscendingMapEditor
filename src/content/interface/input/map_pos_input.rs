use graphics::*;

use crate::{
    content::{
        Content,
        interface::map_pos_input::{MapPosInput, MapPosInputType},
        load_and_apply_map, save_map_change,
        widget::{Alert, AlertBuilder, AlertIndex, Tooltip},
    },
    data_types::{MouseInputType, Result, SelectedTextbox},
    database::{MapPosition, delete_temp_map_file, is_temp_map_exist, map},
    renderer::SystemHolder,
};
use input::{Key, Named};

impl MapPosInput {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        for button in self.button.iter_mut() {
            let in_hover = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_hover);
        }
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

    pub fn click_buttons(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in self.button.iter_mut().enumerate() {
            if button.in_area(systems, screen_pos) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }

    pub fn click_textbox(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let mut did_click = false;

        let old_cur_textbox = self.cur_textbox;

        for (index, inputbox) in self.textbox.iter_mut().enumerate() {
            if inputbox.in_area(systems, mouse_pos) {
                inputbox.set_select(systems, true);
                inputbox.set_hold(true);
                inputbox.select_text(systems, mouse_pos);
                self.cur_textbox = Some(index);
                did_click = true;
                break;
            }
        }

        if old_cur_textbox != self.cur_textbox
            && let Some(index) = old_cur_textbox
        {
            self.textbox[index].set_select(systems, false);
        }

        did_click
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mappos_mouse_input(
    systems: &mut SystemHolder,
    content: &mut Content,
    alert: &mut Alert,
    input_type: MouseInputType,
    tooltip: &mut Tooltip,
    screen_pos: Vec2,
    seconds: f32,
) -> Result<()> {
    if !content.interface.mappos_input.visible {
        return Ok(());
    }

    match input_type {
        MouseInputType::Move => {
            content
                .interface
                .mappos_input
                .hover_widgets(systems, screen_pos, tooltip);
        }
        MouseInputType::LeftDown => {
            if content
                .interface
                .mappos_input
                .hold_scrollbar(systems, screen_pos)
            {
                return Ok(());
            }

            if let Some(index) = content
                .interface
                .mappos_input
                .click_buttons(systems, screen_pos)
            {
                if index == 0 {
                    let mappos = MapPosition {
                        x: content.interface.mappos_input.textbox[0]
                            .text
                            .parse::<i32>()
                            .unwrap_or_default(),
                        y: content.interface.mappos_input.textbox[1]
                            .text
                            .parse::<i32>()
                            .unwrap_or_default(),
                        group: content.interface.mappos_input.textbox[2]
                            .text
                            .parse::<i32>()
                            .unwrap_or_default(),
                    };

                    // Confirm
                    match content.interface.mappos_input.input_type {
                        MapPosInputType::LoadMap => {
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
                        }
                        MapPosInputType::SaveMap => {
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
                        }
                        MapPosInputType::None => {}
                    }
                }
                content.interface.mappos_input.set_visible(systems, false);
            }

            if content
                .interface
                .click_textbox(systems, screen_pos, SelectedTextbox::MapPosTextbox)
            {}
        }
        MouseInputType::LeftDownMove => {
            content
                .interface
                .mappos_input
                .hold_move_scrollbar(systems, screen_pos);
            content.interface.hold_move_textbox(systems, screen_pos);
        }
        MouseInputType::Release => {
            content
                .interface
                .mappos_input
                .reset_widgets(systems, screen_pos);
            content.interface.reset_textbox();
        }
        _ => {}
    }
    Ok(())
}
