use crate::{GfxType, SystemHolder, content::interface::widget::is_within_area};
use graphics::*;

#[derive(Copy, Clone)]
pub struct ScrollbarBackground {
    pub color: Color,
    pub buffer_layer: usize,
    pub order_layer: u32,
    pub got_border: bool,
    pub border_color: Color,
    pub radius: f32,
}

#[derive(Copy, Clone)]
pub struct ScrollbarRect {
    pub color: Color,
    pub buffer_layer: usize,
    pub order_layer: u32,
    pub got_border: bool,
    pub border_color: Color,
    pub hover_color: Color,
    pub hold_color: Color,
    pub radius: f32,
}

#[derive(Default, Clone, Copy)]
pub struct ContentArea {
    adjust_pos: Vec2,
    size: Vec2,
}

impl ContentArea {
    pub fn new(adjust_pos: Vec2, size: Vec2) -> Self {
        ContentArea { adjust_pos, size }
    }
}

pub struct Scrollbar {
    pub visible: bool,
    is_vertical: bool,
    reverse_value: bool,
    bg: Option<GfxType>,
    scroll: GfxType,

    z_pos: f32,

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    hold_pos: Vec2,
    pub pos: Vec2,
    pub size: Vec2,
    bar_size: f32,
    pub value: usize,
    pub max_value: usize,
    start_pos: usize,
    end_pos: usize,
    length: usize,
    min_bar_size: f32,
    default_color: Color,
    hover_color: Color,
    hold_color: Color,
    border_color: Color,
    bg_color: Option<Color>,
    bg_border_color: Option<Color>,
    in_hover: bool,
    pub in_hold: bool,
    pub tooltip: Option<String>,
    pub can_scale: bool,

    pub content: Option<ContentArea>,
}

impl Scrollbar {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        base_pos: Vec2,
        adjust_pos: Vec2,
        bar_size: f32,
        thickness: f32,
        is_vertical: bool,
        z_pos: f32,
        scrollbar: ScrollbarRect,
        background: Option<ScrollbarBackground>,
        max_value: usize,
        min_bar_size: f32,
        reverse_value: bool,
        visible: bool,
        tooltip: Option<String>,
        can_scale: bool,
        content: Option<ContentArea>,
    ) -> Self {
        let (bg, bg_color, bg_border_color) = if let Some(data) = background {
            let pos = base_pos
                + (adjust_pos * if can_scale { systems.scale as f32 } else { 1.0 }).floor();
            let bg_pos = Vec3::new(pos.x - 1.0, pos.y - 1.0, z_pos);

            let mut scrollbg_rect = Rect::new(
                &mut systems.renderer,
                bg_pos,
                if is_vertical {
                    (Vec2::new(thickness + 2.0, bar_size + 2.0)
                        * if can_scale { systems.scale as f32 } else { 1.0 })
                    .floor()
                } else {
                    (Vec2::new(bar_size + 2.0, thickness + 2.0)
                        * if can_scale { systems.scale as f32 } else { 1.0 })
                    .floor()
                },
                data.color,
                data.order_layer,
            );

            scrollbg_rect.set_radius(data.radius);
            if data.got_border {
                scrollbg_rect
                    .set_border_width(1.0)
                    .set_border_color(data.border_color);
            }

            let bg = systems.gfx.add_rect(
                scrollbg_rect,
                data.buffer_layer,
                "Scrollbar BG",
                visible,
                CameraView::SubView1,
            );
            (Some(bg), Some(data.color), Some(data.border_color))
        } else {
            (None, None, None)
        };

        let scrollbar_size = ((bar_size - (min_bar_size * max_value as f32)).max(min_bar_size)
            * if can_scale { systems.scale as f32 } else { 1.0 })
        .floor();

        let (start_pos, end_pos) = if is_vertical {
            (
                (adjust_pos.y * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                    as usize
                    + ((bar_size * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                        as usize
                        - scrollbar_size as usize),
                (adjust_pos.y * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                    as usize,
            )
        } else {
            (
                (adjust_pos.x * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                    as usize,
                (adjust_pos.x * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                    as usize
                    + ((bar_size * if can_scale { systems.scale as f32 } else { 1.0 }).floor()
                        as usize
                        - scrollbar_size as usize),
            )
        };
        let length = if is_vertical {
            start_pos - end_pos
        } else {
            end_pos - start_pos
        };

        let (pos, size) = if is_vertical {
            (
                Vec2::new(
                    (adjust_pos.x * if can_scale { systems.scale as f32 } else { 1.0 }).floor(),
                    start_pos as f32,
                ),
                Vec2::new(
                    (thickness * if can_scale { systems.scale as f32 } else { 1.0 }).floor(),
                    scrollbar_size,
                ),
            )
        } else {
            (
                Vec2::new(
                    start_pos as f32,
                    (adjust_pos.y * if can_scale { systems.scale as f32 } else { 1.0 }).floor(),
                ),
                Vec2::new(
                    scrollbar_size,
                    (thickness * if can_scale { systems.scale as f32 } else { 1.0 }).floor(),
                ),
            )
        };

        let mut scroll_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(base_pos.x + pos.x, base_pos.y + pos.y, z_pos),
            size,
            scrollbar.color,
            scrollbar.order_layer,
        );

        scroll_rect.set_radius(scrollbar.radius);
        if scrollbar.got_border {
            scroll_rect
                .set_border_width(1.0)
                .set_border_color(scrollbar.border_color);
        }
        let scroll = systems.gfx.add_rect(
            scroll_rect,
            scrollbar.buffer_layer,
            "Scrollbar Scroll",
            visible,
            CameraView::SubView1,
        );

        Scrollbar {
            visible,
            bg,
            scroll,
            z_pos,
            reverse_value,
            is_vertical,
            base_pos,
            adjust_pos,
            hold_pos: Vec2::new(0.0, 0.0),
            pos,
            size,
            bar_size,
            value: 0,
            max_value,
            start_pos,
            end_pos,
            length,
            min_bar_size,
            default_color: scrollbar.color,
            hover_color: scrollbar.hover_color,
            hold_color: scrollbar.hold_color,
            border_color: scrollbar.border_color,
            bg_color,
            bg_border_color,
            in_hover: false,
            in_hold: false,
            tooltip,
            can_scale,
            content,
        }
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        if let Some(index) = self.bg {
            systems.gfx.remove_gfx(&mut systems.renderer, &index);
        }
        systems.gfx.remove_gfx(&mut systems.renderer, &self.scroll);
    }

    pub fn in_scroll(&mut self, screen_pos: Vec2) -> bool {
        is_within_area(screen_pos, self.base_pos + self.pos, self.size)
    }

    pub fn in_area(&mut self, screen_pos: Vec2) -> bool {
        let mut in_area = is_within_area(
            screen_pos,
            self.base_pos + self.adjust_pos,
            if self.is_vertical {
                Vec2::new(self.size.x, self.length as f32 + self.size.y)
            } else {
                Vec2::new(self.length as f32 + self.size.x, self.size.y)
            },
        );

        if !in_area && let Some(content_area) = self.content {
            in_area = is_within_area(
                screen_pos,
                self.base_pos + content_area.adjust_pos,
                content_area.size,
            );
        }

        in_area
    }

    pub fn set_hover(&mut self, systems: &mut SystemHolder, in_hover: bool) {
        if self.in_hover == in_hover {
            return;
        }
        self.in_hover = in_hover;
        if self.in_hold {
            return;
        }
        if self.in_hover {
            systems.gfx.set_color(&self.scroll, self.hover_color);
        } else {
            systems.gfx.set_color(&self.scroll, self.default_color);
        }
    }

    pub fn set_move_scroll(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if !self.in_hold {
            return;
        }
        let y_pos = if self.is_vertical {
            let new_pos = ((screen_pos.y - self.base_pos.y) - self.hold_pos.y)
                .clamp(self.end_pos as f32, self.start_pos as f32);
            self.pos.y = new_pos;
            self.start_pos as f32 - new_pos
        } else {
            let new_pos = ((screen_pos.x - self.base_pos.x) - self.hold_pos.x)
                .clamp(self.start_pos as f32, self.end_pos as f32);
            self.pos.x = new_pos;
            new_pos - self.start_pos as f32
        };
        self.value = ((y_pos / self.length as f32) * self.max_value as f32).floor() as usize;

        if self.reverse_value {
            self.value = self.max_value.saturating_sub(self.value);
        }

        let pos = systems.gfx.get_pos(&self.scroll);
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(
                self.base_pos.x + self.pos.x,
                self.base_pos.y + self.pos.y,
                pos.z,
            ),
        );
    }

    pub fn set_hold(&mut self, systems: &mut SystemHolder, in_hold: bool, screen_pos: Vec2) {
        if self.in_hold == in_hold {
            return;
        }
        self.in_hold = in_hold;
        if self.in_hold {
            systems.gfx.set_color(&self.scroll, self.hold_color);
            self.hold_pos = screen_pos - (self.base_pos + self.pos);
        } else if self.in_hover {
            systems.gfx.set_color(&self.scroll, self.hover_color);
        } else {
            systems.gfx.set_color(&self.scroll, self.default_color);
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        if let Some(index) = self.bg {
            systems.gfx.set_visible(&index, visible);
        }
        systems.gfx.set_visible(&self.scroll, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_pos = z_order;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(&index);
            systems
                .gfx
                .set_pos(&index, Vec3::new(pos.x, pos.y, self.z_pos));
        }
        let pos = systems.gfx.get_pos(&self.scroll);
        systems
            .gfx
            .set_pos(&self.scroll, Vec3::new(pos.x, pos.y, self.z_pos));
    }

    pub fn set_size(
        &mut self,
        systems: &mut SystemHolder,
        bar_size: f32,
        min_bar_size: f32,
        thickness: f32,
    ) {
        self.bar_size = bar_size;
        self.min_bar_size = min_bar_size;
        if let Some(bg) = self.bg {
            let pos = self.base_pos
                + (self.adjust_pos
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor();
            let bg_pos = Vec3::new(pos.x - 1.0, pos.y - 1.0, self.z_pos);

            systems.gfx.set_pos(&bg, bg_pos);
            systems.gfx.set_size(
                &bg,
                if self.is_vertical {
                    (Vec2::new(thickness + 2.0, bar_size + 2.0)
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor()
                } else {
                    (Vec2::new(bar_size + 2.0, thickness + 2.0)
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor()
                },
            );
        }

        let scrollbar_size = ((bar_size - (min_bar_size * self.max_value as f32))
            .max(min_bar_size)
            * if self.can_scale {
                systems.scale as f32
            } else {
                1.0
            })
        .floor();

        let (start_pos, end_pos) = if self.is_vertical {
            (
                (self.adjust_pos.y
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize
                    + ((bar_size
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor() as usize)
                        .saturating_sub(scrollbar_size as usize),
                (self.adjust_pos.y
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize,
            )
        } else {
            (
                (self.adjust_pos.x
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize,
                (self.adjust_pos.x
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize
                    + ((bar_size
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor() as usize)
                        .saturating_sub(scrollbar_size as usize),
            )
        };
        self.length = if self.is_vertical {
            start_pos - end_pos
        } else {
            end_pos - start_pos
        };

        let size = if self.is_vertical {
            Vec2::new(
                (thickness
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor(),
                scrollbar_size,
            )
        } else {
            Vec2::new(
                scrollbar_size,
                (thickness
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor(),
            )
        };

        self.start_pos = start_pos;
        self.end_pos = end_pos;
        self.size = size;

        systems.gfx.set_size(&self.scroll, size);

        self.set_value(systems, self.value);
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;
        if let Some(index) = self.bg {
            let pos = systems.gfx.get_pos(&index);
            systems.gfx.set_pos(
                &index,
                Vec3::new(
                    new_pos.x
                        + ((self.adjust_pos.x - 1.0)
                            * if self.can_scale {
                                systems.scale as f32
                            } else {
                                1.0
                            })
                        .floor(),
                    new_pos.y
                        + ((self.adjust_pos.y - 1.0)
                            * if self.can_scale {
                                systems.scale as f32
                            } else {
                                1.0
                            })
                        .floor(),
                    pos.z,
                ),
            );
        }
        let pos = systems.gfx.get_pos(&self.scroll);
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(new_pos.x + self.pos.x, new_pos.y + self.pos.y, pos.z),
        );
    }

    pub fn set_value(&mut self, systems: &mut SystemHolder, value: usize) {
        let new_value = if self.reverse_value {
            self.max_value.saturating_sub(value)
        } else {
            value
        };
        if new_value > self.max_value {
            return;
        }

        let new_pos = if self.max_value > 0 {
            ((new_value as f32 / self.max_value as f32) * self.length as f32).floor()
        } else {
            0.0
        };
        self.value = value;
        let pos = systems.gfx.get_pos(&self.scroll);
        if self.is_vertical {
            self.pos.y = (self.adjust_pos.y
                * if self.can_scale {
                    systems.scale as f32
                } else {
                    1.0
                })
            .floor()
                + (self.length as f32 - new_pos);
            systems.gfx.set_pos(
                &self.scroll,
                Vec3::new(
                    self.base_pos.x + self.pos.x,
                    self.base_pos.y + self.pos.y,
                    pos.z,
                ),
            );
        } else {
            self.pos.x = (self.adjust_pos.x
                * if self.can_scale {
                    systems.scale as f32
                } else {
                    1.0
                })
            .floor()
                + new_pos;
            systems.gfx.set_pos(
                &self.scroll,
                Vec3::new(
                    self.base_pos.x + self.pos.x,
                    self.base_pos.y + self.pos.y,
                    pos.z,
                ),
            );
        }
    }

    pub fn set_max_value(&mut self, systems: &mut SystemHolder, max_value: usize) {
        if self.max_value == max_value {
            return;
        }
        self.max_value = max_value;

        let scrollbar_size = ((self.bar_size - (self.min_bar_size * self.max_value as f32))
            .max(self.min_bar_size)
            * if self.can_scale {
                systems.scale as f32
            } else {
                1.0
            })
        .floor();

        (self.start_pos, self.end_pos) = if self.is_vertical {
            (
                (self.adjust_pos.y
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize
                    + ((self.bar_size
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor() as usize)
                        .saturating_sub(scrollbar_size as usize),
                (self.adjust_pos.y
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize,
            )
        } else {
            (
                (self.adjust_pos.x
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize,
                (self.adjust_pos.x
                    * if self.can_scale {
                        systems.scale as f32
                    } else {
                        1.0
                    })
                .floor() as usize
                    + ((self.bar_size
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor() as usize)
                        .saturating_sub(scrollbar_size as usize),
            )
        };
        self.length = if self.is_vertical {
            self.start_pos - self.end_pos
        } else {
            self.end_pos - self.start_pos
        };

        let (pos, size) = if self.is_vertical {
            (
                Vec2::new(
                    (self.adjust_pos.x
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor(),
                    self.start_pos as f32,
                ),
                Vec2::new(self.size.x, scrollbar_size),
            )
        } else {
            (
                Vec2::new(
                    self.start_pos as f32,
                    (self.adjust_pos.y
                        * if self.can_scale {
                            systems.scale as f32
                        } else {
                            1.0
                        })
                    .floor(),
                ),
                Vec2::new(scrollbar_size, self.size.y),
            )
        };
        systems.gfx.set_pos(
            &self.scroll,
            Vec3::new(self.base_pos.x + pos.x, self.base_pos.y + pos.y, self.z_pos),
        );
        systems.gfx.set_size(&self.scroll, size);
    }
}
