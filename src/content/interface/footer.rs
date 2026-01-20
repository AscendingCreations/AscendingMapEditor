use graphics::*;

use crate::{
    content::widget::{create_label, measure_string},
    data_types::{ORDER_MENU_BAR, RENDER_GUI, RENDER_GUI_TEXT},
    database::MapPosition,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct Footer {
    bg: GfxType,
    map_pos: GfxType,
    tile_pos: GfxType,
}

impl Footer {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let pos = Vec3::new((254.0 * systems.scale as f32).floor(), 0.0, ORDER_MENU_BAR);

        let rect = Rect::new(
            &mut systems.renderer,
            pos,
            Vec2::new(
                systems.size.width - (254.0 * systems.scale as f32).floor(),
                (20.0 * systems.scale as f32).floor(),
            ),
            Color::rgb(110, 110, 110),
            0,
        );
        let bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Footer BG", true, CameraView::SubView1);

        let text_pos = pos + Vec3::new((10.0 * systems.scale as f32).floor(), 0.0, 0.0);
        let text_size = (Vec2::new(500.0, 20.0) * systems.scale as f32).floor();

        let text = create_label(
            systems,
            text_pos,
            text_size,
            Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            ),
            Color::rgb(255, 255, 255),
            1,
            16.0,
            16.0,
            true,
        );
        let map_pos = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Map Pos Text",
            true,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &map_pos, "Unsaved");

        let message = "Tile Pos [X: 00 Y: 00]".to_string();
        let text_width = measure_string(systems, &message, true, 16.0, 16.0).x;

        let text_pos = Vec3::new(
            systems.size.width - text_width - (10.0 * systems.scale as f32).floor(),
            0.0,
            0.0,
        );
        let text_size = Vec2::new(text_width, (20.0) * systems.scale as f32).floor();

        let text = create_label(
            systems,
            text_pos,
            text_size,
            Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            ),
            Color::rgb(255, 255, 255),
            1,
            16.0,
            16.0,
            true,
        );
        let tile_pos = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Map Pos Text",
            true,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &tile_pos, &message);

        Footer {
            bg,
            map_pos,
            tile_pos,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        let pos = Vec3::new((254.0 * systems.scale as f32).floor(), 0.0, ORDER_MENU_BAR);

        systems.gfx.set_pos(&self.bg, pos);
        systems.gfx.set_size(
            &self.bg,
            Vec2::new(
                systems.size.width - (254.0 * systems.scale as f32).floor(),
                (20.0 * systems.scale as f32).floor(),
            ),
        );

        let text_pos = pos + Vec3::new((10.0 * systems.scale as f32).floor(), 0.0, 0.0);
        let text_size = Vec2::new(500.0, 20.0);

        systems.gfx.set_pos(&self.map_pos, text_pos);
        systems.gfx.set_bound(
            &self.map_pos,
            Some(Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            )),
        );

        let message = "Tile Pos [X: 00 Y: 00]".to_string();
        let text_width = measure_string(systems, &message, true, 16.0, 16.0).x;

        let text_pos = Vec3::new(
            systems.size.width - text_width - (10.0 * systems.scale as f32).floor(),
            0.0,
            0.0,
        );
        let text_size = Vec2::new(text_width, (20.0) * systems.scale as f32).floor();

        systems.gfx.set_pos(&self.tile_pos, text_pos);
        systems.gfx.set_bound(
            &self.tile_pos,
            Some(Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            )),
        );
    }

    pub fn set_map_pos(&mut self, systems: &mut SystemHolder, map_pos: MapPosition, saved: bool) {
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.map_pos,
            &format!(
                "Map [X: {} Y: {} Group: {}]{}",
                map_pos.x,
                map_pos.y,
                map_pos.group,
                if saved { "" } else { "*" }
            ),
        );
    }

    pub fn set_tile_pos(&mut self, systems: &mut SystemHolder, tile_pos: (u32, u32)) {
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.tile_pos,
            &format!("Tile Pos [X: {:02} Y: {:02}]", tile_pos.0, tile_pos.1),
        );
    }

    pub fn remove_map_pos(&mut self, systems: &mut SystemHolder) {
        systems
            .gfx
            .set_text(&mut systems.renderer, &self.map_pos, "Unsaved");
    }
}
