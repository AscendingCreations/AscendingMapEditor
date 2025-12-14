use graphics::*;

use crate::{
    audio::AudioCollection,
    content::widget::{button::*, create_label, scrollbar::*, text_list::*},
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct MusicWindow {
    pub visible: bool,
    label: GfxType,
    pub button: Vec<Button>,
    pub music_list: TextList,
}

impl MusicWindow {
    pub fn new(
        audio_collection: &AudioCollection,
        systems: &mut SystemHolder,
        start_pos: Vec2,
        area_size: Vec2,
    ) -> Self {
        let label_pos = Vec3::new(
            start_pos.x + (10.0 * systems.scale as f32).floor(),
            start_pos.y + (area_size.y - (30.0 * systems.scale as f32).floor()),
            ORDER_WINDOW_CONTENT,
        );
        let label_size = Vec2::new(
            area_size.x - (20.0 * systems.scale as f32).floor(),
            (20.0 * systems.scale as f32).floor(),
        );
        let text = create_label(
            systems,
            label_pos,
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
        let label = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Music Label",
            false,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &label, "Map Music");

        let buttonrect = ButtonRect {
            rect_color: Color::rgb(80, 80, 80),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(50, 50, 50)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(80, 80, 80)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::None,
        };
        let button_size = (((area_size.x / systems.scale as f32).floor() - 20.0) * 0.5).floor();

        let button = vec![
            Button::new(
                systems,
                ButtonType::Rect(buttonrect),
                ButtonContentType::Text(ButtonContentText {
                    text: "Play".to_string(),
                    pos: Vec2::new(0.0, 0.0),
                    color: Color::rgb(255, 255, 255),
                    order_layer: 2,
                    buffer_layer: RENDER_GUI_TEXT,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                Vec2::new(start_pos.x, start_pos.y),
                Vec2::new(9.0, 10.0),
                ORDER_WINDOW_CONTENT,
                Vec2::new(button_size, 24.0),
                1,
                RENDER_GUI,
                false,
                None,
                false,
            ),
            Button::new(
                systems,
                ButtonType::Rect(buttonrect),
                ButtonContentType::Text(ButtonContentText {
                    text: "Stop".to_string(),
                    pos: Vec2::new(0.0, 0.0),
                    color: Color::rgb(255, 255, 255),
                    order_layer: 2,
                    buffer_layer: RENDER_GUI_TEXT,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                Vec2::new(start_pos.x, start_pos.y),
                Vec2::new(11.0 + button_size, 10.0),
                ORDER_WINDOW_CONTENT,
                Vec2::new(button_size, 24.0),
                1,
                RENDER_GUI,
                false,
                None,
                false,
            ),
        ];

        let mut list_size = Vec2::new(
            (area_size.x / systems.scale as f32).floor() - 20.0,
            area_size.y - (64.0 * systems.scale as f32).floor(),
        );
        let list_pos = Vec2::new(
            start_pos.x + (10.0 * systems.scale as f32).floor(),
            start_pos.y + (39.0 * systems.scale as f32).floor(),
        );
        let max_visible_list = ((list_size.y - (10.0 * systems.scale as f32).floor())
            / (20.0 * systems.scale as f32).floor())
        .floor() as usize;
        list_size.y = (max_visible_list as f32 * 20.0) + 10.0;

        let mut music_list = TextList::new(
            systems,
            list_pos,
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW_CONTENT,
            list_size,
            false,
            TextListBG::Rect(TextListBGRect {
                color: Color::rgb(85, 85, 85),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 3,
                got_border: false,
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
            audio_collection.audio.clone(),
            TextListData {
                selection_bufferlayer: RENDER_GUI,
                text_bufferlayer: RENDER_GUI_TEXT,
                selection_orderlayer: 4,
                text_orderlayer: 5,
                selection_color: SelectionColor {
                    normal: Color::rgb(85, 85, 85),
                    hover: Color::rgb(120, 120, 120),
                    selected: Color::rgb(60, 60, 60),
                },
                text_color: SelectionColor {
                    normal: Color::rgb(255, 255, 255),
                    hover: Color::rgb(255, 255, 255),
                    selected: Color::rgb(255, 255, 255),
                },
                max_list: max_visible_list,
            },
        );
        music_list.set_select(systems, Some(0), true);

        MusicWindow {
            visible: false,
            label,
            button,
            music_list,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        let label_pos = Vec3::new(
            start_pos.x + (10.0 * systems.scale as f32).floor(),
            start_pos.y + (area_size.y - (30.0 * systems.scale as f32).floor()),
            ORDER_WINDOW_CONTENT,
        );
        let label_size = Vec2::new(
            area_size.x - (20.0 * systems.scale as f32).floor(),
            (20.0 * systems.scale as f32).floor(),
        );
        systems.gfx.set_pos(&self.label, label_pos);
        systems.gfx.set_bound(
            &self.label,
            Bounds::new(
                label_pos.x,
                label_pos.y,
                label_pos.x + label_size.x,
                label_pos.y + label_size.y,
            ),
        );

        let mut list_size = Vec2::new(
            (area_size.x / systems.scale as f32).floor() - 20.0,
            area_size.y - (64.0 * systems.scale as f32).floor(),
        );
        let list_pos = Vec2::new(
            start_pos.x + (10.0 * systems.scale as f32).floor(),
            start_pos.y + (39.0 * systems.scale as f32).floor(),
        );
        let max_visible_list = ((list_size.y - (10.0 * systems.scale as f32).floor())
            / (20.0 * systems.scale as f32).floor())
        .floor() as usize;
        list_size.y = (max_visible_list as f32 * 20.0) + 10.0;

        self.music_list.set_pos(systems, list_pos);
        self.music_list
            .set_size(systems, list_size, max_visible_list);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        systems.gfx.set_visible(&self.label, visible);
        for button in self.button.iter_mut() {
            button.set_visible(systems, visible);
        }
        self.music_list.set_visible(systems, visible, false);
    }
}
