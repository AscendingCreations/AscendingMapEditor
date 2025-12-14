use crate::{
    GfxType, SystemHolder,
    content::interface::widget::{create_label, is_within_area},
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;
use std::default;

#[derive(Clone, Copy)]
pub enum CheckboxChangeType {
    None,
    ImageFrame(usize),
    ColorChange(Color),
}

#[derive(Clone, Copy)]
pub struct CheckRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub pos: Vec2,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct CheckImage {
    pub res: usize,
    pub pos: Vec2,
    pub size: Vec2,
    pub uv: Vec2,
}

#[derive(Clone, Copy)]
pub struct CheckboxRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
    pub disable_change: CheckboxChangeType,
}

#[derive(Clone, Copy)]
pub struct CheckboxImage {
    pub res: usize,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
    pub disable_change: CheckboxChangeType,
}

#[derive(Clone)]
pub struct CheckboxText {
    pub text: String,
    pub offset_pos: Vec2,
    pub buffer_layer: usize,
    pub order_layer: u32,
    pub label_size: Vec2,
    pub color: Color,
    pub hover_change: CheckboxChangeType,
    pub click_change: CheckboxChangeType,
    pub disable_change: CheckboxChangeType,
}

#[derive(Clone, Default)]
pub enum CheckType {
    #[default]
    Empty,
    SetRect(CheckRect),
    SetImage(CheckImage),
}

#[derive(Clone, Default)]
pub enum CheckboxType {
    #[default]
    Empty,
    Rect(CheckboxRect),
    Image(CheckboxImage),
}

#[derive(Default)]
pub struct Checkbox {
    visible: bool,
    pub image: GfxType,
    check_image: GfxType,
    box_type: CheckboxType,
    check_type: CheckType,
    text_type: Option<(GfxType, CheckboxText)>,

    buffer_layer: usize,
    order_layer: u32,
    check_buffer_layer: usize,
    check_order_layer: u32,

    in_hover: bool,
    in_click: bool,
    pub disabled: bool,
    pub value: bool,

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    pub z_order: f32,
    pub box_size: Vec2,
    pub adjust_x: f32,
    pub tooltip: Option<String>,
}

impl Checkbox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        box_type: CheckboxType,
        check_type: CheckType,
        base_pos: Vec2,
        adjust_pos: Vec2,
        z_order: f32,
        box_size: Vec2,
        buffer_layer: usize,
        order_layer: u32,
        check_buffer_layer: usize,
        check_order_layer: u32,
        text_data: Option<CheckboxText>,
        visible: bool,
        tooltip: Option<String>,
    ) -> Self {
        let boxtype = box_type.clone();
        let checktype = check_type.clone();

        let pos = base_pos + (adjust_pos * systems.scale as f32);
        let image = match boxtype {
            CheckboxType::Rect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    box_size * systems.scale as f32,
                    data.rect_color,
                    order_layer,
                );
                rect.set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(
                    rect,
                    buffer_layer,
                    "Checkbox Image",
                    visible,
                    CameraView::SubView1,
                )
            }
            CheckboxType::Image(data) => {
                let img = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    box_size * systems.scale as f32,
                    Vec4::new(
                        0.0,
                        0.0,
                        box_size.x * systems.scale as f32,
                        box_size.y * systems.scale as f32,
                    ),
                    order_layer,
                );

                systems.gfx.add_image(
                    img,
                    buffer_layer,
                    "Checkbox Image",
                    visible,
                    CameraView::SubView1,
                )
            }
            _ => GfxType::None,
        };

        let check_image = match checktype {
            CheckType::SetRect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(
                        pos.x + (data.pos.x * systems.scale as f32),
                        pos.y + (data.pos.y * systems.scale as f32),
                        z_order,
                    ),
                    data.size * systems.scale as f32,
                    data.rect_color,
                    check_order_layer,
                );
                rect.set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(
                    rect,
                    check_buffer_layer,
                    "Checkbox Check",
                    false,
                    CameraView::SubView1,
                )
            }
            CheckType::SetImage(data) => {
                let img = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(
                        pos.x + (data.pos.x * systems.scale as f32),
                        pos.y + (data.pos.y * systems.scale as f32),
                        z_order,
                    ),
                    data.size * systems.scale as f32,
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y),
                    check_order_layer,
                );

                systems.gfx.add_image(
                    img,
                    check_buffer_layer,
                    "Checkbox Check",
                    false,
                    CameraView::SubView1,
                )
            }
            _ => GfxType::None,
        };

        let mut adjust_x = 0.0;
        let text_type = if let Some(data) = text_data {
            let data_copy = data.clone();
            let tpos = Vec3::new(
                pos.x + ((box_size.x + data.offset_pos.x) * systems.scale as f32),
                pos.y + (data.offset_pos.y * systems.scale as f32),
                z_order,
            );
            let txt = create_label(
                systems,
                tpos,
                data.label_size * systems.scale as f32,
                Bounds::new(
                    tpos.x,
                    tpos.y,
                    tpos.x + (data.label_size.x * systems.scale as f32),
                    tpos.y + (data.label_size.y * systems.scale as f32),
                ),
                data.color,
                data.order_layer,
                16.0,
                16.0,
                true,
            );
            let txt_index = systems.gfx.add_text(
                txt,
                data.buffer_layer,
                "Checkbox Text",
                visible,
                CameraView::SubView1,
            );
            systems
                .gfx
                .set_text(&mut systems.renderer, &txt_index, &data.text);
            adjust_x = data.offset_pos.x + data.label_size.x;
            Some((txt_index, data_copy))
        } else {
            None
        };

        Checkbox {
            visible,
            image,
            check_image,
            box_type,
            check_type,
            text_type,
            in_hover: false,
            in_click: false,
            disabled: false,
            value: false,
            base_pos,
            adjust_pos,
            z_order,
            box_size,
            adjust_x,
            tooltip,

            buffer_layer,
            order_layer,
            check_buffer_layer,
            check_order_layer,
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.image);
        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.check_image);
        if let Some(data) = &mut self.text_type {
            systems.gfx.remove_gfx(&mut systems.renderer, &data.0);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems.gfx.set_visible(&self.image, visible);
        if visible {
            systems.gfx.set_visible(&self.check_image, self.value);
        } else {
            systems.gfx.set_visible(&self.check_image, false);
        }
        if let Some(data) = &mut self.text_type {
            systems.gfx.set_visible(&data.0, visible);
        }
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_order = z_order;
        let pos = systems.gfx.get_pos(&self.image);
        systems
            .gfx
            .set_pos(&self.image, Vec3::new(pos.x, pos.y, self.z_order));
        let pos = systems.gfx.get_pos(&self.check_image);
        systems
            .gfx
            .set_pos(&self.check_image, Vec3::new(pos.x, pos.y, self.z_order));
        if let Some(data) = &mut self.text_type {
            let pos = systems.gfx.get_pos(&data.0);
            systems
                .gfx
                .set_pos(&data.0, Vec3::new(pos.x, pos.y, self.z_order));
        }
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;

        let pos = Vec3::new(
            self.base_pos.x + (self.adjust_pos.x * systems.scale as f32),
            self.base_pos.y + (self.adjust_pos.y * systems.scale as f32),
            self.z_order,
        );
        systems.gfx.set_pos(&self.image, pos);

        let contenttype = self.check_type.clone();
        let extra_pos = match contenttype {
            CheckType::SetRect(data) => data.pos * systems.scale as f32,
            CheckType::SetImage(data) => data.pos * systems.scale as f32,
            _ => Vec2::new(0.0, 0.0),
        };
        let pos = Vec3::new(
            self.base_pos.x + (self.adjust_pos.x * systems.scale as f32) + extra_pos.x,
            self.base_pos.y + (self.adjust_pos.y * systems.scale as f32) + extra_pos.y,
            self.z_order,
        );
        systems.gfx.set_pos(&self.check_image, pos);

        if let Some(data) = &mut self.text_type {
            let pos = Vec3::new(
                self.base_pos.x
                    + ((self.adjust_pos.x + self.box_size.x + data.1.offset_pos.x)
                        * systems.scale as f32),
                self.base_pos.y
                    + ((self.adjust_pos.y + data.1.offset_pos.y) * systems.scale as f32),
                self.z_order,
            );
            systems.gfx.set_pos(&data.0, pos);
            systems.gfx.set_bound(
                &data.0,
                Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + (data.1.label_size.x * systems.scale as f32),
                    pos.y + (data.1.label_size.y * systems.scale as f32),
                ),
            );
        }
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_hover == state || !self.visible || self.disabled {
            return;
        }
        self.in_hover = state;
        if !self.in_click {
            if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_click(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_click == state || !self.visible || self.disabled {
            return;
        }
        self.in_click = state;
        if self.in_click {
            self.set_value(systems, !self.value);
        }

        if self.in_click {
            self.apply_click(systems);
        } else if self.in_hover {
            self.apply_hover(systems);
        } else {
            self.apply_normal(systems);
        }
    }

    pub fn set_disabled(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.disabled == state {
            return;
        }
        self.disabled = state;

        if self.disabled {
            systems.gfx.set_visible(&self.check_image, false);
        } else {
            systems
                .gfx
                .set_visible(&self.check_image, self.value && self.visible);
        }

        if self.disabled {
            self.apply_disabled(systems);
        } else if self.in_click {
            self.apply_click(systems);
        } else if self.in_hover {
            self.apply_hover(systems);
        } else {
            self.apply_normal(systems);
        }
    }

    pub fn set_value(&mut self, systems: &mut SystemHolder, value: bool) {
        if self.value == value {
            return;
        }
        self.value = value;
        if self.visible {
            systems.gfx.set_visible(&self.check_image, self.value);
        }
    }

    pub fn change_content_text(&mut self, systems: &mut SystemHolder, text: String) {
        if let Some(text_type) = &mut self.text_type {
            systems
                .gfx
                .set_text(&mut systems.renderer, &text_type.0, &text);
            text_type.1.text.clone_from(&text);
        }
    }

    pub fn in_area(&self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        is_within_area(
            mouse_pos,
            Vec2::new(
                self.base_pos.x + (self.adjust_pos.x * systems.scale as f32).floor(),
                self.base_pos.y + (self.adjust_pos.y * systems.scale as f32).floor(),
            ),
            (Vec2::new(self.box_size.x + self.adjust_x, self.box_size.y) * systems.scale as f32)
                .floor(),
        )
    }

    fn apply_click(&mut self, systems: &mut SystemHolder) {
        match &self.box_type {
            CheckboxType::Rect(data) => {
                if let CheckboxChangeType::ColorChange(color) = data.click_change {
                    systems.gfx.set_color(&self.image, color);
                } else {
                    systems.gfx.set_color(&self.image, data.rect_color);
                }

                if data.got_border {
                    systems.gfx.set_border_color(&self.image, data.border_color);
                }
            }
            CheckboxType::Image(data) => {
                if let CheckboxChangeType::ImageFrame(frame) = data.click_change {
                    systems.gfx.set_uv(
                        &self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type
            && let CheckboxChangeType::ColorChange(color) = data.1.click_change
        {
            systems.gfx.set_color(&data.0, color);
        }
    }

    fn apply_disabled(&mut self, systems: &mut SystemHolder) {
        match &self.box_type {
            CheckboxType::Rect(data) => {
                if let CheckboxChangeType::ColorChange(color) = data.disable_change {
                    systems.gfx.set_color(&self.image, color);
                } else {
                    systems.gfx.set_color(&self.image, data.rect_color);
                }

                if data.got_border {
                    systems.gfx.set_border_color(&self.image, data.border_color);
                }
            }
            CheckboxType::Image(data) => {
                if let CheckboxChangeType::ImageFrame(frame) = data.disable_change {
                    systems.gfx.set_uv(
                        &self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type
            && let CheckboxChangeType::ColorChange(color) = data.1.disable_change
        {
            systems.gfx.set_color(&data.0, color);
        }
    }

    fn apply_hover(&mut self, systems: &mut SystemHolder) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                if let CheckboxChangeType::ColorChange(color) = data.hover_change {
                    systems.gfx.set_color(&self.image, color);
                } else {
                    systems.gfx.set_color(&self.image, data.rect_color);
                }

                if data.got_border {
                    systems.gfx.set_border_color(&self.image, data.border_color);
                }
            }
            CheckboxType::Image(data) => {
                if let CheckboxChangeType::ImageFrame(frame) = data.hover_change {
                    systems.gfx.set_uv(
                        &self.image,
                        Vec4::new(
                            0.0,
                            self.box_size.y * frame as f32,
                            self.box_size.x,
                            self.box_size.y,
                        ),
                    );
                }
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type {
            let contenttype = data.1.clone();
            if let CheckboxChangeType::ColorChange(color) = contenttype.hover_change {
                systems.gfx.set_color(&data.0, color);
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut SystemHolder) {
        let buttontype = self.box_type.clone();
        match buttontype {
            CheckboxType::Rect(data) => {
                systems.gfx.set_color(&self.image, data.rect_color);

                if data.got_border {
                    systems.gfx.set_border_color(&self.image, data.border_color);
                }
            }
            CheckboxType::Image(_) => {
                systems.gfx.set_uv(
                    &self.image,
                    Vec4::new(0.0, 0.0, self.box_size.x, self.box_size.y),
                );
            }
            _ => {}
        }

        if let Some(data) = &mut self.text_type {
            systems.gfx.set_color(&data.0, data.1.color);
        }
    }
}
