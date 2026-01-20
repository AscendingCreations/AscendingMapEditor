use graphics::*;

use crate::{
    content::{
        interface::side_window::zones,
        widget::{Textbox, checkbox::*, create_label, scrollbar::*},
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct ZoneWindow {
    pub visible: bool,
    pub zones: Vec<Checkbox>,
    lower_bg: GfxType,
    seperator: GfxType,
    pub scrollbar: Scrollbar,
    label: Vec<GfxType>,
    pub textbox: Vec<Textbox>,
    textbox_bg: Vec<GfxType>,

    pub cur_zone: usize,
    pub cur_textbox: Option<usize>,
    content_y_size: f32,
    start_pos: Vec2,
    area_size: Vec2,
}

impl ZoneWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let mut zones = Vec::with_capacity(5);

        let checkbox_rect = CheckboxRect {
            rect_color: Color::rgb(150, 150, 150),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 10.0,
            hover_change: CheckboxChangeType::ColorChange(Color::rgb(180, 180, 180)),
            click_change: CheckboxChangeType::ColorChange(Color::rgb(120, 120, 120)),
            disable_change: CheckboxChangeType::None,
        };

        let check_rect = CheckRect {
            rect_color: Color::rgb(90, 90, 90),
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 8.0,
            pos: Vec2::new(3.0, 3.0),
            size: Vec2::new(14.0, 14.0),
        };

        let zone_pos = Vec2::new(
            start_pos.x,
            start_pos.y + (area_size.y - (120.0 * systems.scale as f32).floor()),
        );

        for i in 0..5 {
            zones.push(Checkbox::new(
                systems,
                CheckboxType::Rect(checkbox_rect),
                CheckType::SetRect(check_rect),
                zone_pos,
                Vec2::new(5.0, 22.0 * (4 - i) as f32),
                ORDER_WINDOW_CONTENT,
                Vec2::new(20.0, 20.0),
                RENDER_GUI,
                1,
                RENDER_GUI,
                2,
                Some(CheckboxText {
                    text: format!("Zone {}", i + 1),
                    offset_pos: Vec2::new(3.0, 0.0),
                    buffer_layer: RENDER_GUI_TEXT,
                    order_layer: 2,
                    label_size: Vec2::new(100.0, 20.0),
                    color: Color::rgb(255, 255, 255),
                    hover_change: CheckboxChangeType::None,
                    click_change: CheckboxChangeType::None,
                    disable_change: CheckboxChangeType::None,
                }),
                false,
                None,
            ));
        }

        zones[0].set_value(systems, true);

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
            Vec2::new(area_size.x, start_pos.y),
            Color::rgb(130, 130, 130),
            0,
        );
        let lower_bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI2, "BG", false, CameraView::SubView1);

        let separator_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(
                zone_pos.x + (10.0 * systems.scale as f32).floor(),
                zone_pos.y - (10.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT2,
            ),
            Vec2::new(
                area_size.x - (40.0 * systems.scale as f32).floor(),
                (2.0 * systems.scale as f32).floor(),
            ),
            Color::rgb(140, 140, 140),
            1,
        );
        let seperator = systems.gfx.add_rect(
            separator_rect,
            RENDER_GUI,
            "Seperator",
            false,
            CameraView::SubView1,
        );

        let content_y_size = (368.0 * systems.scale as f32).floor();
        let scroll_value = (content_y_size - area_size.y).max(0.0) as usize;

        let bar_size = (area_size.y / systems.scale as f32).floor() - 20.0;
        let min_bar_size = (bar_size * 0.4).floor();
        let scrollbar = Scrollbar::new(
            systems,
            start_pos + Vec2::new(area_size.x - (14.0 * systems.scale as f32).floor(), 0.0),
            Vec2::new(0.0, 10.0),
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

        let mut label = Vec::with_capacity(8);
        let mut textbox = Vec::with_capacity(6);
        let mut textbox_bg = Vec::with_capacity(6);
        for i in 0..8 {
            let text_pos = Vec3::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_pos.y
                    + (area_size.y - ((156.0 + (26.0 * i as f32)) * systems.scale as f32).floor()),
                ORDER_WINDOW_CONTENT,
            );
            let text_size = Vec2::new(
                area_size.x - (40.0 * systems.scale as f32).floor(),
                (20.0 * systems.scale as f32).floor(),
            );
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
            let text_gfx = systems.gfx.add_text(
                text,
                RENDER_GUI_TEXT,
                "Zone Labels",
                false,
                CameraView::SubView1,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                &text_gfx,
                &match i {
                    0 => "Properties".to_string(),
                    1 => "Max NPC:".to_string(),
                    2 => "NPC ID".to_string(),
                    _ => format!("{}:", i - 2),
                },
            );

            if matches!(i, 0 | 2) {
                systems.gfx.center_text(&text_gfx);
            } else {
                let textbox_pos = Vec3::new(
                    text_pos.x + (85.0 * systems.scale as f32).floor(),
                    text_pos.y,
                    ORDER_WINDOW_CONTENT,
                );
                let textbox_size = Vec2::new(90.0, 20.0);

                let mut textbox_data = Textbox::new(
                    systems,
                    textbox_pos,
                    Vec2::new(0.0, 0.0),
                    textbox_size,
                    Color::rgb(255, 255, 255),
                    RENDER_GUI,
                    RENDER_GUI_TEXT,
                    [2, 3, 4],
                    255,
                    Color::rgb(110, 110, 110),
                    Color::rgb(150, 150, 150),
                    false,
                    false,
                    None,
                    vec![],
                    true,
                );
                textbox_data.set_text(systems, "0".to_string());
                textbox_data.set_select(systems, false);
                textbox_data.set_hold(false);
                textbox.push(textbox_data);

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(
                        textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                        textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                        ORDER_WINDOW_CONTENT,
                    ),
                    ((textbox_size + Vec2::new(2.0, 2.0)) * systems.scale as f32).floor(),
                    Color::rgb(70, 70, 70),
                    1,
                );
                let bg = systems
                    .gfx
                    .add_rect(rect, RENDER_GUI, "BG", false, CameraView::SubView1);
                textbox_bg.push(bg);
            }

            label.push(text_gfx);
        }

        ZoneWindow {
            visible: false,
            zones,
            cur_zone: 0,
            lower_bg,
            seperator,
            scrollbar,
            label,
            textbox,
            textbox_bg,
            cur_textbox: None,
            content_y_size,
            start_pos,
            area_size,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        self.start_pos = start_pos;
        self.area_size = area_size;

        let zone_pos = Vec2::new(
            start_pos.x,
            start_pos.y + (area_size.y - (120.0 * systems.scale as f32).floor()),
        );

        for zones in self.zones.iter_mut() {
            zones.set_pos(systems, zone_pos);
        }

        systems
            .gfx
            .set_size(&self.lower_bg, Vec2::new(area_size.x, start_pos.y));
        systems.gfx.set_pos(
            &self.lower_bg,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
        );

        systems.gfx.set_pos(
            &self.seperator,
            Vec3::new(
                zone_pos.x + (10.0 * systems.scale as f32).floor(),
                zone_pos.y - (10.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT2,
            ),
        );

        let scroll_value = (self.content_y_size - area_size.y).max(0.0) as usize;
        let bar_size = (area_size.y / systems.scale as f32).floor() - 20.0;
        let min_bar_size = (bar_size * 0.4).floor();
        self.scrollbar.set_pos(
            systems,
            start_pos + Vec2::new(area_size.x - (14.0 * systems.scale as f32).floor(), 0.0),
        );
        self.scrollbar
            .set_size(systems, bar_size, min_bar_size, 10.0);
        self.scrollbar.set_value(systems, 0);
        self.scrollbar.set_max_value(systems, scroll_value);

        let mut loop_count = 0;
        for (i, label) in self.label.iter().enumerate() {
            let text_pos = Vec3::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_pos.y
                    + (area_size.y - ((156.0 + (26.0 * i as f32)) * systems.scale as f32).floor()),
                ORDER_WINDOW_CONTENT,
            );
            let text_size = Vec2::new(
                area_size.x - (40.0 * systems.scale as f32).floor(),
                (20.0 * systems.scale as f32).floor(),
            );

            systems.gfx.set_pos(label, text_pos);
            systems.gfx.set_bound(
                label,
                Some(Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                )),
            );

            if matches!(i, 0 | 2) {
                systems.gfx.center_text(label);
            } else {
                let textbox_pos = Vec2::new(
                    text_pos.x + (85.0 * systems.scale as f32).floor(),
                    text_pos.y,
                );

                self.textbox[loop_count].set_pos(systems, textbox_pos);
                systems.gfx.set_pos(
                    &self.textbox_bg[loop_count],
                    Vec3::new(
                        textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                        textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                        ORDER_WINDOW_CONTENT,
                    ),
                );

                loop_count += 1;
            }
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        for checkbox in self.zones.iter_mut() {
            checkbox.set_visible(systems, visible);
        }
        systems.gfx.set_visible(&self.lower_bg, visible);
        systems.gfx.set_visible(&self.seperator, visible);
        self.scrollbar.set_visible(systems, visible);
        for gfx in self.label.iter() {
            systems.gfx.set_visible(gfx, visible);
        }
        for textbox in self.textbox.iter_mut() {
            textbox.set_visible(systems, visible);
        }
        for gfx in self.textbox_bg.iter() {
            systems.gfx.set_visible(gfx, visible);
        }
    }

    pub fn update_content(&mut self, systems: &mut SystemHolder) {
        let zone_pos = Vec2::new(
            self.start_pos.x,
            self.start_pos.y
                + (self.area_size.y - (120.0 * systems.scale as f32).floor())
                + self.scrollbar.value as f32,
        );

        for zones in self.zones.iter_mut() {
            zones.set_pos(systems, zone_pos);
        }

        systems.gfx.set_pos(
            &self.seperator,
            Vec3::new(
                zone_pos.x + (10.0 * systems.scale as f32).floor(),
                zone_pos.y - (10.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT2,
            ),
        );

        let mut loop_count = 0;
        for (i, label) in self.label.iter().enumerate() {
            let text_pos = Vec3::new(
                self.start_pos.x + (10.0 * systems.scale as f32).floor(),
                self.start_pos.y
                    + (self.area_size.y
                        - ((156.0 + (26.0 * i as f32)) * systems.scale as f32).floor())
                    + self.scrollbar.value as f32,
                ORDER_WINDOW_CONTENT,
            );
            let text_size = Vec2::new(
                self.area_size.x - (40.0 * systems.scale as f32).floor(),
                (20.0 * systems.scale as f32).floor(),
            );

            systems.gfx.set_pos(label, text_pos);
            systems.gfx.set_bound(
                label,
                Some(Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                )),
            );

            if matches!(i, 0 | 2) {
                systems.gfx.center_text(label);
            } else {
                let textbox_pos = Vec2::new(
                    text_pos.x + (85.0 * systems.scale as f32).floor(),
                    text_pos.y,
                );

                self.textbox[loop_count].set_pos(systems, textbox_pos);
                systems.gfx.set_pos(
                    &self.textbox_bg[loop_count],
                    Vec3::new(
                        textbox_pos.x - (1.0 * systems.scale as f32).floor(),
                        textbox_pos.y - (1.0 * systems.scale as f32).floor(),
                        ORDER_WINDOW_CONTENT,
                    ),
                );

                loop_count += 1;
            }
        }
    }
}
