use crate::SystemHolder;
use cosmic_text::{Attrs, Metrics};
use graphics::{cosmic_text::Wrap, *};

#[allow(clippy::too_many_arguments)]
pub fn create_label(
    systems: &mut SystemHolder,
    pos: Vec3,
    label_size: Vec2,
    bounds: Bounds,
    color: Color,
    render_layer: u32,
    line_height: f32,
    font_size: f32,
    can_scale: bool,
) -> Text {
    let mut text = Text::new(
        &mut systems.renderer,
        Some(Metrics::new(font_size, line_height).scale(if can_scale {
            systems.scale as f32
        } else {
            1.0
        })),
        Vec3::new(pos.x, pos.y, pos.z),
        label_size,
        1.0,
        render_layer,
    );
    text.set_buffer_size(
        &mut systems.renderer,
        Some(systems.size.width),
        Some(systems.size.height),
    )
    .set_bounds(bounds)
    .set_default_color(color);
    text.changed = true;
    text
}

pub fn create_empty_label(
    systems: &mut SystemHolder,
    render_layer: u32,
    color: Color,
    can_scale: bool,
) -> Text {
    let mut text = Text::new(
        &mut systems.renderer,
        Some(Metrics::new(16.0, 16.0).scale(if can_scale { systems.scale as f32 } else { 1.0 })),
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
        1.0,
        render_layer,
    );
    text.set_buffer_size(
        &mut systems.renderer,
        Some(systems.size.width),
        Some(systems.size.height),
    )
    .set_bounds(Bounds::new(0.0, 0.0, 0.0, 0.0))
    .set_default_color(color);
    text.changed = true;
    text
}

pub fn measure_string(
    systems: &mut SystemHolder,
    text: &str,
    can_scale: bool,
    font_size: f32,
    line_height: f32,
) -> Vec2 {
    Text::measure_string(
        &mut systems.renderer.font_sys,
        text,
        &Attrs::new(),
        TextOptions {
            shaping: Shaping::Advanced,
            metrics: Some(Metrics::new(font_size, line_height).scale(if can_scale {
                systems.scale as f32
            } else {
                1.0
            })),
            buffer_width: Some(4096.0),
            buffer_height: Some(4096.0),
            scale: 1.0,
            wrap: Wrap::None,
        },
        None,
    )
}

pub fn measure_character(systems: &mut SystemHolder, text: &str, can_scale: bool) -> Vec<Vec2> {
    Text::measure_glyphs(
        &mut systems.renderer.font_sys,
        text,
        &Attrs::new(),
        TextOptions {
            shaping: Shaping::Advanced,
            metrics: Some(Metrics::new(16.0, 16.0).scale(if can_scale {
                systems.scale as f32
            } else {
                1.0
            })),
            buffer_width: Some(4096.0),
            buffer_height: Some(systems.size.height),
            scale: 1.0,
            wrap: Wrap::None,
        },
        None,
    )
}
