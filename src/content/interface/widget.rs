use graphics::*;
use winit::dpi::PhysicalSize;

pub mod alert;
pub mod button;
pub mod checkbox;
pub mod label;
pub mod option_list;
pub mod scrollbar;
pub mod text_list;
pub mod textbox;
pub mod tooltip;

pub use alert::*;
pub use button::*;
pub use checkbox::*;
pub use label::*;
pub use option_list::*;
pub use scrollbar::*;
pub use text_list::*;
pub use textbox::*;
pub use tooltip::*;

use crate::{content::Content, data_types::TEXTURE_SIZE, renderer::SystemHolder};

pub fn is_within_area(area: Vec2, target_pos: Vec2, target_size: Vec2) -> bool {
    area.x >= target_pos.x
        && area.x <= target_pos.x + target_size.x
        && area.y >= target_pos.y
        && area.y <= target_pos.y + target_size.y
}

pub fn get_screen_center(size: &PhysicalSize<f32>) -> Vec2 {
    Vec2::new(size.width * 0.5, size.height * 0.5)
}

pub fn in_view_screen(systems: &SystemHolder, screen_pos: Vec2) -> bool {
    let start_pos = (Vec2::new(254.0, 0.0) * systems.scale as f32).floor();
    let view_size = Vec2::new(
        systems.size.width - (254.0 * systems.scale as f32).floor(),
        systems.size.height - (20.0 * systems.scale as f32).floor(),
    );
    is_within_area(screen_pos, start_pos, view_size)
}

pub fn in_layer_area(systems: &SystemHolder, screen_pos: Vec2) -> bool {
    let size = Vec2::new(48.0, 137.0);
    let pos = Vec2::new(systems.size.width - size.x, 0.0);
    is_within_area(screen_pos, pos, size)
}

pub fn in_drawing_area(content: &Content, systems: &SystemHolder, screen_pos: Vec2) -> bool {
    let start_pos = (content.map_view.map.pos * systems.config.zoom).floor();
    let tile_size = (Vec2::new(32.0 * TEXTURE_SIZE as f32, 32.0 * TEXTURE_SIZE as f32)
        * systems.config.zoom)
        .floor();
    is_within_area(screen_pos, start_pos, tile_size)
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ScaleVec2 {
    pub val: Vec2,
    pub scale: bool,
}

impl ScaleVec2 {
    pub fn new(x: f32, y: f32, scale: bool) -> Self {
        ScaleVec2 {
            val: Vec2::new(x, y),
            scale,
        }
    }

    pub fn get_val(&self, systems: &SystemHolder) -> Vec2 {
        (self.val
            * if self.scale {
                systems.scale as f32
            } else {
                1.0
            })
        .floor()
    }

    pub fn get_origin(&self) -> Vec2 {
        self.val
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ScaleVec3 {
    pub val: Vec3,
    pub scale: bool,
}

impl ScaleVec3 {
    pub fn new(x: f32, y: f32, z: f32, scale: bool) -> Self {
        ScaleVec3 {
            val: Vec3::new(x, y, z),
            scale,
        }
    }

    pub fn get_val(&self, systems: &SystemHolder) -> Vec3 {
        Vec3::new(
            (self.val.x
                * if self.scale {
                    systems.scale as f32
                } else {
                    1.0
                })
            .floor(),
            (self.val.y
                * if self.scale {
                    systems.scale as f32
                } else {
                    1.0
                })
            .floor(),
            self.val.z,
        )
    }

    pub fn get_origin(&self) -> Vec3 {
        self.val
    }
}
