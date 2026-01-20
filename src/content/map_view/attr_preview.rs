use graphics::*;

use crate::{
    content::widget::{create_label, measure_string},
    data_types::*,
    database::MapAttribute,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

#[derive(Default)]
pub struct AttrPreview {
    pub bg: GfxType,
    pub text: Vec<GfxType>,
    cur_attr: MapAttribute,
    size: Vec2,
    show_timer: f32,
    visible: bool,
    loaded: bool,
    pub in_drag: bool,
}

impl AttrPreview {
    pub fn show_attr(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        attr: &MapAttribute,
        seconds: f32,
    ) {
        self.visible = false;
        self.show_timer = seconds + 0.5;
        self.loaded = false;

        if self.loaded && self.cur_attr.eq(attr) {
            self.update_pos(systems, mouse_pos);
            return;
        }

        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        for gfx in self.text.iter() {
            systems.gfx.remove_gfx(&mut systems.renderer, gfx);
        }

        let msgs = match attr {
            MapAttribute::Blocked => {
                vec!["Blocked".to_string()]
            }
            MapAttribute::ItemSpawn(data) => {
                vec![
                    "Item Spawn".to_string(),
                    format!("Index: {}", data.index),
                    format!("Amount: {}", data.amount),
                    format!("Respawn Time: {}ms", data.timer),
                ]
            }
            MapAttribute::NpcBlocked => {
                vec!["Npc Blocked".to_string()]
            }
            MapAttribute::Shop(data) => {
                vec!["Shop".to_string(), format!("Index: {data}")]
            }
            MapAttribute::Sign(data) => {
                vec!["Sign".to_string(), format!("Msg: {data}")]
            }
            MapAttribute::Storage => {
                vec!["Storage".to_string()]
            }
            MapAttribute::Warp(data) => {
                vec![
                    "Warp".to_string(),
                    format!("Tile: {}x{}", data.tile_x, data.tile_y),
                    format!("Map Pos: {}x{}", data.map_x, data.map_y),
                    format!("Group: {}", data.map_group),
                ]
            }
            MapAttribute::Walkable | MapAttribute::Count => return,
        };

        let mut width = 0.0;
        for msg in msgs.iter() {
            let check_width = measure_string(systems, msg, true, 16.0, 16.0).x;
            if width < check_width {
                width = check_width;
            }
        }
        let size = Vec2::new(
            width + (10.0 * systems.scale as f32).floor(),
            (((msgs.len() as f32 * 20.0) + 10.0) * systems.scale as f32).floor(),
        );
        let pos = mouse_pos + Vec2::new(-(size.x * 0.5).floor(), 10.0);

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(pos.x, pos.y, ORDER_ATTR_PREVIEW),
            size,
            Color::rgb(100, 100, 100),
            0,
        );
        rect.set_border_color(Color::rgb(0, 0, 0))
            .set_border_width(1.0);
        self.bg = systems.gfx.add_rect(
            rect,
            RENDER_GUI,
            "Preview Attr BG",
            false,
            CameraView::SubView1,
        );

        self.text.clear();
        for (i, msg) in msgs.iter().enumerate() {
            let label_pos = Vec2::new(
                pos.x,
                (pos.y + size.y) - (((20.0 * (i + 1) as f32) + 5.0) * systems.scale as f32).floor(),
            );
            let label_size = Vec2::new(size.x, (20.0 * systems.scale as f32).floor());
            let label = create_label(
                systems,
                Vec3::new(label_pos.x, label_pos.y, ORDER_ATTR_PREVIEW),
                label_size,
                Bounds::new(
                    label_pos.x,
                    label_pos.y,
                    label_pos.x + label_size.x,
                    label_pos.y + label_size.y,
                ),
                Color::rgb(255, 255, 255),
                1,
                16.0,
                16.0,
                true,
            );
            let lbl = systems.gfx.add_text(
                label,
                RENDER_GUI_TEXT,
                "Preview Attr Label",
                false,
                CameraView::SubView1,
            );

            systems.gfx.set_text(&mut systems.renderer, &lbl, msg);
            systems.gfx.center_text(&lbl);
            self.text.push(lbl);
        }

        self.cur_attr = attr.clone();
        self.size = size;
        self.loaded = true;
    }

    pub fn update_pos(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        let pos = mouse_pos + Vec2::new(-(self.size.x * 0.5).floor(), 10.0);
        systems
            .gfx
            .set_pos(&self.bg, Vec3::new(pos.x, pos.y, ORDER_ATTR_PREVIEW));

        for (i, gfx) in self.text.iter().enumerate() {
            systems.gfx.set_visible(gfx, false);

            let label_pos = Vec2::new(
                pos.x,
                (pos.y + self.size.y)
                    - (((20.0 * (i + 1) as f32) + 5.0) * systems.scale as f32).floor(),
            );
            let label_size = Vec2::new(self.size.x, (20.0 * systems.scale as f32).floor());

            systems
                .gfx
                .set_pos(gfx, Vec3::new(label_pos.x, label_pos.y, ORDER_ATTR_PREVIEW));
            systems.gfx.set_bound(
                gfx,
                Some(Bounds::new(
                    label_pos.x,
                    label_pos.y,
                    label_pos.x + label_size.x,
                    label_pos.y + label_size.y,
                )),
            );
            systems.gfx.center_text(gfx);
        }
    }

    pub fn clear_attr(&mut self, systems: &mut SystemHolder, seconds: f32) {
        systems.gfx.set_visible(&self.bg, false);
        for gfx in self.text.iter() {
            systems.gfx.set_visible(gfx, false);
        }
        self.visible = false;
        self.show_timer = seconds + 0.5;
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        for gfx in self.text.iter() {
            systems.gfx.remove_gfx(&mut systems.renderer, gfx);
        }
        self.visible = false;
        self.loaded = false;
    }

    pub fn update_visible(&mut self, systems: &mut SystemHolder, seconds: f32) {
        if !self.loaded || self.in_drag || self.visible || self.show_timer > seconds {
            return;
        }

        systems.gfx.set_visible(&self.bg, true);
        for gfx in self.text.iter() {
            systems.gfx.set_visible(gfx, true);
        }

        self.visible = true;
    }
}
