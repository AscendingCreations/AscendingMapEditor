use graphics::*;

use crate::{
    content::{Content, interface::side_window::MusicWindow, widget::Tooltip},
    data_types::Result,
    renderer::SystemHolder,
};

impl MusicWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        self.music_list.hover_list(systems, mouse_pos);
        self.music_list.hover_scrollbar(systems, mouse_pos);

        for button in self.button.iter_mut() {
            let in_area = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_area);
        }
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.music_list
            .scrollbar
            .set_hold(systems, false, mouse_pos);

        for button in self.button.iter_mut() {
            button.set_click(systems, false);
        }
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.music_list.scrollbar.in_scroll(mouse_pos) {
            self.music_list.scrollbar.set_hold(systems, true, mouse_pos);
            return true;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }

        if self.music_list.visible {
            self.music_list
                .scrollbar
                .set_move_scroll(systems, mouse_pos);
            self.music_list.update_list_scroll(systems);
        }
    }

    pub fn click_button(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        for (index, button) in self.button.iter_mut().enumerate() {
            if button.in_area(systems, mouse_pos) && !button.disabled {
                button.set_click(systems, true);
                return Some(index);
            }
        }

        None
    }
}

pub fn side_music_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> Result<bool> {
    if !content.interface.side_window.music.visible {
        return Ok(false);
    }

    let gui = &mut content.interface.side_window.music;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return Ok(true);
    }

    if let Some(list_index) = gui.music_list.select_list_by_pos(systems, mouse_pos, true) {
        let data = content
            .audio_collection
            .audio
            .get(list_index)
            .cloned()
            .filter(|text| text != "None");

        content.data.mapdata.music = data;
        content.data.changed = true;
        content.data.temp_saved = false;
        if let Some(map_pos) = content.data.pos {
            content
                .interface
                .footer
                .set_map_pos(systems, map_pos, false);
        }
        return Ok(true);
    }

    if let Some(index) = gui.click_button(systems, mouse_pos) {
        match index {
            0 => {
                if let Some(music) = &content.data.mapdata.music
                    && music != "None"
                {
                    systems.audio.set_music(format!("./audio/{music}"))?
                }
            } // Play
            1 => {
                systems.audio.stop_music();
            } // Stop
            _ => {}
        }
        return Ok(true);
    }

    Ok(false)
}
