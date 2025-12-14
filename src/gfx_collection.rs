use crate::info;
use cosmic_text::{Attrs, Weight};
use graphics::*;
use indexmap::IndexSet;
use slab::Slab;
use slotmap::SlotMap;
use std::{borrow::Cow, default};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum GfxType {
    #[default]
    None,
    Image(Index),
    Rect(Index),
    Text(Index),
    Light(Index),
    Mesh(Index),
}

pub struct GfxData {
    pub layer: usize,
    pub visible: bool,
    pub override_visible: Option<bool>,
    pub debug_track: bool,
    pub identifier: Cow<'static, str>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct LightIndex {
    ltype: u8,
    index: Index,
}

impl LightIndex {
    fn new(ltype: u8, index: Index) -> Self {
        LightIndex { ltype, index }
    }
}

pub struct GfxImage {
    pub data: GfxData,
    pub gfx: Image,
}

pub struct GfxRect {
    pub data: GfxData,
    pub gfx: Rect,
}

pub struct GfxText {
    pub data: GfxData,
    pub gfx: Text,
}

pub struct GfxLight {
    pub data: GfxData,
    pub gfx: Lights,
    visible_lights: IndexSet<LightIndex>,
}

pub struct GfxMesh {
    pub data: GfxData,
    pub gfx: Mesh2D,
}

#[derive(Default)]
pub struct GfxCollection {
    pub image_storage: SlotMap<Index, GfxImage>,
    pub rect_storage: SlotMap<Index, GfxRect>,
    pub text_storage: SlotMap<Index, GfxText>,
    pub light_storage: SlotMap<Index, GfxLight>,
    pub mesh_storage: SlotMap<Index, GfxMesh>,
}

impl GfxCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_collection(&self) {
        info!("Image Size: {:?}", self.image_storage.len());
        info!("Rect Size: {:?}", self.rect_storage.len());
        info!("Text Size: {:?}", self.text_storage.len());
        info!("Light Size: {:?}", self.light_storage.len());
        info!("Mesh Size: {:?}", self.mesh_storage.len());
    }

    pub fn add_image(
        &mut self,
        mut gfx: Image,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType::Image(self.image_storage.insert(GfxImage { data, gfx }))
    }

    pub fn add_rect(
        &mut self,
        mut gfx: Rect,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType::Rect(self.rect_storage.insert(GfxRect { data, gfx }))
    }

    pub fn add_text(
        &mut self,
        mut gfx: Text,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType::Text(self.text_storage.insert(GfxText { data, gfx }))
    }

    pub fn add_light(
        &mut self,
        gfx: Lights,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType::Light(self.light_storage.insert(GfxLight {
            data,
            gfx,
            visible_lights: IndexSet::default(),
        }))
    }

    pub fn add_mesh(
        &mut self,
        gfx: Mesh2D,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType::Mesh(self.mesh_storage.insert(GfxMesh { data, gfx }))
    }

    pub fn clear_mesh(&mut self, index: &GfxType) {
        if let GfxType::Mesh(gfx_index) = index
            && let Some(gfx) = self.mesh_storage.get_mut(*gfx_index)
        {
            gfx.gfx.clear();
        }
    }

    pub fn update_mesh_builder(&mut self, index: &GfxType, builder: &Mesh2DBuilder) {
        if let GfxType::Mesh(gfx_index) = index
            && let Some(gfx) = self.mesh_storage.get_mut(*gfx_index)
        {
            gfx.gfx.from_builder(builder);
        }
    }

    pub fn remove_gfx(&mut self, renderer: &mut GpuRenderer, index: &GfxType) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
                self.image_storage.remove(*gfx_index);
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::None => {}
        }
    }

    pub fn get_visible(&mut self, index: &GfxType) -> bool {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    return gfx.data.visible;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    return gfx.data.visible;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    return gfx.data.visible;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    return gfx.data.visible;
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    return gfx.data.visible;
                }
            }
            GfxType::None => {}
        }

        false
    }

    pub fn set_visible(&mut self, index: &GfxType, visible: bool) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::None => {}
        }
    }

    pub fn set_override_visible(&mut self, index: &GfxType, visible: Option<bool>) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.data.override_visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.data.override_visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.data.override_visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    gfx.data.override_visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    gfx.data.override_visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::None => {}
        }
    }

    pub fn set_debug(&mut self, index: &GfxType) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.data.debug_track = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.data.debug_track = true;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.data.debug_track = true;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    gfx.data.debug_track = true;
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    gfx.data.debug_track = true;
                }
            }
            GfxType::None => {}
        }
    }

    pub fn set_image(&mut self, index: &GfxType, texture: usize) {
        if let GfxType::Image(gfx_index) = index
            && let Some(gfx) = self.image_storage.get_mut(*gfx_index)
        {
            gfx.gfx.texture = Some(texture);
            gfx.gfx.changed = true;
        }
    }

    pub fn set_color(&mut self, index: &GfxType, color: Color) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.color = color;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_color(color);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_default_color(color);
                }
            }
            _ => {}
        }
    }

    pub fn set_border_color(&mut self, index: &GfxType, color: Color) {
        if let GfxType::Rect(gfx_index) = index
            && let Some(gfx) = self.rect_storage.get_mut(*gfx_index)
        {
            gfx.gfx.set_border_color(color);
        }
    }

    pub fn set_border_width(&mut self, index: &GfxType, width: f32) {
        if let GfxType::Rect(gfx_index) = index
            && let Some(gfx) = self.rect_storage.get_mut(*gfx_index)
        {
            gfx.gfx.set_border_width(width);
        }
    }

    pub fn set_pos_z(&mut self, index: &GfxType, z: f32) {
        let mut pos = self.get_pos(index);
        pos.z = z;
        self.set_pos(index, pos);
    }

    pub fn set_pos(&mut self, index: &GfxType, pos: Vec3) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    //let _ = gfx.gfx.set_pos(pos);
                    gfx.gfx.pos = pos;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    if gfx.gfx.pos == pos {
                        return;
                    }
                    gfx.gfx.set_pos(pos);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    if gfx.gfx.pos == pos {
                        return;
                    }
                    gfx.gfx.set_pos(pos);
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    if gfx.gfx.pos == pos {
                        return;
                    }
                    gfx.gfx.set_pos(pos);
                }
            }
            _ => {}
        }
    }

    pub fn set_override_pos(&mut self, index: &GfxType, pos: Vec3) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_override(pos);
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_pos(pos);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_override(pos);
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_pos(pos);
                }
            }
            _ => {}
        }
    }

    pub fn set_render_layer(&mut self, index: &GfxType, render_layer: u32) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_layer(render_layer);
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_layer(render_layer);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_layer(render_layer);
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_order_layer(render_layer);
                }
            }
            _ => {}
        }
    }

    pub fn set_bound(&mut self, index: &GfxType, bound: Bounds) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            gfx.gfx.set_bounds(bound);
        }
    }

    pub fn set_size(&mut self, index: &GfxType, size: Vec2) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.size = size;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_size(size);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.size = size;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_size(size);
                }
            }
            _ => {}
        }
    }

    pub fn set_buffer_size(&mut self, renderer: &mut GpuRenderer, index: &GfxType, size: Vec2) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            gfx.gfx
                .set_buffer_size(renderer, Some(size.x), Some(size.y));
        }
    }

    pub fn set_uv(&mut self, index: &GfxType, uv: Vec4) {
        if let GfxType::Image(gfx_index) = index
            && let Some(gfx) = self.image_storage.get_mut(*gfx_index)
        {
            let _ = gfx.gfx.set_uv(uv);
        }
    }

    pub fn set_text(&mut self, renderer: &mut GpuRenderer, index: &GfxType, msg: &str) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            gfx.gfx
                .set_text(renderer, msg, &Attrs::new(), Shaping::Advanced, None);
        }
    }

    pub fn set_custom_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        msg: &str,
        attrs: Attrs,
    ) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            gfx.gfx
                .set_text(renderer, msg, &attrs, Shaping::Advanced, None);
        }
    }

    pub fn set_rich_text<'s, 'r, I>(&mut self, renderer: &mut GpuRenderer, index: &GfxType, msg: I)
    where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            gfx.gfx
                .set_rich_text(renderer, msg, &Attrs::new(), Shaping::Advanced, None);
        }
    }

    pub fn set_text_wrap(&mut self, renderer: &mut GpuRenderer, index: &GfxType, can_wrap: bool) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            if can_wrap {
                gfx.gfx.set_wrap(renderer, cosmic_text::Wrap::Word);
            } else {
                gfx.gfx.set_wrap(renderer, cosmic_text::Wrap::None);
            }
        }
    }

    pub fn center_text(&mut self, index: &GfxType) {
        if let GfxType::Text(gfx_index) = index
            && let Some(gfx) = self.text_storage.get_mut(*gfx_index)
        {
            let size = gfx.gfx.measure();
            let bound = gfx.gfx.bounds;
            let textbox_size = bound.right - bound.left;
            gfx.gfx.pos.x = bound.left + ((textbox_size * 0.5) - (size.x * 0.5));
            gfx.gfx.changed = true;
        }
    }

    pub fn get_pos_and_size(&self, index: &GfxType) -> (Vec2, Vec2) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    let pos = gfx.gfx.pos;
                    return (Vec2::new(pos.x, pos.y), gfx.gfx.size);
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    let pos = gfx.gfx.pos;
                    return (Vec2::new(pos.x, pos.y), gfx.gfx.size);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    let pos = gfx.gfx.pos;
                    return (Vec2::new(pos.x, pos.y), gfx.gfx.size);
                }
            }
            _ => {}
        }

        (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))
    }

    pub fn get_pos(&mut self, index: &GfxType) -> Vec3 {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.pos;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.pos;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.pos;
                }
            }
            _ => {}
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn get_size(&self, index: &GfxType) -> Vec2 {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.size;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.size;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.size;
                }
            }
            _ => return Vec2::new(0.0, 0.0),
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn get_uv(&mut self, index: &GfxType) -> Vec4 {
        if let GfxType::Image(gfx_index) = index
            && let Some(gfx) = self.image_storage.get(*gfx_index)
        {
            return gfx.gfx.uv;
        }

        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn get_color(&mut self, index: &GfxType) -> Color {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.color;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.color;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.default_color;
                }
            }
            _ => return Color::rgba(0, 0, 0, 0),
        }

        Color::rgba(0, 0, 0, 0)
    }

    pub fn get_measure(&mut self, index: &GfxType) -> Vec2 {
        match index {
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.measure();
                }
            }
            _ => return Vec2::new(0.0, 0.0),
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn get_override_pos(&self, index: &GfxType) -> DrawOrder {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.order;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.order;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.order;
                }
            }
            GfxType::Mesh(gfx_index) => {
                if let Some(gfx) = self.mesh_storage.get(*gfx_index) {
                    return gfx.gfx.order;
                }
            }
            _ => {}
        }
        DrawOrder::default()
    }

    pub fn set_light_world_color(&mut self, index: &GfxType, color: Vec4) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.gfx.world_color = color;
            gfx.gfx.changed = true;
        }
    }

    pub fn count_area_light(&mut self, index: &GfxType) -> usize {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.gfx.area_lights.len()
        } else {
            0
        }
    }

    pub fn count_directional_light(&mut self, index: &GfxType) -> usize {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.gfx.directional_lights.len()
        } else {
            0
        }
    }

    pub fn count_visible_light(&mut self, index: &GfxType) -> usize {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.visible_lights.len()
        } else {
            0
        }
    }

    pub fn get_mut_area_light(&mut self, index: &GfxType, light: Index) -> Option<&mut AreaLight> {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            return gfx.gfx.get_mut_area_light(light);
        }

        None
    }

    pub fn add_area_light(&mut self, index: &GfxType, light: AreaLight) -> Option<Index> {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            let index = gfx.gfx.insert_area_light(light);
            if let Some(light_index) = index {
                gfx.visible_lights.insert(LightIndex::new(0, light_index));
            }
            return index;
        }

        None
    }

    pub fn add_directional_light(
        &mut self,
        index: &GfxType,
        light: DirectionalLight,
    ) -> Option<Index> {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            let index = gfx.gfx.insert_directional_light(light);
            if let Some(light_index) = index {
                gfx.visible_lights.insert(LightIndex::new(1, light_index));
            }
            return index;
        }

        None
    }

    pub fn remove_area_light(&mut self, index: &GfxType, light_key: Index) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.visible_lights
                .swap_remove(&LightIndex::new(0, light_key));
            gfx.gfx.remove_area_light(light_key);
        }
    }

    pub fn remove_directional_light(&mut self, index: &GfxType, light_key: Index) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
        {
            gfx.visible_lights
                .swap_remove(&LightIndex::new(1, light_key));
            gfx.gfx.remove_directional_light(light_key);
        }
    }

    pub fn set_area_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
        screen_size: Vec2,
    ) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
            && let Some(light_data) = gfx.gfx.get_mut_area_light(light_key)
        {
            light_data.pos = pos;
            let size = light_data.max_distance;

            let light_index = LightIndex::new(0, light_key);

            if pos.x + size >= 0.0
                && pos.x - size <= screen_size.x
                && pos.y + size >= 0.0
                && pos.y - size <= screen_size.y
            {
                gfx.visible_lights.insert(light_index);
            } else {
                gfx.visible_lights.swap_remove(&light_index);
            }
        }
    }

    pub fn set_directional_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
        screen_size: Vec2,
    ) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
            && let Some(light_data) = gfx.gfx.get_mut_directional_light(light_key)
        {
            light_data.pos = pos;
            let size = light_data.max_distance;

            let light_index = LightIndex::new(1, light_key);

            if pos.x + size >= 0.0
                && pos.x - size <= screen_size.x
                && pos.y + size >= 0.0
                && pos.y - size <= screen_size.y
            {
                gfx.visible_lights.insert(light_index);
            } else {
                gfx.visible_lights.swap_remove(&light_index);
            }
        }
    }

    pub fn set_area_light_color(&mut self, index: &GfxType, light_key: Index, color: Color) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
            && let Some(light_data) = gfx.gfx.get_mut_area_light(light_key)
        {
            light_data.color = color;
        }
    }

    pub fn set_directional_light_color(&mut self, index: &GfxType, light_key: Index, color: Color) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
            && let Some(light_data) = gfx.gfx.get_mut_directional_light(light_key)
        {
            light_data.color = color;
        }
    }

    pub fn set_directional_light_dir(&mut self, index: &GfxType, light_key: Index, dir: u8) {
        if let GfxType::Light(gfx_index) = index
            && let Some(gfx) = self.light_storage.get_mut(*gfx_index)
            && let Some(light_data) = gfx.gfx.get_mut_directional_light(light_key)
        {
            light_data.angle = match dir {
                1 => 0.0,   // Right
                2 => 90.0,  // Up
                3 => 180.0, // Left
                _ => 270.0, // Down
            };
        }
    }
}
