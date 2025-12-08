use graphics::*;

use crate::{
    content::{
        Content,
        interface::side_window::{AttrIndex, AttrPosition},
        widget::Tooltip,
    },
    renderer::SystemHolder,
};

impl AttrIndex {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        let in_scroll = self.content_scroll.in_scroll(mouse_pos);
        self.content_scroll.set_hover(systems, in_scroll);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.content_scroll.set_hold(systems, false, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.content_scroll.in_scroll(mouse_pos) {
            self.content_scroll.set_hold(systems, true, mouse_pos);
            return true;
        }
        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }

        self.content_scroll.set_move_scroll(systems, mouse_pos);
        if self.content_scroll.in_hold {
            self.update_content(systems);
        }
    }

    pub fn click_textbox(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.input_box.textbox.in_area(systems, mouse_pos) {
            self.input_box.textbox.set_select(systems, true);
            self.input_box.textbox.set_hold(true);
            self.input_box.textbox.select_text(systems, mouse_pos);
            true
        } else {
            false
        }
    }
}

pub fn attr_index_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.attributes.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.attributes;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    false
}
