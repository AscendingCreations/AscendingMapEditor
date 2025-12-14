use graphics::*;

use crate::{
    content::widget::{
        Button, ButtonChangeType, ButtonContentText, ButtonContentType, ButtonRect, ButtonType,
        ScrollbarBackground, ScrollbarRect, SelectionColor, TextList, TextListBG, TextListBGRect,
        TextListData,
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct MenuBar {
    bg: GfxType,
    pub button: Vec<Button>,
    pub file_menu: TextList,
    pub edit_menu: TextList,
}

impl MenuBar {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(
                0.0,
                systems.size.height - (20.0 * systems.scale as f32).floor(),
                ORDER_MENU_BAR,
            ),
            Vec2::new(systems.size.width, (20.0 * systems.scale as f32).floor()),
            Color::rgb(130, 130, 130),
            0,
        );
        let bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Menu Bar BG", true, CameraView::SubView1);

        let button_rect = ButtonRect {
            rect_color: Color::rgb(130, 130, 130),
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(180, 180, 180)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(100, 100, 100)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::None,
        };

        let button = vec![
            Button::new(
                systems,
                ButtonType::Rect(button_rect),
                ButtonContentType::Text(ButtonContentText {
                    text: "File".to_string(),
                    pos: Vec2::new(0.0, 0.0),
                    color: Color::rgb(255, 255, 255),
                    order_layer: 3,
                    buffer_layer: RENDER_GUI_TEXT,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                Vec2::new(
                    0.0,
                    systems.size.height - (20.0 * systems.scale as f32).floor(),
                ),
                Vec2::new(0.0, 0.0),
                ORDER_MENU_BAR,
                Vec2::new(50.0, 20.0),
                2,
                RENDER_GUI,
                true,
                None,
                false,
            ),
            Button::new(
                systems,
                ButtonType::Rect(button_rect),
                ButtonContentType::Text(ButtonContentText {
                    text: "Edit".to_string(),
                    pos: Vec2::new(0.0, 0.0),
                    color: Color::rgb(255, 255, 255),
                    order_layer: 3,
                    buffer_layer: RENDER_GUI_TEXT,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                Vec2::new(
                    (51.0 * systems.scale as f32).floor(),
                    systems.size.height - (20.0 * systems.scale as f32).floor(),
                ),
                Vec2::new(0.0, 0.0),
                ORDER_MENU_BAR,
                Vec2::new(50.0, 20.0),
                2,
                RENDER_GUI,
                true,
                None,
                false,
            ),
        ];

        let bg_rect = TextListBGRect {
            color: Color::rgb(130, 130, 130),
            buffer_layer: RENDER_GUI,
            order_layer: 2,
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            radius: 5.0,
        };
        let scrollbar_rect = ScrollbarRect {
            color: Color::rgb(130, 130, 130),
            buffer_layer: RENDER_GUI,
            order_layer: 3,
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            hover_color: Color::rgb(130, 130, 130),
            hold_color: Color::rgb(130, 130, 130),
            radius: 0.0,
        };
        let scrollbar_bg = ScrollbarBackground {
            color: Color::rgb(130, 130, 130),
            buffer_layer: RENDER_GUI,
            order_layer: 2,
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            radius: 0.0,
        };
        let selection_color = SelectionColor {
            normal: Color::rgb(130, 130, 130),
            hover: Color::rgb(180, 180, 180),
            selected: Color::rgb(130, 130, 130),
        };
        let text_color = SelectionColor {
            normal: Color::rgb(255, 255, 255),
            hover: Color::rgb(255, 255, 255),
            selected: Color::rgb(255, 255, 255),
        };

        let file_menu = TextList::new(
            systems,
            Vec2::new(
                0.0,
                systems.size.height - (110.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            ORDER_MENU_BAR,
            Vec2::new(120.0, 90.0),
            false,
            TextListBG::Rect(bg_rect),
            scrollbar_rect,
            Some(scrollbar_bg),
            vec![
                "Open Map".to_string(),
                "Save".to_string(),
                "Save As...".to_string(),
                "Reload Map".to_string(),
            ],
            TextListData {
                selection_bufferlayer: RENDER_GUI,
                text_bufferlayer: RENDER_GUI_TEXT,
                selection_orderlayer: 4,
                text_orderlayer: 5,
                selection_color,
                text_color,
                max_list: 4,
            },
        );

        let edit_menu = TextList::new(
            systems,
            Vec2::new(
                (51.0 * systems.scale as f32).floor(),
                systems.size.height - (70.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            ORDER_MENU_BAR,
            Vec2::new(120.0, 50.0),
            false,
            TextListBG::Rect(bg_rect),
            scrollbar_rect,
            Some(scrollbar_bg),
            vec!["Undo".to_string(), "Redo".to_string()],
            TextListData {
                selection_bufferlayer: RENDER_GUI,
                text_bufferlayer: RENDER_GUI_TEXT,
                selection_orderlayer: 4,
                text_orderlayer: 5,
                selection_color,
                text_color,
                max_list: 8,
            },
        );

        MenuBar {
            bg,
            button,
            file_menu,
            edit_menu,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        systems.gfx.set_pos(
            &self.bg,
            Vec3::new(
                0.0,
                systems.size.height - (20.0 * systems.scale as f32).floor(),
                ORDER_MENU_BAR,
            ),
        );
        systems.gfx.set_size(
            &self.bg,
            Vec2::new(systems.size.width, (20.0 * systems.scale as f32).floor()),
        );

        for (index, button) in self.button.iter_mut().enumerate() {
            let x_pos = match index {
                1 => (51.0 * systems.scale as f32).floor(),
                2 => (102.0 * systems.scale as f32).floor(),
                _ => 0.0,
            };

            button.set_pos(
                systems,
                Vec2::new(
                    x_pos,
                    systems.size.height - (20.0 * systems.scale as f32).floor(),
                ),
            );
        }

        self.file_menu.set_pos(
            systems,
            Vec2::new(
                0.0,
                systems.size.height - (110.0 * systems.scale as f32).floor(),
            ),
        );
        self.edit_menu.set_pos(
            systems,
            Vec2::new(
                (51.0 * systems.scale as f32).floor(),
                systems.size.height - (190.0 * systems.scale as f32).floor(),
            ),
        );
    }
}
