use graphics::*;

use crate::{
    content::widget::{create_label, measure_string},
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct NotificationData {
    bg: GfxType,
    text: GfxType,

    size: Vec2,

    timer: f32,
    cur_y: usize,
    target_y: usize,
}

#[derive(Default)]
pub struct Notification {
    pub data: Vec<NotificationData>,
}

impl Notification {
    pub fn add_msg(&mut self, systems: &mut SystemHolder, msg: String, seconds: f32) {
        let msg_size = measure_string(systems, &msg, true, 16.0, 16.0).floor();
        let window_size = msg_size + (Vec2::new(20.0, 10.0) * systems.scale as f32).floor();
        let window_pos = Vec2::new(
            systems.size.width - (window_size.x + (10.0 * systems.scale as f32).floor()),
            (35.0 * systems.scale as f32).floor(),
        );

        for data in self.data.iter_mut() {
            data.target_y += 30;
        }

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(window_pos.x, window_pos.y, ORDER_NOTIFICATION),
            window_size,
            Color::rgb(100, 100, 100),
            0,
        );
        rect.set_border_width(1.0)
            .set_border_color(Color::rgb(0, 0, 0))
            .set_radius(5.0);
        let bg = systems.gfx.add_rect(
            rect,
            RENDER_NOTIFICATION,
            "Notification Window",
            true,
            CameraView::SubView1,
        );

        let text_pos = Vec2::new(
            window_pos.x,
            window_pos.y + (5.0 * systems.scale as f32).floor(),
        );
        let text_size = Vec2::new(window_size.x, (20.0 * systems.scale as f32).floor());
        let txt = create_label(
            systems,
            Vec3::new(text_pos.x, text_pos.y, ORDER_NOTIFICATION),
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
        let text = systems.gfx.add_text(
            txt,
            RENDER_NOTIFICATION_TEXT,
            "Notification Text",
            true,
            CameraView::SubView1,
        );
        systems.gfx.set_text(&mut systems.renderer, &text, &msg);
        systems.gfx.center_text(&text);

        self.data.push(NotificationData {
            bg,
            text,
            size: window_size,
            timer: seconds + 5.0,
            cur_y: 0,
            target_y: 0,
        });
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        for data in self.data.iter_mut() {
            let window_size = data.size;
            let window_pos = Vec2::new(
                systems.size.width - (window_size.x + (10.0 * systems.scale as f32).floor()),
                (35.0 * systems.scale as f32).floor() + data.cur_y as f32,
            );

            systems.gfx.set_pos(
                &data.bg,
                Vec3::new(window_pos.x, window_pos.y, ORDER_NOTIFICATION),
            );

            let text_pos = Vec2::new(
                window_pos.x,
                window_pos.y + (5.0 * systems.scale as f32).floor(),
            );
            let text_size = Vec2::new(window_size.x, (20.0 * systems.scale as f32).floor());

            systems.gfx.set_pos(
                &data.text,
                Vec3::new(text_pos.x, text_pos.y, ORDER_NOTIFICATION),
            );
            systems.gfx.set_bound(
                &data.text,
                Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                ),
            );
            systems.gfx.center_text(&data.text);
        }
    }

    pub fn update(&mut self, systems: &mut SystemHolder, seconds: f32) {
        let mut to_remove = Vec::with_capacity(self.data.len());

        for (index, data) in self.data.iter_mut().enumerate() {
            if data.timer <= seconds {
                to_remove.push(index);
            }

            if data.cur_y < data.target_y {
                data.cur_y += 3;

                let window_size = data.size;
                let window_pos = Vec2::new(
                    systems.size.width - (window_size.x + (10.0 * systems.scale as f32).floor()),
                    (35.0 * systems.scale as f32).floor() + data.cur_y as f32,
                );

                systems.gfx.set_pos(
                    &data.bg,
                    Vec3::new(window_pos.x, window_pos.y, ORDER_NOTIFICATION),
                );

                let text_pos = Vec2::new(
                    window_pos.x,
                    window_pos.y + (5.0 * systems.scale as f32).floor(),
                );
                let text_size = Vec2::new(window_size.x, (20.0 * systems.scale as f32).floor());

                systems.gfx.set_pos(
                    &data.text,
                    Vec3::new(text_pos.x, text_pos.y, ORDER_NOTIFICATION),
                );
                systems.gfx.set_bound(
                    &data.text,
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + text_size.x,
                        text_pos.y + text_size.y,
                    ),
                );
                systems.gfx.center_text(&data.text);
            }
        }

        for index in to_remove.iter().rev() {
            let data = self.data.remove(*index);
            systems.gfx.remove_gfx(&mut systems.renderer, &data.bg);
            systems.gfx.remove_gfx(&mut systems.renderer, &data.text);
        }
    }
}
