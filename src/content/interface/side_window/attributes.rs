use graphics::*;

use crate::{
    content::widget::{checkbox::*, scrollbar::*},
    data_types::*,
    database::{EditorMapAttribute, MapAttribute},
    gfx_collection::GfxType,
    renderer::SystemHolder,
};

pub mod attrs_index;
pub mod attrs_itemspawn;
pub mod attrs_position;
pub mod attrs_sign;

pub use attrs_index::*;
pub use attrs_itemspawn::*;
pub use attrs_position::*;
pub use attrs_sign::*;

pub struct AttributeWindow {
    pub visible: bool,
    bg: GfxType,
    lower_bg: GfxType,
    pub attribute: Vec<Checkbox>,
    pub attribute_scroll: Scrollbar,
    seperator: GfxType,

    pub cur_attr_display: Option<usize>,
    pub cur_attribute: EditorMapAttribute,

    pub attr_position: AttrPosition,
    pub attr_index: AttrIndex,
    pub attr_sign: AttrSign,
    pub attr_itemspawn: AttrItemSpawn,
}

impl AttributeWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let option_area_size = Vec2::new(area_size.x, (MAX_VISIBLE_ATTRIBUTE * 22) as f32);

        let option_pos = Vec2::new(
            start_pos.x,
            start_pos.y
                + (area_size.y - ((option_area_size.y + 10.0) * systems.scale as f32).floor()),
        );

        let content_attr_size = Vec2::new(
            area_size.x,
            area_size.y - ((option_area_size.y + 25.0) * systems.scale as f32).floor(),
        );

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

        let bg_size = Vec2::new(area_size.x, area_size.y - content_attr_size.y);
        let bg_pos = Vec2::new(start_pos.x, start_pos.y + content_attr_size.y);
        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(bg_pos.x, bg_pos.y, ORDER_WINDOW_CONTENT2),
            bg_size,
            Color::rgb(100, 100, 100),
            0,
        );
        let bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI2, "BG", false, CameraView::SubView1);

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

        let mut attribute = Vec::with_capacity(12);
        let max_attribute = (EditorMapAttribute::Count as usize).saturating_sub(1);
        let visible_count = max_attribute.min(MAX_VISIBLE_ATTRIBUTE);
        let left_over = max_attribute.saturating_sub(MAX_VISIBLE_ATTRIBUTE);

        for i in 0..visible_count {
            attribute.push(Checkbox::new(
                systems,
                CheckboxType::Rect(checkbox_rect),
                CheckType::SetRect(check_rect),
                option_pos,
                Vec2::new(5.0, 22.0 * ((MAX_VISIBLE_ATTRIBUTE - 1) - i) as f32),
                ORDER_WINDOW_CONTENT2,
                Vec2::new(20.0, 20.0),
                RENDER_GUI2,
                1,
                RENDER_GUI2,
                2,
                Some(CheckboxText {
                    text: MapAttribute::as_str((i as u32).saturating_add(1)).to_string(),
                    offset_pos: Vec2::new(3.0, 0.0),
                    buffer_layer: RENDER_GUI_TEXT2,
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

        attribute[0].set_value(systems, true);

        let attribute_scroll = Scrollbar::new(
            systems,
            option_pos
                + Vec2::new(
                    option_area_size.x - (15.0 * systems.scale as f32).floor(),
                    0.0,
                ),
            Vec2::new(0.0, -1.0),
            option_area_size.y,
            10.0,
            true,
            ORDER_WINDOW_CONTENT2,
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI2,
                order_layer: 4,
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                hover_color: Color::rgb(180, 180, 180),
                hold_color: Color::rgb(120, 120, 120),
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgb(90, 90, 90),
                buffer_layer: RENDER_GUI2,
                order_layer: 3,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            left_over,
            20.0,
            false,
            false,
            None,
            true,
            None,
        );

        let separator_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(
                start_pos.x + (5.0 * systems.scale as f32).floor(),
                option_pos.y - (10.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT2,
            ),
            Vec2::new(
                option_area_size.x - (10.0 * systems.scale as f32).floor(),
                (2.0 * systems.scale as f32).floor(),
            ),
            Color::rgb(140, 140, 140),
            1,
        );
        let seperator = systems.gfx.add_rect(
            separator_rect,
            RENDER_GUI2,
            "Seperator",
            false,
            CameraView::SubView1,
        );

        AttributeWindow {
            visible: false,

            bg,
            lower_bg,
            attribute,
            attribute_scroll,
            seperator,

            cur_attr_display: Some(0),
            cur_attribute: EditorMapAttribute::Blocked,

            attr_position: AttrPosition::new(systems, start_pos, content_attr_size, false),
            attr_index: AttrIndex::new(systems, start_pos, content_attr_size, false),
            attr_sign: AttrSign::new(systems, start_pos, content_attr_size, false),
            attr_itemspawn: AttrItemSpawn::new(systems, start_pos, content_attr_size, false),
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        let option_area_size = Vec2::new(area_size.x, (MAX_VISIBLE_ATTRIBUTE * 22) as f32);

        let option_pos = Vec2::new(
            start_pos.x,
            start_pos.y
                + (area_size.y - ((option_area_size.y + 10.0) * systems.scale as f32).floor()),
        );

        let content_attr_size = Vec2::new(
            area_size.x,
            area_size.y - ((option_area_size.y + 25.0) * systems.scale as f32).floor(),
        );

        let bg_size = Vec2::new(area_size.x, area_size.y - content_attr_size.y);
        let bg_pos = Vec2::new(start_pos.x, start_pos.y + content_attr_size.y);

        systems.gfx.set_size(&self.bg, bg_size);
        systems.gfx.set_pos(
            &self.bg,
            Vec3::new(bg_pos.x, bg_pos.y, ORDER_WINDOW_CONTENT2),
        );

        systems
            .gfx
            .set_size(&self.lower_bg, Vec2::new(area_size.x, start_pos.y));
        systems.gfx.set_pos(
            &self.lower_bg,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
        );

        for attribute in self.attribute.iter_mut() {
            attribute.set_pos(systems, option_pos);
        }

        self.attribute_scroll.set_pos(
            systems,
            option_pos
                + Vec2::new(
                    option_area_size.x - (15.0 * systems.scale as f32).floor(),
                    0.0,
                ),
        );

        systems.gfx.set_pos(
            &self.seperator,
            Vec3::new(
                start_pos.x + 5.0,
                option_pos.y - 10.0,
                ORDER_WINDOW_CONTENT2,
            ),
        );

        self.attr_position
            .screen_resize(systems, start_pos, content_attr_size);
        self.attr_index
            .screen_resize(systems, start_pos, content_attr_size);
        self.attr_sign
            .screen_resize(systems, start_pos, content_attr_size);
        self.attr_itemspawn
            .screen_resize(systems, start_pos, content_attr_size);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        self.attribute_scroll.set_visible(systems, visible);

        systems.gfx.set_visible(&self.seperator, visible);
        systems.gfx.set_visible(&self.bg, visible);
        systems.gfx.set_visible(&self.lower_bg, visible);

        for checkbox in self.attribute.iter_mut() {
            checkbox.set_visible(systems, visible);
            checkbox.set_value(systems, false);
        }

        self.attribute[0].set_value(systems, true);
        self.attribute_scroll.set_value(systems, 0);
        self.cur_attr_display = Some(0);
        self.cur_attribute = EditorMapAttribute::Blocked;

        self.attr_position.set_visible(systems, false);
        self.attr_index.set_visible(systems, false);
        self.attr_sign.set_visible(systems, false);
        self.attr_itemspawn.set_visible(systems, false);
    }
}
