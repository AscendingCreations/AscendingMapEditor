use graphics::*;

use crate::{
    content::widget::{button::*, create_label, textbox::*},
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub enum MapPosInputType {
    None,
    LoadMap,
    SaveMap,
}

pub struct MapPosInput {
    pub visible: bool,
    black: GfxType,
    bg: GfxType,
    label: Vec<GfxType>,
    label_bg: Vec<GfxType>,
    pub textbox: Vec<Textbox>,
    pub button: Vec<Button>,
    pub cur_textbox: Option<usize>,
    pub input_type: MapPosInputType,
}

impl MapPosInput {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let screen_size = Vec2::new(systems.size.width, systems.size.height);

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_MAPPOS_BG),
            screen_size,
            Color::rgba(0, 0, 0, 150),
            0,
        );
        let black = systems.gfx.add_rect(
            rect,
            RENDER_MAPPOS_GUI,
            "Map Pos Shadow",
            false,
            CameraView::SubView1,
        );

        let bg_size = (Vec2::new(350.0, 120.0) * systems.scale as f32).floor();
        let bg_pos = ((screen_size - bg_size) * 0.5).floor();

        let mut window = Rect::new(
            &mut systems.renderer,
            Vec3::new(bg_pos.x, bg_pos.y, ORDER_MAPPOS),
            bg_size,
            Color::rgb(100, 100, 100),
            1,
        );
        window
            .set_border_width(1.0)
            .set_border_color(Color::rgb(0, 0, 0));
        let bg = systems.gfx.add_rect(
            window,
            RENDER_MAPPOS_GUI,
            "Map Pos Window",
            false,
            CameraView::SubView1,
        );

        let mut label = Vec::with_capacity(4);
        let mut textbox = Vec::with_capacity(3);
        let mut label_bg = Vec::with_capacity(3);

        let mut add_x = 0.0;
        for i in 0..4 {
            let (text_pos, text_size) = if i == 3 {
                (
                    Vec3::new(
                        bg_pos.x + (10.0 * systems.scale as f32).floor(),
                        bg_pos.y + (80.0 * systems.scale as f32).floor(),
                        ORDER_MAPPOS,
                    ),
                    Vec2::new(
                        bg_size.x - (20.0 * systems.scale as f32).floor(),
                        (20.0 * systems.scale as f32).floor(),
                    ),
                )
            } else {
                (
                    Vec3::new(
                        bg_pos.x + (10.0 * systems.scale as f32).floor() + add_x,
                        bg_pos.y + (50.0 * systems.scale as f32).floor(),
                        ORDER_MAPPOS,
                    ),
                    (Vec2::new(50.0, 20.0) * systems.scale as f32).floor(),
                )
            };

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
                2,
                16.0,
                16.0,
                true,
            );
            let lbl = systems.gfx.add_text(
                text,
                RENDER_MAPPOS_TEXT,
                "Map Pos Text",
                false,
                CameraView::SubView1,
            );
            systems.gfx.set_text(
                &mut systems.renderer,
                &lbl,
                match i {
                    0 => "X:",
                    1 => "Y:",
                    2 => "Group:",
                    _ => "Input Map Position",
                },
            );
            systems.gfx.center_text(&lbl);
            label.push(lbl);

            match i {
                0 => add_x += (40.0 * systems.scale as f32).floor(),
                1 => add_x += (40.0 * systems.scale as f32).floor(),
                2 => add_x += (60.0 * systems.scale as f32).floor(),
                _ => {}
            }

            if i < 3 {
                let textbox_pos = Vec3::new(
                    bg_pos.x + (10.0 * systems.scale as f32).floor() + add_x,
                    bg_pos.y + (51.0 * systems.scale as f32).floor(),
                    ORDER_MAPPOS,
                );
                let textbox_size = Vec2::new(50.0, 20.0);

                let mut textbox_data = Textbox::new(
                    systems,
                    textbox_pos,
                    Vec2::new(0.0, 0.0),
                    textbox_size,
                    Color::rgb(255, 255, 255),
                    RENDER_MAPPOS_GUI,
                    RENDER_MAPPOS_TEXT,
                    [3, 4, 5],
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
                    textbox_pos,
                    (textbox_size * systems.scale as f32).floor(),
                    Color::rgb(70, 70, 70),
                    2,
                );
                let rect_gfx = systems.gfx.add_rect(
                    rect,
                    RENDER_MAPPOS_GUI,
                    "Textbox BG",
                    false,
                    CameraView::SubView1,
                );
                label_bg.push(rect_gfx);
            }

            add_x += (60.0 * systems.scale as f32).floor()
        }

        let button_rect = ButtonRect {
            rect_color: Color::rgb(150, 150, 150),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(180, 180, 180)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(120, 120, 120)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::None,
        };

        let confirm = Button::new(
            systems,
            ButtonType::Rect(button_rect),
            ButtonContentType::Text(ButtonContentText {
                text: "Confirm".to_string(),
                pos: Vec2::new(0.0, 2.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 3,
                buffer_layer: RENDER_MAPPOS_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            bg_pos,
            Vec2::new(70.0, 15.0),
            ORDER_MAPPOS,
            Vec2::new(100.0, 24.0),
            2,
            RENDER_MAPPOS_GUI,
            false,
            None,
            false,
        );
        let cancel = Button::new(
            systems,
            ButtonType::Rect(button_rect),
            ButtonContentType::Text(ButtonContentText {
                text: "Cancel".to_string(),
                pos: Vec2::new(0.0, 2.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 3,
                buffer_layer: RENDER_MAPPOS_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            bg_pos,
            Vec2::new(180.0, 15.0),
            ORDER_MAPPOS,
            Vec2::new(100.0, 24.0),
            2,
            RENDER_MAPPOS_GUI,
            false,
            None,
            false,
        );

        let button = vec![confirm, cancel];

        MapPosInput {
            visible: false,
            black,
            bg,
            textbox,
            label,
            label_bg,
            button,
            cur_textbox: None,
            input_type: MapPosInputType::None,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_size(
            &self.black,
            Vec2::new(systems.size.width, systems.size.height),
        );

        let screen_size = Vec2::new(systems.size.width, systems.size.height);
        println!("Size {screen_size:?}");
        let bg_size = (Vec2::new(350.0, 120.0) * systems.scale as f32).floor();
        let bg_pos = ((screen_size - bg_size) * 0.5).floor();

        systems
            .gfx
            .set_pos(&self.bg, Vec3::new(bg_pos.x, bg_pos.y, ORDER_MAPPOS));

        let mut add_x = 0.0;
        for i in 0..4 {
            let (text_pos, text_size) = if i == 3 {
                (
                    Vec3::new(
                        bg_pos.x + (10.0 * systems.scale as f32).floor(),
                        bg_pos.y + (80.0 * systems.scale as f32).floor(),
                        ORDER_MAPPOS,
                    ),
                    Vec2::new(
                        bg_size.x - (20.0 * systems.scale as f32).floor(),
                        (20.0 * systems.scale as f32).floor(),
                    ),
                )
            } else {
                (
                    Vec3::new(
                        bg_pos.x + (10.0 * systems.scale as f32).floor() + add_x,
                        bg_pos.y + (50.0 * systems.scale as f32).floor(),
                        ORDER_MAPPOS,
                    ),
                    (Vec2::new(50.0, 20.0) * systems.scale as f32).floor(),
                )
            };

            systems.gfx.set_pos(&self.label[i], text_pos);
            systems.gfx.set_bound(
                &self.label[i],
                Some(Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                )),
            );
            systems.gfx.center_text(&self.label[i]);

            match i {
                0 => add_x += (40.0 * systems.scale as f32).floor(),
                1 => add_x += (40.0 * systems.scale as f32).floor(),
                2 => add_x += (60.0 * systems.scale as f32).floor(),
                _ => {}
            }

            if i < 3 {
                let textbox_pos = Vec3::new(
                    bg_pos.x + (10.0 * systems.scale as f32).floor() + add_x,
                    bg_pos.y + (51.0 * systems.scale as f32).floor(),
                    ORDER_MAPPOS,
                );
                self.textbox[i].set_pos(systems, Vec2::new(textbox_pos.x, textbox_pos.y));

                systems.gfx.set_pos(&self.label_bg[i], textbox_pos);
            }

            add_x += (60.0 * systems.scale as f32).floor()
        }

        self.button[0].set_pos(systems, bg_pos);
        self.button[1].set_pos(systems, bg_pos);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        self.visible = visible;
        systems.gfx.set_visible(&self.black, visible);
        systems.gfx.set_visible(&self.bg, visible);
        for textbox in self.textbox.iter_mut() {
            textbox.set_visible(systems, visible);
        }
        for gfx in self.label.iter() {
            systems.gfx.set_visible(gfx, visible);
        }
        for gfx in self.label_bg.iter() {
            systems.gfx.set_visible(gfx, visible);
        }
        for button in self.button.iter_mut() {
            button.set_visible(systems, visible);
        }
    }

    pub fn open(&mut self, systems: &mut SystemHolder, input_type: MapPosInputType) {
        self.input_type = input_type;
        self.set_visible(systems, true);
        for textbox in self.textbox.iter_mut() {
            textbox.set_text(systems, "0".to_string());
        }
    }
}
