use graphics::*;

use crate::{
    content::widget::{
        Button, ButtonChangeType, ButtonContentImg, ButtonContentText, ButtonContentType,
        ButtonImage, ButtonRect, ButtonType, Scrollbar, ScrollbarBackground, ScrollbarRect,
        create_label,
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
    resource::GuiTexture,
};

pub struct DrawingTool {
    tool_bg: GfxType,
    zoom_bg: GfxType,
    pub tool_button: Vec<Button>,
    pub zoom_scroll: Scrollbar,
    pub zoom_label: GfxType,
    pub layer_button: Vec<Button>,

    pub cur_tool: ToolType,
    pub cur_layer: usize,
}

impl DrawingTool {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let max_tool = ToolType::Count as usize;

        let tool_bg_size = (Vec2::new(
            10.0 + (30.0 * max_tool as f32) + (max_tool.saturating_sub(1) as f32),
            34.0,
        ) * systems.scale as f32)
            .floor();
        let tool_bg_pos = Vec2::new(
            (260.0 * systems.scale as f32).floor(),
            systems.size.height - (64.0 * systems.scale as f32).floor(),
        );

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(tool_bg_pos.x, tool_bg_pos.y, ORDER_WINDOW),
            tool_bg_size,
            Color::rgb(90, 90, 90),
            0,
        );
        rect.set_border_color(Color::rgb(0, 0, 0)).set_radius(5.0);
        let tool_bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Tool BG", true, CameraView::SubView1);

        let button_rect = ButtonRect {
            rect_color: Color::rgb(90, 90, 90),
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(110, 110, 110)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(130, 130, 130)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::ColorChange(Color::rgb(70, 70, 70)),
        };

        let mut tool_button = Vec::with_capacity(max_tool);
        for i in 0..max_tool {
            tool_button.push(Button::new(
                systems,
                ButtonType::Rect(button_rect),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.interface[GuiTexture::ToolIcon as usize],
                    pos: Vec2::new(0.0, 0.0),
                    uv: Vec2::new(30.0 * i as f32, 0.0),
                    size: Vec2::new(30.0, 30.0),
                    order_layer: 1,
                    buffer_layer: RENDER_GUI2,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                tool_bg_pos,
                Vec2::new(5.0 + (31.0 * i as f32), 2.0),
                ORDER_WINDOW,
                Vec2::new(30.0, 30.0),
                2,
                RENDER_GUI,
                true,
                Some(match i {
                    1 => "Paint Tool".to_string(),
                    2 => "Eraser Tool".to_string(),
                    3 => "Fill Tool".to_string(),
                    4 => "Picker Tool".to_string(),
                    _ => "Move Tool".to_string(),
                }),
                false,
            ));
        }
        tool_button[0].set_disable(systems, true);

        let zoom_bg_size = (Vec2::new(160.0, 20.0) * systems.scale as f32).floor();
        let zoom_bg_pos = Vec2::new(
            (250.0 * systems.scale as f32).floor() + 10.0,
            (20.0 * systems.scale as f32).floor() + 10.0,
        );

        let mut rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(zoom_bg_pos.x, zoom_bg_pos.y, ORDER_WINDOW),
            zoom_bg_size,
            Color::rgb(90, 90, 90),
            0,
        );
        rect.set_border_color(Color::rgb(0, 0, 0)).set_radius(5.0);
        let zoom_bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Zoom BG", true, CameraView::SubView1);

        let zoom_scroll = Scrollbar::new(
            systems,
            zoom_bg_pos,
            Vec2::new(56.0, 4.0),
            100.0,
            12.0,
            false,
            ORDER_WINDOW,
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                hover_color: Color::rgb(180, 180, 180),
                hold_color: Color::rgb(120, 120, 120),
                radius: 3.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgb(50, 50, 50),
                buffer_layer: RENDER_GUI,
                order_layer: 1,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 3.0,
            }),
            20,
            25.0,
            false,
            true,
            None,
            true,
            None,
        );

        let label_pos = Vec3::new(
            zoom_bg_pos.x + (4.0 * systems.scale as f32).floor(),
            zoom_bg_pos.y,
            ORDER_WINDOW,
        );
        let label_size = Vec2::new(
            zoom_bg_size.x - (108.0 * systems.scale as f32).floor(),
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
        let zoom_label = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Zoom Label",
            true,
            CameraView::SubView1,
        );
        systems.gfx.set_text(
            &mut systems.renderer,
            &zoom_label,
            &format!("{}%", 100 + (10 * zoom_scroll.value)),
        );
        systems.gfx.center_text(&zoom_label);

        let mut layer_button = Vec::with_capacity(MapLayers::Count as usize);
        for layer in 0..MapLayers::Count as usize {
            layer_button.push(Button::new(
                systems,
                ButtonType::Image(ButtonImage {
                    res: match layer {
                        1..=4 => systems.resource.interface[GuiTexture::AnimLayerIcon as usize],
                        6 | 8 => systems.resource.interface[GuiTexture::LayerTopIcon as usize],
                        _ => systems.resource.interface[GuiTexture::LayerIcon as usize],
                    },
                    hover_change: ButtonChangeType::ImageFrame(1),
                    click_change: ButtonChangeType::ImageFrame(1),
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::ImageFrame(2),
                }),
                ButtonContentType::None,
                Vec2::new(
                    systems.size.width - (28.0 * systems.scale as f32).floor() - 14.0,
                    10.0 + (20.0 * systems.scale as f32).floor(),
                ),
                Vec2::new(
                    if matches!(layer, 1..=4) { 0.0 } else { 4.0 },
                    11.0 * layer as f32 + if matches!(layer, 7 | 8) { 10.0 } else { 0.0 },
                ),
                ORDER_WINDOW,
                if matches!(layer, 6 | 8) {
                    Vec2::new(28.0, 17.0)
                } else {
                    Vec2::new(28.0, 13.0)
                },
                2,
                RENDER_GUI,
                true,
                None,
                false,
            ));
        }
        layer_button[0].set_disable(systems, true);

        DrawingTool {
            tool_bg,
            zoom_bg,
            tool_button,
            cur_tool: ToolType::Move,
            zoom_scroll,
            zoom_label,
            layer_button,
            cur_layer: 0,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        let tool_bg_pos = Vec2::new(
            (260.0 * systems.scale as f32).floor(),
            systems.size.height - (64.0 * systems.scale as f32).floor(),
        );

        systems.gfx.set_pos(
            &self.tool_bg,
            Vec3::new(tool_bg_pos.x, tool_bg_pos.y, ORDER_WINDOW),
        );

        for button in self.tool_button.iter_mut() {
            button.set_pos(systems, tool_bg_pos);
        }

        for button in self.layer_button.iter_mut() {
            button.set_pos(
                systems,
                Vec2::new(
                    systems.size.width - (28.0 * systems.scale as f32).floor() - 10.0,
                    10.0 + (20.0 * systems.scale as f32).floor(),
                ),
            );
        }

        let zoom_bg_pos = Vec2::new(
            (250.0 * systems.scale as f32).floor() + 10.0,
            (20.0 * systems.scale as f32).floor() + 10.0,
        );
        let zoom_bg_size = (Vec2::new(160.0, 20.0) * systems.scale as f32).floor();

        systems.gfx.set_pos(
            &self.zoom_bg,
            Vec3::new(zoom_bg_pos.x, zoom_bg_pos.y, ORDER_WINDOW),
        );

        self.zoom_scroll.set_pos(systems, zoom_bg_pos);

        let label_pos = Vec3::new(
            zoom_bg_pos.x + (4.0 * systems.scale as f32).floor(),
            zoom_bg_pos.y,
            ORDER_WINDOW,
        );
        let label_size = Vec2::new(
            zoom_bg_size.x - (108.0 * systems.scale as f32).floor(),
            (20.0 * systems.scale as f32).floor(),
        );
        systems.gfx.set_pos(&self.zoom_label, label_pos);
        systems.gfx.set_bound(
            &self.zoom_label,
            Bounds::new(
                label_pos.x,
                label_pos.y,
                label_pos.x + label_size.x,
                label_pos.y + label_size.y,
            ),
        );
        systems.gfx.center_text(&self.zoom_label);
    }
}
