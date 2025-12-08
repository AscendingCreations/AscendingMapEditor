use graphics::*;

use crate::{
    content::{Content, interface::side_window::PropertiesWindow, widget::Tooltip},
    renderer::SystemHolder,
};

impl PropertiesWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {}

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }
    }
}

pub fn side_properties_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.properties.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.properties;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    false
}
