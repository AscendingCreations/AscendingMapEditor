use graphics::*;

use crate::{
    content::widget::{Textbox, create_label, scrollbar::*},
    data_types::*,
    database::{ItemSpawnData, MapPosition, Position},
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct InputTextbox {
    label: GfxType,
    bg: GfxType,
    pub textbox: Textbox,
}

pub struct AttrItemSpawn {
    pub visible: bool,
    pub input_box: Vec<InputTextbox>,
    pub content_scroll: Scrollbar,
    pub cur_textbox: Option<usize>,
    content_y_size: f32,
    start_pos: Vec2,
    area_size: Vec2,
}

impl AttrItemSpawn {
    pub fn new(
        systems: &mut SystemHolder,
        start_pos: Vec2,
        area_size: Vec2,
        visible: bool,
    ) -> Self {
        let mut content_y_size = 0.0;
        let mut start_y_pos = start_pos.y + area_size.y - (30.0 * systems.scale as f32).floor();

        let mut input_box = Vec::with_capacity(3);
        for i in 0..3 {
            let text_pos = Vec3::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            let text_size = (Vec2::new(100.0, 20.0) * systems.scale as f32).floor();
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
            let gfx = systems
                .gfx
                .add_text(text, RENDER_GUI_TEXT, "AttrItemSpawn Label", visible);
            systems.gfx.set_text(
                &mut systems.renderer,
                &gfx,
                match i {
                    1 => "Amount:",
                    2 => "Timer:",
                    _ => "Index:",
                },
            );

            let textbox_pos = Vec3::new(
                (116.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            let mut textbox_data = Textbox::new(
                systems,
                textbox_pos,
                Vec2::new(0.0, 0.0),
                Vec2::new(80.0, 20.0),
                Color::rgb(255, 255, 255),
                RENDER_GUI,
                RENDER_GUI_TEXT,
                [2, 3, 4],
                255,
                Color::rgb(110, 110, 110),
                Color::rgb(150, 150, 150),
                false,
                visible,
                None,
                vec![],
                true,
            );
            textbox_data.set_text(systems, "0".to_string());
            textbox_data.set_select(systems, false);
            textbox_data.set_hold(false);

            let rect = Rect::new(
                &mut systems.renderer,
                Vec3::new(
                    textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                    textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                    ORDER_WINDOW_CONTENT,
                ),
                (Vec2::new(82.0, 22.0) * systems.scale as f32).floor(),
                Color::rgb(70, 70, 70),
                1,
            );
            let bg = systems.gfx.add_rect(rect, RENDER_GUI, "BG", visible);

            content_y_size += (30.0 * systems.scale as f32).floor();
            start_y_pos -= (30.0 * systems.scale as f32).floor();

            input_box.push(InputTextbox {
                label: gfx,
                textbox: textbox_data,
                bg,
            })
        }

        content_y_size += (30.0 * systems.scale as f32).floor();

        let bar_size = area_size.y - 10.0;
        let min_bar_size = (bar_size * 0.2).floor();

        let scroll_value = (content_y_size - area_size.y).max(0.0) as usize;

        let content_scroll = Scrollbar::new(
            systems,
            start_pos + Vec2::new(area_size.x - (15.0 * systems.scale as f32).floor(), 5.0),
            Vec2::new(0.0, 0.0),
            bar_size,
            10.0,
            true,
            ORDER_WINDOW_CONTENT,
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 3,
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                hover_color: Color::rgb(180, 180, 180),
                hold_color: Color::rgb(120, 120, 120),
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgb(90, 90, 90),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            scroll_value,
            min_bar_size,
            false,
            false,
            None,
            true,
            None,
        );

        AttrItemSpawn {
            visible,
            content_scroll,
            input_box,
            cur_textbox: None,
            content_y_size,
            start_pos,
            area_size,
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;

        for input_box in self.input_box.iter_mut() {
            systems.gfx.set_visible(&input_box.bg, visible);
            systems.gfx.set_visible(&input_box.label, visible);
            input_box.textbox.set_visible(systems, visible);
        }
        self.content_scroll.set_visible(systems, visible);
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        self.content_scroll.set_pos(
            systems,
            start_pos + Vec2::new(area_size.x - (15.0 * systems.scale as f32).floor(), 5.0),
        );

        self.start_pos = start_pos;
        self.area_size = area_size;

        let mut content_y_size = 0.0;
        let mut start_y_pos = start_pos.y + area_size.y - (30.0 * systems.scale as f32).floor();

        for inputbox in self.input_box.iter_mut() {
            let text_pos = Vec3::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            let text_size = (Vec2::new(100.0, 20.0) * systems.scale as f32).floor();

            systems.gfx.set_pos(&inputbox.label, text_pos);
            systems.gfx.set_bound(
                &inputbox.label,
                Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                ),
            );

            let textbox_pos = Vec3::new(
                (116.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            inputbox
                .textbox
                .set_pos(systems, Vec2::new(textbox_pos.x, textbox_pos.y));

            systems.gfx.set_pos(
                &inputbox.bg,
                Vec3::new(
                    textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                    textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                    ORDER_WINDOW_CONTENT,
                ),
            );

            content_y_size += (30.0 * systems.scale as f32).floor();
            start_y_pos -= (30.0 * systems.scale as f32).floor();
        }

        content_y_size += (30.0 * systems.scale as f32).floor();

        self.content_y_size = content_y_size;

        let bar_size = (area_size.y / systems.scale as f32).floor() - 10.0;
        let min_bar_size = (bar_size * 0.2).floor().max(4.0);

        self.content_scroll
            .set_size(systems, bar_size, min_bar_size, 10.0);
        let scroll_value = (content_y_size - area_size.y).max(0.0) as usize;

        self.content_scroll.set_value(systems, 0);
        self.content_scroll.set_max_value(systems, scroll_value);
    }

    pub fn update_content(&mut self, systems: &mut SystemHolder) {
        let mut start_y_pos = (self.start_pos.y + self.area_size.y
            - (30.0 * systems.scale as f32).floor())
            + self.content_scroll.value as f32;

        for inputbox in self.input_box.iter_mut() {
            let text_pos = Vec3::new(
                self.start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            let text_size = (Vec2::new(100.0, 20.0) * systems.scale as f32).floor();

            systems.gfx.set_pos(&inputbox.label, text_pos);
            systems.gfx.set_bound(
                &inputbox.label,
                Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                ),
            );

            let textbox_pos = Vec3::new(
                (116.0 * systems.scale as f32).floor(),
                start_y_pos,
                ORDER_WINDOW_CONTENT,
            );
            inputbox
                .textbox
                .set_pos(systems, Vec2::new(textbox_pos.x, textbox_pos.y));

            systems.gfx.set_pos(
                &inputbox.bg,
                Vec3::new(
                    textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                    textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                    ORDER_WINDOW_CONTENT,
                ),
            );

            start_y_pos -= (30.0 * systems.scale as f32).floor();
        }
    }

    pub fn get_value(&self) -> ItemSpawnData {
        ItemSpawnData {
            index: self.input_box[0]
                .textbox
                .text
                .parse::<u32>()
                .unwrap_or_default(),
            amount: self.input_box[1]
                .textbox
                .text
                .parse::<u16>()
                .unwrap_or_default(),
            timer: self.input_box[2]
                .textbox
                .text
                .parse::<u64>()
                .unwrap_or_default(),
        }
    }
}
