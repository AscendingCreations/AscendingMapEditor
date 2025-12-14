use crate::{
    SystemHolder, content::interface::widget::create_label, data_types::*, gfx_collection::GfxType,
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;

pub struct Tooltip {
    window: GfxType,
    text: GfxType,
    visible: bool,
    init: bool,
    start_tmr: bool,
    init_pos: Vec2,
    init_tmr: f32,
}

impl Tooltip {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let visible = false;

        let mut window_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_TOOLTIP),
            Vec2::new(24.0, 24.0),
            Color::rgb(100, 100, 100),
            0,
        );
        window_rect
            .set_border_width(1.0)
            .set_border_color(Color::rgb(0, 0, 0))
            .set_radius(4.0);
        let window = systems.gfx.add_rect(
            window_rect,
            RENDER_TOOLTIP,
            "Tool Tips Window",
            visible,
            CameraView::SubView1,
        );

        let mut text_label = create_label(
            systems,
            Vec3::new(2.0, 2.0, ORDER_TOOLTIP),
            Vec2::new(20.0, 20.0),
            Bounds::new(2.0, 2.0, 22.0, 22.0),
            Color::rgb(255, 255, 255),
            1,
            16.0,
            16.0,
            true,
        );
        text_label.set_buffer_size(
            &mut systems.renderer,
            Some(300.0),
            Some(systems.size.height),
        );
        let text = systems.gfx.add_text(
            text_label,
            RENDER_TOOLTIP_TEXT,
            "Tool Tips Text",
            visible,
            CameraView::SubView1,
        );

        Tooltip {
            window,
            text,
            visible,

            init: false,
            start_tmr: false,
            init_pos: Vec2::new(0.0, 0.0),
            init_tmr: 0.0,
        }
    }

    pub fn init_tooltip(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
        msg: String,
        instant: bool,
    ) {
        self.start_tmr = true;
        self.init_pos = screen_pos;
        self.init_tmr = 0.0;
        self.init = true;
        self.hide_tooltip(systems);

        systems
            .gfx
            .set_text(&mut systems.renderer, &self.text, &msg);
        let text_size = systems.gfx.get_measure(&self.text) + Vec2::new(0.0, 4.0);
        let w_size = text_size + Vec2::new(16.0, 16.0);
        let w_pos = screen_pos;

        systems.gfx.set_size(&self.text, text_size);
        systems.gfx.set_bound(
            &self.text,
            Bounds::new(
                w_pos.x + 8.0,
                w_pos.y + 8.0,
                w_pos.x + text_size.x + 8.0,
                w_pos.y + text_size.y + 8.0,
            ),
        );
        systems.gfx.set_pos(
            &self.text,
            Vec3::new(w_pos.x + 8.0, w_pos.y + 8.0, ORDER_TOOLTIP),
        );
        systems
            .gfx
            .set_pos(&self.window, Vec3::new(w_pos.x, w_pos.y, ORDER_TOOLTIP));
        systems.gfx.set_size(&self.window, w_size);

        if instant {
            self.start_tmr = false;
            self.init_tmr = 0.0;
        }
    }

    pub fn hide_tooltip(&mut self, systems: &mut SystemHolder) {
        if !self.visible {
            return;
        }
        self.visible = false;
        systems.gfx.set_visible(&self.window, self.visible);
        systems.gfx.set_visible(&self.text, self.visible);
    }

    pub fn show_tooltip(&mut self, systems: &mut SystemHolder) {
        if self.visible {
            return;
        }
        self.visible = true;
        systems.gfx.set_visible(&self.window, self.visible);
        systems.gfx.set_visible(&self.text, self.visible);
    }

    pub fn check_tooltip(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if self.init_pos != screen_pos {
            self.init = false;
            self.hide_tooltip(systems);
        }
    }

    pub fn handle_tooltip_logic(&mut self, systems: &mut SystemHolder, seconds: f32) {
        if !self.init || self.visible {
            return;
        }

        if self.start_tmr {
            self.init_tmr = seconds + 1.0;
            self.start_tmr = false;
        } else if self.init_tmr <= seconds {
            self.init = false;
            self.show_tooltip(systems);
        }
    }
}
