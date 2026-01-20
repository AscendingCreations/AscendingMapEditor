use graphics::*;

use crate::{
    content::{
        interface::widget::create_label,
        widget::{button::*, scrollbar::*, text_list::*},
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
    resource::GuiTexture,
};

#[derive(Clone, Copy)]
pub struct OptionListColor {
    pub bar: [Color; 4],
    pub list_data_text: Color,
    pub list_scroll: [Color; 4],
    pub list_selection: [Color; 3],
    pub list_text: [Color; 3],
}

pub struct OptionList {
    pub bar: GfxType,
    text: GfxType,
    pub button: Button,
    pub list: TextList,
    pub size: Vec2,
    pub color: OptionListColor,
    max_visible_list: usize,
    adjust_pos: Vec2,
    list_size: Vec2,
    pub disabled: bool,
}

impl OptionList {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        pos: Vec2,
        adjust_pos: Vec2,
        size: Vec2,
        color: OptionListColor,
        list: Vec<String>,
        max_visible_list: usize,
        selected_index: Option<usize>,
        z_pos: [f32; 2],
        buffer_layer: [usize; 5],
        order_layer: [u32; 6],
        visible: bool,
    ) -> Self {
        let widget_pos = pos + adjust_pos;

        let data_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: color.bar[0],
                got_border: false,
                border_color: Color::rgba(0, 0, 0, 0),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(color.bar[1]),
                click_change: ButtonChangeType::ColorChange(color.bar[2]),
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::ColorChange(color.bar[3]),
            }),
            ButtonContentType::Image(ButtonContentImg {
                res: systems.resource.interface[GuiTexture::VerticalArrow as usize],
                pos: Vec2::new(-2.0, -2.0),
                uv: Vec2::new(24.0, 0.0),
                size: Vec2::new(24.0, 24.0),
                order_layer: order_layer[2],
                buffer_layer: buffer_layer[2],
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            widget_pos,
            Vec2::new(size.x - 22.0, 2.0),
            z_pos[0],
            Vec2::new(20.0, 20.0),
            order_layer[1],
            buffer_layer[0],
            visible,
            None,
            false,
        );

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(widget_pos.x, widget_pos.y, z_pos[0]),
            (size * systems.scale as f32).floor(),
            color.bar[0],
            order_layer[0],
        );
        rect.set_border_color(Color::rgb(0, 0, 0))
            .set_border_width(1.0);
        let data_bar = systems.gfx.add_rect(
            rect,
            buffer_layer[0],
            "Option Bar",
            visible,
            CameraView::SubView1,
        );

        let y_adjust = ((size.y - 16.0) * 0.5).floor() + 1.0;

        let textdata = create_label(
            systems,
            Vec3::new(
                widget_pos.x + (4.0 * systems.scale as f32).floor(),
                widget_pos.y + (y_adjust * systems.scale as f32).floor(),
                z_pos[0],
            ),
            (Vec2::new(size.x, 16.0) * systems.scale as f32).floor(),
            Bounds::new(
                widget_pos.x + (4.0 * systems.scale as f32).floor(),
                widget_pos.y + (y_adjust * systems.scale as f32).floor(),
                widget_pos.x + ((4.0 + size.x) * systems.scale as f32).floor(),
                widget_pos.y + ((size.y + y_adjust) * systems.scale as f32).floor(),
            ),
            color.list_data_text,
            order_layer[0],
            16.0,
            16.0,
            true,
        );
        let data_text = systems.gfx.add_text(
            textdata,
            buffer_layer[1],
            "Option Text",
            visible,
            CameraView::SubView1,
        );
        systems.gfx.set_text(
            &mut systems.renderer,
            &data_text,
            if let Some(text_index) = selected_index {
                &list[text_index]
            } else {
                &list[0]
            },
        );

        let list_size = Vec2::new(size.x, (max_visible_list as f32 * 20.0) + 9.0);

        let mut data_list = TextList::new(
            systems,
            widget_pos - (Vec2::new(0.0, list_size.y) * systems.scale as f32).floor(),
            Vec2::new(0.0, 0.0),
            z_pos[1],
            list_size,
            false,
            TextListBG::Rect(TextListBGRect {
                color: color.bar[0],
                buffer_layer: buffer_layer[3],
                order_layer: order_layer[3],
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            ScrollbarRect {
                color: color.list_scroll[0],
                buffer_layer: buffer_layer[3],
                order_layer: order_layer[5],
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                hover_color: color.list_scroll[1],
                hold_color: color.list_scroll[2],
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: color.list_scroll[3],
                buffer_layer: buffer_layer[3],
                order_layer: order_layer[4],
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            list,
            TextListData {
                selection_bufferlayer: buffer_layer[3],
                text_bufferlayer: buffer_layer[4],
                selection_orderlayer: order_layer[4],
                text_orderlayer: order_layer[5],
                selection_color: SelectionColor {
                    normal: color.list_selection[0],
                    hover: color.list_selection[1],
                    selected: color.list_selection[2],
                },
                text_color: SelectionColor {
                    normal: color.list_text[0],
                    hover: color.list_text[1],
                    selected: color.list_text[2],
                },
                max_list: max_visible_list,
            },
        );
        data_list.set_select(systems, selected_index, true);

        OptionList {
            bar: data_bar,
            text: data_text,
            button: data_button,
            list: data_list,
            color,
            max_visible_list,
            size,
            adjust_pos,
            list_size,
            disabled: false,
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bar);
        systems.gfx.remove_gfx(&mut systems.renderer, &self.text);
        self.button.unload(systems);
        self.list.unload(systems);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        systems.gfx.set_visible(&self.bar, visible);
        systems.gfx.set_visible(&self.text, visible);
        self.button.set_visible(systems, visible);
        self.list.set_visible(systems, false, true);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, detail_origin: [f32; 2]) {
        systems.gfx.set_pos_z(&self.bar, detail_origin[0]);
        systems.gfx.set_pos_z(&self.text, detail_origin[0]);

        self.button.set_z_order(systems, detail_origin[0]);
        self.list.set_z_order(systems, detail_origin[1]);
    }

    pub fn set_disabled(&mut self, systems: &mut SystemHolder, disabled: bool) {
        if self.disabled == disabled {
            return;
        }
        self.disabled = disabled;

        self.button.set_disable(systems, disabled);
        systems.gfx.set_color(
            &self.bar,
            if disabled {
                self.color.bar[3]
            } else {
                self.color.bar[0]
            },
        );
    }

    pub fn move_window(&mut self, systems: &mut SystemHolder, pos: Vec2, detail_origin: f32) {
        let widget_pos = pos + self.adjust_pos;

        systems.gfx.set_pos(
            &self.bar,
            Vec3::new(widget_pos.x, widget_pos.y, detail_origin),
        );

        let y_adjust = ((self.size.y - 16.0) * 0.5).floor() + 1.0;

        systems.gfx.set_pos(
            &self.text,
            Vec3::new(
                widget_pos.x + (4.0 * systems.scale as f32).floor(),
                widget_pos.y + (y_adjust * systems.scale as f32).floor(),
                detail_origin,
            ),
        );
        systems.gfx.set_bound(
            &self.text,
            Some(Bounds::new(
                widget_pos.x + (4.0 * systems.scale as f32).floor(),
                widget_pos.y + (y_adjust * systems.scale as f32).floor(),
                widget_pos.x + ((4.0 + self.size.x) * systems.scale as f32).floor(),
                widget_pos.y + ((self.size.y + y_adjust) * systems.scale as f32).floor(),
            )),
        );

        self.button.set_pos(systems, widget_pos);
        self.list.set_pos(
            systems,
            widget_pos - (Vec2::new(0.0, self.list_size.y) * systems.scale as f32).floor(),
        );
    }

    pub fn update_label(&mut self, systems: &mut SystemHolder, index: usize) {
        systems.gfx.set_text(
            &mut systems.renderer,
            &self.text,
            if let Some(text) = self.list.list_text.get(index) {
                text
            } else {
                "Error"
            },
        );
    }

    pub fn hover_widget(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        let button_hover = self.button.in_area(systems, mouse_pos);
        self.button.set_hover(systems, button_hover);

        if self.list.visible {
            self.list.hover_list(systems, mouse_pos);
            self.list.hover_scrollbar(systems, mouse_pos);
        }
    }

    pub fn reset_widget(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.button.set_click(systems, false);
        self.list.scrollbar.set_hold(systems, false, mouse_pos);
    }
}
