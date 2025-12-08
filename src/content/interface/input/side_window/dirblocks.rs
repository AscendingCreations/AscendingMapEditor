use graphics::*;

use crate::{
    content::{Content, interface::side_window::DirBlockWindow, widget::Tooltip},
    renderer::SystemHolder,
};

impl DirBlockWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        _tooltip: &mut Tooltip,
    ) {
        if !self.visible {
            return;
        }

        for checkbox in self.blocks.iter_mut() {
            let in_hover = checkbox.in_area(systems, mouse_pos);
            checkbox.set_hover(systems, in_hover);
        }
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, _mouse_pos: Vec2) {
        for button in self.blocks.iter_mut() {
            button.set_click(systems, false);
        }
    }

    pub fn hold_scrollbar(&mut self, _systems: &mut SystemHolder, _mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, _systems: &mut SystemHolder, _mouse_pos: Vec2) {
        if !self.visible {}
    }

    pub fn click_blocks(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        for (index, checkbox) in self.blocks.iter_mut().enumerate() {
            if checkbox.in_area(systems, mouse_pos) {
                return Some(index);
            }
        }

        None
    }
}

pub fn side_dirblock_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if !content.interface.side_window.dirblocks.visible {
        return false;
    }

    let gui = &mut content.interface.side_window.dirblocks;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if let Some(index) = gui.click_blocks(systems, mouse_pos) {
        gui.blocks[index].set_click(systems, true);
        return true;
    }

    false
}
