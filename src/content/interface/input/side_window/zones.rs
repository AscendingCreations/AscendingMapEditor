use graphics::*;

use crate::{
    content::{Content, interface::side_window::ZoneWindow, update_zone_visible, widget::Tooltip},
    renderer::SystemHolder,
};

impl ZoneWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        for checkbox in self.zones.iter_mut() {
            let in_hover = checkbox.in_area(systems, mouse_pos);
            checkbox.set_hover(systems, in_hover);
        }

        let in_scroll = self.scrollbar.in_scroll(mouse_pos);
        self.scrollbar.set_hover(systems, in_scroll);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        for button in self.zones.iter_mut() {
            button.set_click(systems, false);
        }

        self.scrollbar.set_hold(systems, false, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.scrollbar.in_scroll(mouse_pos) {
            self.scrollbar.set_hold(systems, true, mouse_pos);
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
    }

    pub fn click_zones(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        for (index, checkbox) in self.zones.iter_mut().enumerate() {
            if checkbox.in_area(systems, mouse_pos) {
                return Some(index);
            }
        }

        None
    }

    pub fn click_textbox(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        let mut did_click = false;

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

        did_click
    }
}

pub fn side_zone_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.zone.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.zone;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if let Some(index) = gui.click_zones(systems, mouse_pos)
        && index != gui.cur_zone
    {
        gui.zones[gui.cur_zone].set_value(systems, false);
        gui.cur_zone = index;
        gui.zones[gui.cur_zone].set_value(systems, true);

        let zone_data = content.data.mapdata.zones[index];

        gui.textbox[0].set_text(systems, format!("{}", zone_data.0));

        for i in 0..5 {
            gui.textbox[i + 1].set_text(
                systems,
                if let Some(data) = zone_data.1[i] {
                    format!("{data}")
                } else {
                    String::new()
                },
            );
        }

        update_zone_visible(content, systems);
        return true;
    }

    false
}
