use crate::{
    GfxType, SystemHolder,
    content::interface::widget::{is_within_area, label::create_label},
};
use cosmic_text::{Attrs, Metrics};
use graphics::{cosmic_text::Weight, *};

#[derive(Copy, Clone)]
pub enum ButtonChangeType {
    None,
    ImageFrame(usize),
    ColorChange(Color),
    AdjustY(usize),
    ChangeUV(Vec4),
}

#[derive(Copy, Clone)]
pub struct ButtonRect {
    pub rect_color: Color,
    pub got_border: bool,
    pub border_color: Color,
    pub border_radius: f32,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
    pub alert_change: ButtonChangeType,
    pub disable_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonImage {
    pub res: usize,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
    pub alert_change: ButtonChangeType,
    pub disable_change: ButtonChangeType,
}

#[derive(Clone, Copy)]
pub struct ButtonContentImg {
    pub res: usize,
    pub pos: Vec2,
    pub uv: Vec2,
    pub size: Vec2,
    pub order_layer: u32,
    pub buffer_layer: usize,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
    pub alert_change: ButtonChangeType,
    pub disable_change: ButtonChangeType,
}

#[derive(Clone)]
pub struct ButtonContentText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub order_layer: u32,
    pub buffer_layer: usize,
    pub hover_change: ButtonChangeType,
    pub click_change: ButtonChangeType,
    pub alert_change: ButtonChangeType,
    pub disable_change: ButtonChangeType,
}

#[derive(Clone)]
pub enum ButtonType {
    None,
    Rect(ButtonRect),
    Image(ButtonImage),
}

#[derive(Clone)]
pub enum ButtonContentType {
    None,
    Image(ButtonContentImg),
    Text(ButtonContentText),
}

#[derive(Clone)]
pub struct Button {
    pub visible: bool,
    pub disabled: bool,
    index: Option<GfxType>,
    content: Option<GfxType>,
    in_hover: bool,
    in_click: bool,
    in_alert: bool,

    button_type: ButtonType,
    content_type: ButtonContentType,

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    pub z_order: f32,
    pub size: Vec2,
    pub tooltip: Option<String>,

    order_layer: u32,
    buffer_layer: usize,

    pub fix_scale: bool,
}

impl Button {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        button_type: ButtonType,
        content_type: ButtonContentType,
        base_pos: Vec2,
        adjust_pos: Vec2,
        z_order: f32,
        size: Vec2,
        order_layer: u32,
        buffer_layer: usize,
        visible: bool,
        tooltip: Option<String>,
        fix_scale: bool,
    ) -> Self {
        let pos =
            base_pos + (adjust_pos * if fix_scale { 1.0 } else { systems.scale as f32 }).floor();

        let buttontype = button_type.clone();
        let index = match buttontype {
            ButtonType::Rect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    (size * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                    data.rect_color,
                    order_layer,
                );
                rect.set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                let rect_index = systems
                    .gfx
                    .add_rect(rect, buffer_layer, "Button Image", visible);
                Some(rect_index)
            }
            ButtonType::Image(data) => {
                let image = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, z_order),
                    (size * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                    Vec4::new(0.0, 0.0, size.x, size.y),
                    order_layer,
                );

                let image_index =
                    systems
                        .gfx
                        .add_image(image, buffer_layer, "Button Image", visible);
                Some(image_index)
            }
            _ => None,
        };

        let contenttype = content_type.clone();
        let content = match contenttype {
            ButtonContentType::None => None,
            ButtonContentType::Image(data) => {
                let spos = Vec3::new(pos.x, pos.y, z_order);

                let image = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(
                        spos.x
                            + (data.pos.x * if fix_scale { 1.0 } else { systems.scale as f32 })
                                .floor(),
                        spos.y
                            + (data.pos.y * if fix_scale { 1.0 } else { systems.scale as f32 })
                                .floor(),
                        spos.z,
                    ),
                    (data.size * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y),
                    data.order_layer,
                );

                let image_index =
                    systems
                        .gfx
                        .add_image(image, data.buffer_layer, "Button Content", visible);
                Some(image_index)
            }
            ButtonContentType::Text(data) => {
                let spos = Vec3::new(pos.x, pos.y, z_order);
                let text_pos = Vec2::new(
                    spos.x
                        + (data.pos.x * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                    spos.y
                        + (data.pos.y * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                );
                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, spos.z),
                    (Vec2::new(size.x, 20.0) * if fix_scale { 1.0 } else { systems.scale as f32 })
                        .floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x
                            + (size.x * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                        text_pos.y
                            + (20.0 * if fix_scale { 1.0 } else { systems.scale as f32 }).floor(),
                    ),
                    data.color,
                    data.order_layer,
                    16.0,
                    16.0,
                    !fix_scale,
                );
                let index =
                    systems
                        .gfx
                        .add_text(text, data.buffer_layer, "Button Content", visible);
                systems
                    .gfx
                    .set_text(&mut systems.renderer, &index, &data.text);

                systems.gfx.center_text(&index);
                Some(index)
            }
        };

        Button {
            visible,
            disabled: false,
            index,
            content,
            in_hover: false,
            in_click: false,
            in_alert: false,
            button_type,
            content_type,
            base_pos,
            adjust_pos,
            z_order,
            size,
            tooltip,
            order_layer,
            buffer_layer,
            fix_scale,
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        if let Some(index) = self.index {
            systems.gfx.remove_gfx(&mut systems.renderer, &index);
        }
        if let Some(content_index) = self.content {
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &content_index);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        if !visible {
            self.set_click(systems, false);
            self.set_hover(systems, false);
        }
        self.visible = visible;
        if let Some(index) = self.index {
            systems.gfx.set_visible(&index, visible);
        }
        if let Some(index) = self.content {
            systems.gfx.set_visible(&index, visible);
        }
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_order = z_order;
        if let Some(index) = self.index {
            let mut pos = systems.gfx.get_pos(&index);
            pos.z = self.z_order;
            systems.gfx.set_pos(&index, pos);
        }
        if let Some(content_index) = self.content {
            let pos = systems.gfx.get_pos(&content_index);
            systems
                .gfx
                .set_pos(&content_index, Vec3::new(pos.x, pos.y, self.z_order));
        }
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;
        if let Some(index) = self.index {
            let pos = Vec3::new(
                self.base_pos.x
                    + (self.adjust_pos.x
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                self.base_pos.y
                    + (self.adjust_pos.y
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                self.z_order,
            );
            systems.gfx.set_pos(&index, pos);
        }
        if let Some(content_index) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Image(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.z_order,
                    );
                    systems.gfx.set_pos(&content_index, pos);
                }
                ButtonContentType::Text(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.z_order,
                    );
                    systems.gfx.set_pos(&content_index, pos);
                    systems.gfx.set_bound(
                        &content_index,
                        Bounds::new(
                            pos.x,
                            pos.y,
                            pos.x
                                + (self.size.x
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            pos.y
                                + (self.size.y
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                        ),
                    );
                    systems.gfx.center_text(&content_index);
                }
                _ => {}
            };
        }
    }

    pub fn set_adjust_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.adjust_pos = new_pos;
        if let Some(index) = self.index {
            let pos = Vec3::new(
                self.base_pos.x
                    + (self.adjust_pos.x
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                self.base_pos.y
                    + (self.adjust_pos.y
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                self.z_order,
            );
            systems.gfx.set_pos(&index, pos);
        }
        if let Some(content_index) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Image(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.z_order,
                    );
                    systems.gfx.set_pos(&content_index, pos);
                }
                ButtonContentType::Text(data) => {
                    let pos = Vec3::new(
                        self.base_pos.x
                            + ((self.adjust_pos.x + data.pos.x)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.base_pos.y
                            + ((self.adjust_pos.y + data.pos.y)
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        self.z_order,
                    );
                    systems.gfx.set_pos(&content_index, pos);
                    systems.gfx.set_bound(
                        &content_index,
                        Bounds::new(
                            pos.x,
                            pos.y,
                            pos.x
                                + (self.size.x
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            pos.y
                                + (self.size.y
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                        ),
                    );
                    systems.gfx.center_text(&content_index);
                }
                _ => {}
            };
        }
    }

    pub fn change_button_type(
        &mut self,
        systems: &mut SystemHolder,
        button_type: ButtonType,
        change_size: Option<Vec2>,
    ) {
        if let Some(size) = change_size {
            self.size = size;
        }
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();

        if let Some(gfx) = self.index {
            systems.gfx.remove_gfx(&mut systems.renderer, &gfx);
        }

        self.button_type = button_type.clone();
        self.index = match button_type {
            ButtonType::Rect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, self.z_order),
                    (self.size
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                    data.rect_color,
                    self.order_layer,
                );
                rect.set_radius(data.border_radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                let rect_index =
                    systems
                        .gfx
                        .add_rect(rect, self.buffer_layer, "Button Image", self.visible);
                Some(rect_index)
            }
            ButtonType::Image(data) => {
                let image = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(pos.x, pos.y, self.z_order),
                    (self.size
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                    Vec4::new(0.0, 0.0, self.size.x, self.size.y),
                    self.order_layer,
                );

                let image_index =
                    systems
                        .gfx
                        .add_image(image, self.buffer_layer, "Button Image", self.visible);
                Some(image_index)
            }
            _ => None,
        };
    }

    pub fn change_button_content(
        &mut self,
        systems: &mut SystemHolder,
        content_type: ButtonContentType,
    ) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();

        if let Some(content_index) = self.content {
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &content_index);
        }
        self.content_type = content_type.clone();
        self.content = match content_type {
            ButtonContentType::None => None,
            ButtonContentType::Image(data) => {
                let spos = Vec3::new(pos.x, pos.y, self.z_order);

                let image = Image::new(
                    Some(data.res),
                    &mut systems.renderer,
                    Vec3::new(
                        spos.x
                            + (data.pos.x
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        spos.y
                            + (data.pos.y
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        spos.z,
                    ),
                    (data.size
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                    Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y),
                    data.order_layer,
                );

                let image_index =
                    systems
                        .gfx
                        .add_image(image, data.buffer_layer, "Button Content", self.visible);
                Some(image_index)
            }
            ButtonContentType::Text(data) => {
                let spos = Vec3::new(pos.x, pos.y, self.z_order);
                let text_pos = Vec2::new(
                    spos.x
                        + (data.pos.x
                            * if self.fix_scale {
                                1.0
                            } else {
                                systems.scale as f32
                            })
                        .floor(),
                    spos.y
                        + (data.pos.y
                            * if self.fix_scale {
                                1.0
                            } else {
                                systems.scale as f32
                            })
                        .floor(),
                );
                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, spos.z),
                    (Vec2::new(self.size.x, 20.0)
                        * if self.fix_scale {
                            1.0
                        } else {
                            systems.scale as f32
                        })
                    .floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x
                            + (self.size.x
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                        text_pos.y
                            + (20.0
                                * if self.fix_scale {
                                    1.0
                                } else {
                                    systems.scale as f32
                                })
                            .floor(),
                    ),
                    data.color,
                    data.order_layer,
                    16.0,
                    16.0,
                    !self.fix_scale,
                );
                let index =
                    systems
                        .gfx
                        .add_text(text, data.buffer_layer, "Button Content", self.visible);
                systems
                    .gfx
                    .set_text(&mut systems.renderer, &index, &data.text);

                systems.gfx.center_text(&index);
                Some(index)
            }
        };
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_hover == state || !self.visible {
            return;
        }
        self.in_hover = state;
        if !self.in_click && !self.in_alert && !self.disabled {
            if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_click(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_click == state || !self.visible {
            return;
        }
        self.in_click = state;

        if !self.in_alert && !self.disabled {
            if self.in_click {
                self.apply_click(systems);
            } else if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_alert(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.in_alert == state || !self.visible {
            return;
        }
        self.in_alert = state;

        if !self.disabled {
            if self.in_alert {
                self.apply_alert(systems);
            } else if self.in_click {
                self.apply_click(systems);
            } else if self.in_hover {
                self.apply_hover(systems);
            } else {
                self.apply_normal(systems);
            }
        }
    }

    pub fn set_disable(&mut self, systems: &mut SystemHolder, state: bool) {
        if self.disabled == state {
            return;
        }
        self.disabled = state;

        if self.disabled {
            self.apply_disable(systems);
        } else if self.in_alert {
            self.apply_alert(systems);
        } else if self.in_click {
            self.apply_click(systems);
        } else if self.in_hover {
            self.apply_hover(systems);
        } else {
            self.apply_normal(systems);
        }
    }

    pub fn change_text(&mut self, systems: &mut SystemHolder, msg: String) {
        if let Some(content_data) = self.content
            && let ButtonContentType::Text(data) = &mut self.content_type
        {
            systems
                .gfx
                .set_text(&mut systems.renderer, &content_data, &msg);
            data.text = msg;
            systems.gfx.center_text(&content_data);
        }
    }

    pub fn in_area(&self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        is_within_area(
            mouse_pos,
            self.base_pos
                + (self.adjust_pos
                    * if self.fix_scale {
                        1.0
                    } else {
                        systems.scale as f32
                    })
                .floor(),
            (self.size
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor(),
        )
    }

    fn apply_click(&mut self, systems: &mut SystemHolder) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &index,
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&index, uv),
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                        systems.gfx.center_text(&content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.click_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&content_data, uv),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn apply_hover(&mut self, systems: &mut SystemHolder) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &index,
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&index, uv),
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                        systems.gfx.center_text(&content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.hover_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&content_data, uv),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn apply_normal(&mut self, systems: &mut SystemHolder) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            systems
                .gfx
                .set_pos(&index, Vec3::new(pos.x, pos.y, self.z_order));
            match buttontype {
                ButtonType::Rect(data) => {
                    systems.gfx.set_color(&index, data.rect_color);
                }
                ButtonType::Image(_) => {
                    systems
                        .gfx
                        .set_uv(&index, Vec4::new(0.0, 0.0, self.size.x, self.size.y));
                }
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => {
                    systems.gfx.set_pos(
                        &content_data,
                        Vec3::new(
                            pos.x
                                + (data.pos.x
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            pos.y
                                + (data.pos.y
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            self.z_order,
                        ),
                    );
                    systems.gfx.center_text(&content_data);
                }
                ButtonContentType::Image(data) => {
                    systems.gfx.set_pos(
                        &content_data,
                        Vec3::new(
                            pos.x
                                + (data.pos.x
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            pos.y
                                + (data.pos.y
                                    * if self.fix_scale {
                                        1.0
                                    } else {
                                        systems.scale as f32
                                    })
                                .floor(),
                            self.z_order,
                        ),
                    );
                    systems.gfx.set_uv(
                        &content_data,
                        Vec4::new(data.uv.x, data.uv.y, data.size.x, data.size.y),
                    );
                }
                _ => {}
            }
        }
    }

    fn apply_disable(&mut self, systems: &mut SystemHolder) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.disable_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.disable_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &index,
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&index, uv),
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.disable_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                        systems.gfx.center_text(&content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.disable_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&content_data, uv),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn apply_alert(&mut self, systems: &mut SystemHolder) {
        let pos = self.base_pos
            + (self.adjust_pos
                * if self.fix_scale {
                    1.0
                } else {
                    systems.scale as f32
                })
            .floor();
        if let Some(index) = self.index {
            let buttontype = self.button_type.clone();
            match buttontype {
                ButtonType::Rect(data) => match data.alert_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&index, color);
                    }
                    _ => {}
                },
                ButtonType::Image(data) => match data.alert_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &index,
                            Vec3::new(
                                pos.x,
                                pos.y
                                    + (adjusty as f32
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &index,
                            Vec4::new(0.0, self.size.y * frame as f32, self.size.x, self.size.y),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&index, uv),
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(content_data) = self.content {
            let contenttype = self.content_type.clone();
            match contenttype {
                ButtonContentType::Text(data) => match data.alert_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                        systems.gfx.center_text(&content_data);
                    }
                    ButtonChangeType::ColorChange(color) => {
                        systems.gfx.set_color(&content_data, color);
                    }
                    _ => {}
                },
                ButtonContentType::Image(data) => match data.alert_change {
                    ButtonChangeType::AdjustY(adjusty) => {
                        systems.gfx.set_pos(
                            &content_data,
                            Vec3::new(
                                pos.x
                                    + (data.pos.x
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                pos.y
                                    + ((data.pos.y + adjusty as f32)
                                        * if self.fix_scale {
                                            1.0
                                        } else {
                                            systems.scale as f32
                                        })
                                    .floor(),
                                self.z_order,
                            ),
                        );
                    }
                    ButtonChangeType::ImageFrame(frame) => {
                        systems.gfx.set_uv(
                            &content_data,
                            Vec4::new(
                                data.uv.x,
                                data.uv.y + data.size.y * frame as f32,
                                data.size.x,
                                data.size.y,
                            ),
                        );
                    }
                    ButtonChangeType::ChangeUV(uv) => systems.gfx.set_uv(&content_data, uv),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
