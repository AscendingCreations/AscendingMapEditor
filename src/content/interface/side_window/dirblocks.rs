use graphics::*;

use crate::{content::widget::checkbox::*, data_types::*, renderer::SystemHolder};

pub struct DirBlockWindow {
    pub visible: bool,
    pub blocks: Vec<Checkbox>,
}

impl DirBlockWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let mut blocks = Vec::with_capacity(4);

        let checkbox_rect = CheckboxRect {
            rect_color: Color::rgb(150, 150, 150),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: CheckboxChangeType::ColorChange(Color::rgb(180, 180, 180)),
            click_change: CheckboxChangeType::ColorChange(Color::rgb(120, 120, 120)),
            disable_change: CheckboxChangeType::None,
        };

        let check_rect = CheckRect {
            rect_color: Color::rgb(90, 90, 90),
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            pos: Vec2::new(3.0, 3.0),
            size: Vec2::new(14.0, 14.0),
        };

        let blocks_pos = Vec2::new(
            start_pos.x,
            start_pos.y + (area_size.y - (98.0 * systems.scale as f32).floor()),
        );

        for i in 0..4 {
            blocks.push(Checkbox::new(
                systems,
                CheckboxType::Rect(checkbox_rect),
                CheckType::SetRect(check_rect),
                blocks_pos,
                Vec2::new(5.0, 22.0 * (3 - i) as f32),
                ORDER_WINDOW_CONTENT,
                Vec2::new(20.0, 20.0),
                RENDER_GUI,
                1,
                RENDER_GUI,
                2,
                Some(CheckboxText {
                    text: match i {
                        1 => "Block Up".to_string(),
                        2 => "Block Left".to_string(),
                        3 => "Block Right".to_string(),
                        _ => "Block Down".to_string(),
                    },
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

        DirBlockWindow {
            visible: false,
            blocks,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        let blocks_pos = Vec2::new(
            start_pos.x,
            start_pos.y + (area_size.y - (98.0 * systems.scale as f32).floor()),
        );

        for zones in self.blocks.iter_mut() {
            zones.set_pos(systems, blocks_pos);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        for checkbox in self.blocks.iter_mut() {
            checkbox.set_visible(systems, visible);
        }
    }
}
