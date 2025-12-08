use graphics::*;

use crate::renderer::SystemHolder;

pub struct PropertiesWindow {
    pub visible: bool,
}

impl PropertiesWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        PropertiesWindow { visible: false }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {}

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
    }
}
