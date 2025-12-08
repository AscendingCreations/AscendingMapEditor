use graphics::*;

use crate::{
    content::{
        interface::widget::{
            Button, ButtonChangeType, ButtonContentText, ButtonContentType, ButtonRect, ButtonType,
            CheckRect, CheckType, Checkbox, CheckboxChangeType, CheckboxRect, CheckboxText,
            CheckboxType, Scrollbar, ScrollbarBackground, ScrollbarRect, SelectionColor, TextList,
            TextListBG, TextListBGRect, TextListData, Textbox,
        },
        widget::{OptionList, OptionListColor},
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct SampleWindow {
    pub bg: GfxType,
    pub content_bg: GfxType,

    pub sample_button: Button,
    pub sample_checkbox: Checkbox,
    pub sample_scrollbar: Scrollbar,
    pub sample_textbox: Textbox,
    pub sample_textlist: TextList,
    pub sample_optionlist: OptionList,
}

impl SampleWindow {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let bg_pos = (Vec2::new(5.0, 5.0) * systems.scale as f32).floor();
        let bg_size = Vec2::new(
            (200.0 * systems.scale as f32).floor(),
            systems.size.height - (5.0 + (20.0 * systems.scale as f32).floor()),
        );

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_WINDOW),
            Vec2::new(bg_size.x + 10.0, systems.size.height),
            Color::rgba(130, 130, 130, 255),
            0,
        );
        let bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Side Window BG", true);

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(bg_pos.x, bg_pos.y, ORDER_WINDOW),
            bg_size,
            Color::rgba(100, 100, 100, 255),
            1,
        );
        let content_bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Side Window BG", true);

        let sample_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: Color::rgb(150, 150, 150),
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(Color::rgb(180, 180, 180)),
                click_change: ButtonChangeType::ColorChange(Color::rgb(120, 120, 120)),
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            ButtonContentType::Text(ButtonContentText {
                text: "Test".to_string(),
                pos: Vec2::new(0.0, 0.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 3,
                buffer_layer: RENDER_GUI_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            Vec2::new(20.0, systems.size.height - 60.0),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW,
            Vec2::new(70.0, 24.0),
            2,
            RENDER_GUI,
            true,
            None,
            false,
        );

        let sample_checkbox = Checkbox::new(
            systems,
            CheckboxType::Rect(CheckboxRect {
                rect_color: Color::rgb(150, 150, 150),
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                border_radius: 0.0,
                hover_change: CheckboxChangeType::ColorChange(Color::rgb(180, 180, 180)),
                click_change: CheckboxChangeType::ColorChange(Color::rgb(120, 120, 120)),
                disable_change: CheckboxChangeType::None,
            }),
            CheckType::SetRect(CheckRect {
                rect_color: Color::rgb(90, 90, 90),
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                border_radius: 0.0,
                pos: Vec2::new(2.0, 2.0),
                size: Vec2::new(16.0, 16.0),
            }),
            Vec2::new(20.0, systems.size.height - 90.0),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW,
            Vec2::new(20.0, 20.0),
            RENDER_GUI,
            2,
            RENDER_GUI,
            3,
            Some(CheckboxText {
                text: "Test".to_string(),
                offset_pos: Vec2::new(3.0, 0.0),
                buffer_layer: RENDER_GUI_TEXT,
                order_layer: 1,
                label_size: Vec2::new(100.0, 20.0),
                color: Color::rgb(255, 255, 255),
                hover_change: CheckboxChangeType::None,
                click_change: CheckboxChangeType::None,
                disable_change: CheckboxChangeType::None,
            }),
            true,
            None,
        );

        let sample_scrollbar = Scrollbar::new(
            systems,
            Vec2::new(180.0, systems.size.height - 120.0),
            Vec2::new(0.0, 0.0),
            80.0,
            20.0,
            true,
            ORDER_WINDOW,
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 3,
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                hover_color: Color::rgb(180, 180, 180),
                hold_color: Color::rgb(120, 120, 120),
                radius: 1.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgb(90, 90, 90),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            4,
            20.0,
            false,
            true,
            None,
            true,
            None,
        );

        let mut sample_textbox = Textbox::new(
            systems,
            Vec3::new(20.0, systems.size.height - 320.0, ORDER_WINDOW),
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 20.0),
            Color::rgb(255, 255, 255),
            RENDER_GUI,
            RENDER_GUI_TEXT,
            [2, 3, 4],
            255,
            Color::rgb(110, 110, 110),
            Color::rgb(150, 150, 150),
            false,
            true,
            None,
            vec![],
            true,
        );
        sample_textbox.set_text(systems, "Test Textbox".to_string());
        sample_textbox.set_select(systems, false);
        sample_textbox.set_hold(false);

        let sample_textlist = TextList::new(
            systems,
            Vec2::new(20.0, systems.size.height - 220.0),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW,
            Vec2::new(180.0, 90.0),
            true,
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
            vec![
                "Test 1".to_string(),
                "Next List".to_string(),
                "Another List".to_string(),
                "Test 2 but not 2".to_string(),
                "Teeeee".to_string(),
            ],
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
                max_list: 4,
            },
        );

        let sample_optionlist = OptionList::new(
            systems,
            Vec2::new(20.0, systems.size.height - 270.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(170.0, 24.0),
            OptionListColor {
                bar: [
                    Color::rgb(85, 85, 85),
                    Color::rgb(120, 120, 120),
                    Color::rgb(60, 60, 60),
                    Color::rgb(40, 40, 40),
                ],
                list_data_text: Color::rgb(255, 255, 255),
                list_scroll: [
                    Color::rgb(150, 150, 150),
                    Color::rgb(180, 180, 180),
                    Color::rgb(120, 120, 120),
                    Color::rgb(90, 90, 90),
                ],
                list_selection: [
                    Color::rgb(85, 85, 85),
                    Color::rgb(120, 120, 120),
                    Color::rgb(60, 60, 60),
                ],
                list_text: [
                    Color::rgb(255, 255, 255),
                    Color::rgb(255, 255, 255),
                    Color::rgb(255, 255, 255),
                ],
            },
            vec![
                "Sample Option 1".to_string(),
                "Sample Option 2".to_string(),
                "Sample Option 3".to_string(),
                "Sample Option 4".to_string(),
            ],
            3,
            Some(0),
            [ORDER_WINDOW, ORDER_ABOVE_WINDOW],
            [
                RENDER_GUI,
                RENDER_GUI_TEXT,
                RENDER_GUI2,
                RENDER_GUI3,
                RENDER_GUI_TEXT3,
            ],
            [2, 3, 4, 10, 11, 12],
            true,
        );

        Self {
            bg,
            content_bg,
            sample_button,
            sample_checkbox,
            sample_scrollbar,
            sample_textbox,
            sample_textlist,
            sample_optionlist,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        let bg_size = Vec2::new(
            (200.0 * systems.scale as f32).floor(),
            systems.size.height - (5.0 + (20.0 * systems.scale as f32).floor()),
        );
        systems.gfx.set_size(&self.content_bg, bg_size);
        systems
            .gfx
            .set_size(&self.bg, Vec2::new(bg_size.x + 10.0, systems.size.height));

        self.sample_button
            .set_pos(systems, Vec2::new(20.0, systems.size.height - 60.0));

        self.sample_checkbox
            .set_pos(systems, Vec2::new(20.0, systems.size.height - 90.0));

        self.sample_scrollbar
            .set_pos(systems, Vec2::new(180.0, systems.size.height - 120.0));

        self.sample_textlist
            .set_pos(systems, Vec2::new(20.0, systems.size.height - 220.0));

        self.sample_optionlist.move_window(
            systems,
            Vec2::new(20.0, systems.size.height - 270.0),
            ORDER_WINDOW,
        );

        self.sample_textbox
            .set_pos(systems, Vec2::new(20.0, systems.size.height - 320.0));
    }
}
