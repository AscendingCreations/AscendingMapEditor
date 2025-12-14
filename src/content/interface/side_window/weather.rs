use graphics::*;

use crate::{
    content::widget::{create_label, option_list::*},
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub struct WeatherWindow {
    pub visible: bool,
    label: GfxType,
    pub weather_list: OptionList,
}

impl WeatherWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let label_pos = Vec3::new(
            start_pos.x + (5.0 * systems.scale as f32).floor(),
            start_pos.y + (area_size.y - (30.0 * systems.scale as f32).floor()),
            ORDER_WINDOW_CONTENT,
        );
        let label_size = Vec2::new(
            area_size.x - (10.0 * systems.scale as f32).floor(),
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
            "Weather Label",
            false,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &label, "Weather");

        let weather_list_name = vec![
            "None".to_string(),
            "Rain".to_string(),
            "Snow".to_string(),
            "Sunny".to_string(),
            "Storm".to_string(),
            "Blizzard".to_string(),
            "Heat".to_string(),
            "Hail".to_string(),
            "SandStorm".to_string(),
            "Windy".to_string(),
        ];
        let list_size = weather_list_name.len();

        let weather_list = OptionList::new(
            systems,
            Vec2::new(
                start_pos.x + (5.0 * systems.scale as f32).floor(),
                start_pos.y + (area_size.y - (60.0 * systems.scale as f32).floor()),
            ),
            Vec2::new(0.0, 0.0),
            Vec2::new((area_size.x / systems.scale as f32).floor() - 10.0, 24.0),
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
            weather_list_name,
            list_size,
            Some(0),
            [ORDER_WINDOW_CONTENT, ORDER_ABOVE_WINDOW],
            [
                RENDER_GUI,
                RENDER_GUI_TEXT,
                RENDER_GUI2,
                RENDER_GUI3,
                RENDER_GUI_TEXT3,
            ],
            [2, 3, 4, 10, 11, 12],
            false,
        );

        WeatherWindow {
            visible: false,
            label,
            weather_list,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        self.weather_list.move_window(
            systems,
            Vec2::new(
                start_pos.x + (5.0 * systems.scale as f32).floor(),
                start_pos.y + (area_size.y - (60.0 * systems.scale as f32).floor()),
            ),
            ORDER_WINDOW_CONTENT,
        );

        let label_pos = Vec3::new(
            start_pos.x + (5.0 * systems.scale as f32).floor(),
            start_pos.y + (area_size.y - (30.0 * systems.scale as f32).floor()),
            ORDER_WINDOW_CONTENT,
        );
        let label_size = Vec2::new(
            area_size.x - (10.0 * systems.scale as f32).floor(),
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
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        self.weather_list.set_visible(systems, visible);
        systems.gfx.set_visible(&self.label, visible);
    }
}
