use graphics::*;

use crate::{
    content::{Content, interface::side_window::WeatherWindow, widget::Tooltip},
    database::Weather,
    renderer::SystemHolder,
};

impl WeatherWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        self.weather_list.hover_widget(systems, mouse_pos);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.weather_list.reset_widget(systems, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.weather_list.list.visible && self.weather_list.list.scrollbar.in_scroll(mouse_pos) {
            self.weather_list
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

        if self.weather_list.list.visible {
            self.weather_list
                .list
                .scrollbar
                .set_move_scroll(systems, mouse_pos);
            self.weather_list.list.update_list_scroll(systems);
        }
    }

    pub fn click_weather_list(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let in_button_area = self.weather_list.button.in_area(systems, mouse_pos);

        if self.weather_list.list.visible {
            let result = self
                .weather_list
                .list
                .select_list_by_pos(systems, mouse_pos, true);

            if let Some(index) = result {
                self.weather_list.update_label(systems, index);
                self.weather_list.list.set_visible(systems, false, false);
                return result;
            }
        }

        if in_button_area {
            self.weather_list.button.set_click(systems, true);
            self.weather_list
                .list
                .set_visible(systems, !self.weather_list.list.visible, false);
        }

        None
    }
}

pub fn side_weather_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.weather.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.weather;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if let Some(list_index) = gui.click_weather_list(systems, mouse_pos) {
        content.data.mapdata.weather = Weather::from_index(list_index);
        content.data.changed = true;
        content.data.temp_saved = false;
        if let Some(map_pos) = content.data.pos {
            content
                .interface
                .footer
                .set_map_pos(systems, map_pos, false);
        }
        return true;
    }

    false
}
