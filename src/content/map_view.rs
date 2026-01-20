use camera::controls::FlatControls;
use graphics::*;
use snafu::Backtrace;

use crate::{
    content::widget::{create_label, is_within_area},
    data_types::*,
    gfx_collection::GfxType,
    renderer::{Graphics, SystemHolder},
    resource::GuiTexture,
};

mod attr_preview;
mod autotile;
mod editor;

pub use attr_preview::*;
pub use autotile::*;
pub use editor::*;

#[derive(Default)]
pub struct MapDrag {
    pub in_hold: bool,
    pub start_mouse_pos: Vec2,
    pub start_map_pos: Vec2,
}

pub struct TileSelect {
    pub gfx: GfxType,
    pub frame: usize,

    pub cur_pos: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ViewAttribute {
    pub bg: GfxType,
    pub text: GfxType,
}

pub struct LinkedMap {
    pub map: Map,
    pub bg: GfxType,
}

pub struct MapView {
    pub map: Map,
    pub linked_map: Vec<LinkedMap>,
    pub attribute: Vec<ViewAttribute>,
    pub zones: Vec<GfxType>,
    pub dir_block: Vec<GfxType>,
    pub map_border: [GfxType; 4],
    pub attr_preview: AttrPreview,

    pub attr_visible: bool,
    pub zone_visible: bool,
    pub dirblock_visible: bool,
    pub tile: TileSelect,
    pub drag: MapDrag,
    pub hover_linked_map: Option<usize>,
    pub camera_pos: Vec2,
    pub last_camera_pos: Vec2,
}

impl MapView {
    pub fn new(systems: &mut SystemHolder, map_renderer: &mut MapRenderer) -> Result<Self> {
        let map_pos = Vec2::new(300.0, 60.0);

        let map = if let Some(mut map) = Map::new(
            &mut systems.renderer,
            map_renderer,
            TEXTURE_SIZE,
            map_pos,
            MapZLayers {
                ground: 10.9,
                mask: 10.4,
                mask2: 10.3,
                anim1: 10.8,
                anim2: 10.7,
                anim3: 10.6,
                anim4: 10.5,
                fringe: 10.2,
                fringe2: 10.1,
            },
        ) {
            map.can_render = true;

            map
        } else {
            return Err(EditorError::Other {
                source: OtherError::new("Failed to create Map"),
                backtrace: Backtrace::new(),
            });
        };

        let mut linked_map = Vec::with_capacity(8);
        let map_size = 32.0 * TEXTURE_SIZE as f32;

        for i in 0..8 {
            let linked_map_pos = match i {
                1 => Vec2::new(0.0, map_size),        // Top
                2 => Vec2::new(map_size, map_size),   // Top Right
                3 => Vec2::new(-map_size, 0.0),       // Left
                4 => Vec2::new(map_size, 0.0),        // Right
                5 => Vec2::new(-map_size, -map_size), // Down Left
                6 => Vec2::new(0.0, -map_size),       // Down
                7 => Vec2::new(map_size, -map_size),  // Down Right
                _ => Vec2::new(-map_size, map_size),  // Top Left
            };
            let l_pos = map_pos + linked_map_pos;
            let l_map = if let Some(mut map) = Map::new(
                &mut systems.renderer,
                map_renderer,
                TEXTURE_SIZE,
                l_pos,
                MapZLayers {
                    ground: 11.9,
                    mask: 11.4,
                    mask2: 11.3,
                    anim1: 11.8,
                    anim2: 11.7,
                    anim3: 11.6,
                    anim4: 11.5,
                    fringe: 11.2,
                    fringe2: 11.1,
                },
            ) {
                map.can_render = true;

                map
            } else {
                return Err(EditorError::Other {
                    source: OtherError::new("Failed to create Map"),
                    backtrace: Backtrace::new(),
                });
            };

            let rect = Rect::new(
                &mut systems.renderer,
                Vec3::new(l_pos.x, l_pos.y, ORDER_LINKED_TILE_BG),
                Vec2::new(map_size, map_size),
                Color::rgba(0, 0, 0, 150),
                1,
            );
            let bg = systems.gfx.add_rect(
                rect,
                RENDER_TOP_MAP,
                "Linked Map BG",
                true,
                CameraView::MainView,
            );

            linked_map.push(LinkedMap { map: l_map, bg });
        }

        let image = Image::new(
            Some(systems.resource.interface[GuiTexture::TileSelect as usize]),
            &mut systems.renderer,
            Vec3::new(map_pos.x, map_pos.y, ORDER_TILE_SELECT),
            Vec2::new(20.0, 20.0),
            Vec4::new(0.0, 0.0, 20.0, 20.0),
            4,
        );
        let tile = TileSelect {
            gfx: systems.gfx.add_image(
                image,
                RENDER_IMG,
                "Tile Selection",
                true,
                CameraView::MainView,
            ),
            frame: 0,
            cur_pos: Vec2::new(0.0, 0.0),
        };

        let mut attribute = Vec::with_capacity(MAX_TILE);
        let mut zones = Vec::with_capacity(MAX_TILE);
        let mut dir_block = Vec::with_capacity(MAX_TILE);
        for x in 0..32 {
            for y in 0..32 {
                let tile_size = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
                let attr_zoom_pos = Vec2::new(map_pos.x, map_pos.y);
                let tile_pos = Vec2::new(
                    attr_zoom_pos.x + (tile_size.x * x as f32),
                    attr_zoom_pos.y + (tile_size.y * y as f32),
                );

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG),
                    tile_size,
                    Color::rgba(0, 0, 0, 0),
                    0,
                );
                let bg = systems.gfx.add_rect(
                    rect,
                    RENDER_GUI,
                    "MapView Attribute BG",
                    false,
                    CameraView::MainView,
                );

                let text_size = Vec2::new(tile_size.x, 20.0);
                let text_pos = Vec2::new(
                    tile_pos.x,
                    tile_pos.y + ((tile_size.y - text_size.y) * 0.5).floor(),
                );
                let label = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, ORDER_TILE_BG),
                    text_size,
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + text_size.x,
                        text_pos.y + text_size.y,
                    ),
                    Color::rgb(255, 255, 255),
                    1,
                    16.0,
                    16.0,
                    false,
                );
                let text = systems.gfx.add_text(
                    label,
                    RENDER_GUI_TEXT,
                    "MapView Attribute Text",
                    false,
                    CameraView::MainView,
                );

                attribute.push(ViewAttribute { bg, text });

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG),
                    tile_size,
                    Color::rgba(0, 0, 0, 0),
                    0,
                );
                let gfx = systems.gfx.add_rect(
                    rect,
                    RENDER_GUI,
                    "MapView Zones BG",
                    false,
                    CameraView::MainView,
                );
                zones.push(gfx);

                let img = Image::new(
                    Some(systems.resource.interface[GuiTexture::DirBlock as usize]),
                    &mut systems.renderer,
                    Vec3::new(tile_pos.x, tile_pos.y, ORDER_TILE_BG),
                    Vec2::new(20.0, 20.0),
                    Vec4::new(0.0, 0.0, 20.0, 20.0),
                    1,
                );
                let gfx = systems.gfx.add_image(
                    img,
                    RENDER_GUI2,
                    "MapView DirBlock",
                    false,
                    CameraView::MainView,
                );
                dir_block.push(gfx);
            }
        }

        let mut map_border = [GfxType::default(); 4];
        for (i, gfx) in map_border.iter_mut().enumerate() {
            let set_pos = match i {
                1 => Vec2::new(map_pos.x - 2.0, map_pos.y - 2.0), // Bottom
                2 => Vec2::new(map_pos.x - 2.0, map_pos.y - 2.0), // Left
                3 => Vec2::new(map_pos.x + map_size - 1.0, map_pos.y - 2.0), // Right
                _ => Vec2::new(map_pos.x - 2.0, map_pos.y + map_size - 1.0), // Top
            };
            let set_size = if matches!(i, 2 | 3) {
                Vec2::new(3.0, map_size + 4.0)
            } else {
                Vec2::new(map_size + 4.0, 3.0)
            };

            let rect = Rect::new(
                &mut systems.renderer,
                Vec3::new(set_pos.x, set_pos.y, ORDER_LINKED_TILE_BG),
                set_size,
                Color::rgb(0, 0, 0),
                2,
            );
            *gfx = systems
                .gfx
                .add_rect(rect, RENDER_TOP_MAP, "Border", true, CameraView::MainView);
        }

        Ok(MapView {
            map,
            drag: MapDrag::default(),
            tile,
            attribute,
            attr_visible: false,
            zones,
            zone_visible: false,
            dir_block,
            dirblock_visible: false,
            linked_map,
            hover_linked_map: None,
            map_border,
            attr_preview: AttrPreview::default(),
            camera_pos: Vec2::new(0.0, 0.0),
            last_camera_pos: Vec2::new(0.0, 0.0),
        })
    }

    pub fn set_map_drag(&mut self, mouse_pos: Vec2) {
        self.drag = MapDrag {
            in_hold: true,
            start_mouse_pos: mouse_pos,
            start_map_pos: self.map.pos,
        };
    }

    pub fn clear_map_drag(&mut self) {
        self.drag = MapDrag::default();
    }

    pub fn set_attr_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        self.attr_visible = visible;
        for attribute in self.attribute.iter() {
            systems.gfx.set_visible(&attribute.bg, visible);
            systems.gfx.set_visible(&attribute.text, visible);
        }
    }

    pub fn set_zone_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        self.zone_visible = visible;
        for zone in self.zones.iter() {
            systems.gfx.set_visible(zone, visible);
        }
    }

    pub fn set_dirblock_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        self.dirblock_visible = visible;
        for zone in self.dir_block.iter() {
            systems.gfx.set_visible(zone, visible);
        }
    }

    pub fn hover_tile(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        if self.drag.in_hold {
            return;
        }

        let map_size = (Vec2::new(32.0 * TEXTURE_SIZE as f32, 32.0 * TEXTURE_SIZE as f32)
            * systems.config.zoom)
            .floor();

        let start_pos = (self.map.pos * systems.config.zoom).round() + self.camera_pos;
        if is_within_area(mouse_pos, start_pos, map_size) {
            let selecting_pos = mouse_pos - start_pos;
            let tile_size = (TEXTURE_SIZE as f32 * systems.config.zoom).round();
            let tile_pos = Vec2::new(
                (selecting_pos.x / tile_size).floor().min(31.0),
                (selecting_pos.y / tile_size).floor().min(31.0),
            );

            self.tile.cur_pos = tile_pos;
            let cursor_tile_pos = Vec2::new(
                tile_pos.x * TEXTURE_SIZE as f32,
                tile_pos.y * TEXTURE_SIZE as f32,
            );

            systems.gfx.set_pos(
                &self.tile.gfx,
                Vec3::new(
                    self.map.pos.x + cursor_tile_pos.x,
                    self.map.pos.y + cursor_tile_pos.y,
                    ORDER_TILE_SELECT,
                ),
            );
        }
    }

    pub fn update_tile_frame(&mut self, systems: &mut SystemHolder) {
        self.tile.frame += 1;
        if self.tile.frame > 3 {
            self.tile.frame = 0;
        }

        systems.gfx.set_uv(
            &self.tile.gfx,
            Vec4::new(20.0 * self.tile.frame as f32, 0.0, 20.0, 20.0),
        );
    }

    pub fn update_map_drag(
        &mut self,
        graphics: &mut Graphics<FlatControls>,
        mouse_pos: Vec2,
    ) -> Vec2 {
        if !self.drag.in_hold {
            return self.camera_pos;
        }

        let difference = self.camera_pos + (mouse_pos - self.drag.start_mouse_pos);

        let input = graphics.system.controls_mut().inputs_mut();
        input.translation.x = difference.x;
        input.translation.y = difference.y;

        difference
    }

    pub fn adjust_map_by_zoom(
        &mut self,
        systems: &mut SystemHolder,
        graphics: &mut Graphics<FlatControls>,
        new_zoom: f32,
    ) {
        if new_zoom == systems.config.zoom {
            return;
        }

        let zoom_in = new_zoom > systems.config.zoom;
        let difference = self.camera_pos
            + if zoom_in {
                -Vec2::new(60.0, 30.0)
            } else {
                Vec2::new(60.0, 30.0)
            };

        let input = graphics.system.controls_mut().inputs_mut();
        input.translation.x = difference.x;
        input.translation.y = difference.y;

        self.camera_pos = difference;
    }

    pub fn in_linked_area(&self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        let map_size = 32.0 * TEXTURE_SIZE as f32;
        for (i, map) in self.linked_map.iter().enumerate() {
            let pos = (map.map.pos * systems.config.zoom).floor();
            let size = Vec2::new(
                (map_size * systems.config.zoom).floor(),
                (map_size * systems.config.zoom).floor(),
            );
            if is_within_area(mouse_pos, pos + self.camera_pos, size) {
                return Some(i);
            }
        }
        None
    }
}
