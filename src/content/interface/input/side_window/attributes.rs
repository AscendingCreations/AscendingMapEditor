use graphics::*;

use crate::{
    content::{
        Content,
        interface::side_window::AttributeWindow,
        widget::{Tooltip, checkbox},
    },
    database::{EditorMapAttribute, MapAttribute},
    renderer::SystemHolder,
};

mod attrs_index;
mod attrs_itemspawn;
mod attrs_position;
mod attrs_sign;

use attrs_index::*;
use attrs_itemspawn::*;
use attrs_position::*;
use attrs_sign::*;

impl AttributeWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        for checkbox in self.attribute.iter_mut() {
            let in_hover = checkbox.in_area(systems, mouse_pos);
            checkbox.set_hover(systems, in_hover);
        }
        let in_scroll = self.attribute_scroll.in_scroll(mouse_pos);
        self.attribute_scroll.set_hover(systems, in_scroll);
        self.attr_position
            .hover_widgets(systems, mouse_pos, tooltip);
        self.attr_index.hover_widgets(systems, mouse_pos, tooltip);
        self.attr_sign.hover_widgets(systems, mouse_pos, tooltip);
        self.attr_itemspawn
            .hover_widgets(systems, mouse_pos, tooltip);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        for button in self.attribute.iter_mut() {
            button.set_click(systems, false);
        }
        self.attribute_scroll.set_hold(systems, false, mouse_pos);
        self.attr_position.reset_widgets(systems, mouse_pos);
        self.attr_index.reset_widgets(systems, mouse_pos);
        self.attr_sign.reset_widgets(systems, mouse_pos);
        self.attr_itemspawn.reset_widgets(systems, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.attribute_scroll.in_scroll(mouse_pos) {
            self.attribute_scroll.set_hold(systems, true, mouse_pos);
            return true;
        }

        if self.attr_position.hold_scrollbar(systems, mouse_pos)
            || self.attr_index.hold_scrollbar(systems, mouse_pos)
            || self.attr_sign.hold_scrollbar(systems, mouse_pos)
            || self.attr_itemspawn.hold_scrollbar(systems, mouse_pos)
        {
            return true;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }

        self.attribute_scroll.set_move_scroll(systems, mouse_pos);
        if self.attribute_scroll.in_hold {
            self.update_attributes(systems);
        }

        self.attr_position.hold_move_scrollbar(systems, mouse_pos);
        self.attr_index.hold_move_scrollbar(systems, mouse_pos);
        self.attr_sign.hold_move_scrollbar(systems, mouse_pos);
        self.attr_itemspawn.hold_move_scrollbar(systems, mouse_pos);
    }

    pub fn click_attribute(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        for (index, checkbox) in self.attribute.iter_mut().enumerate() {
            if checkbox.in_area(systems, mouse_pos) {
                return Some(index);
            }
        }

        None
    }

    pub fn update_attributes(&mut self, systems: &mut SystemHolder) {
        self.cur_attr_display = None;
        for (index, checkbox) in self.attribute.iter_mut().enumerate() {
            let attr_index = index + self.attribute_scroll.value + 1;
            let mapattr_text = MapAttribute::as_str(attr_index as u32).to_string();
            let mapattr = EditorMapAttribute::convert_to_plain_enum(attr_index as u32);

            checkbox.change_content_text(systems, mapattr_text);
            checkbox.set_value(systems, self.cur_attribute == mapattr);
            self.cur_attr_display = Some(index);
        }
    }
}

pub fn side_attribute_click_widget(
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

    if let Some(index) = gui.click_attribute(systems, mouse_pos) {
        let attr_index = index + gui.attribute_scroll.value + 1;
        let mapattr = EditorMapAttribute::convert_to_plain_enum(attr_index as u32);
        switch_attributes(content, systems, mapattr, index);
        return true;
    }

    if attr_position_click_widget(content, systems, mouse_pos)
        || attr_index_click_widget(content, systems, mouse_pos)
        || attr_sign_click_widget(content, systems, mouse_pos)
        || attr_itemspawn_click_widget(content, systems, mouse_pos)
    {
        return true;
    }

    false
}

pub fn switch_attributes(
    content: &mut Content,
    systems: &mut SystemHolder,
    mapattr: EditorMapAttribute,
    attr_index: usize,
) {
    let gui = &mut content.interface.side_window.attributes;

    if let Some(index) = gui.cur_attr_display {
        gui.attribute[index].set_value(systems, false);
    }
    gui.attribute[attr_index].set_value(systems, true);
    gui.cur_attr_display = Some(attr_index);
    gui.cur_attribute = mapattr;

    gui.attr_position
        .set_visible(systems, mapattr == EditorMapAttribute::Warp);
    gui.attr_index
        .set_visible(systems, mapattr == EditorMapAttribute::Shop);
    gui.attr_sign
        .set_visible(systems, mapattr == EditorMapAttribute::Sign);
    gui.attr_itemspawn
        .set_visible(systems, mapattr == EditorMapAttribute::ItemSpawn);
}
