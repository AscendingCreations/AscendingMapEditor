use graphics::*;

use crate::{
    content::widget::{option_list::*, scrollbar::*},
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
    resource::GuiTexture,
};

pub struct TileSelection {
    gfx: [GfxType; 4],
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub in_hold: bool,
}

pub struct TilesetWindow {
    pub visible: bool,
    bg: GfxType,
    tileset: GfxType,
    lower_bg: GfxType,
    pub tile_list: OptionList,
    pub scrollbar: Scrollbar,
    pub selection: TileSelection,

    content_y_size: f32,
    pub cur_tileset: usize,
    pub start_pos: Vec2,
    pub area_size: Vec2,
}

impl TilesetWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            start_pos.x + (5.0 * systems.scale as f32).floor(),
            start_pos.y + ((area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );

        let rect = Rect::new(
            &mut systems.renderer,
            tileset_pos,
            tileset_size,
            Color::rgb(80, 80, 80),
            0,
        );
        let bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI, "Tileset BG", true, CameraView::SubView1);

        let content_y_size = tileset_size.y + (54.0 * systems.scale as f32).floor();
        let scroll_value = (content_y_size - area_size.y).max(0.0) as usize;

        let bar_size = (area_size.y / systems.scale as f32).floor() - 20.0;
        let min_bar_size = (bar_size * 0.4).floor();
        let scrollbar = Scrollbar::new(
            systems,
            start_pos + Vec2::new(area_size.x - (14.0 * systems.scale as f32).floor(), 0.0),
            Vec2::new(0.0, 10.0),
            bar_size,
            10.0,
            true,
            ORDER_WINDOW_CONTENT,
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 3,
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                hover_color: Color::rgb(180, 180, 180),
                hold_color: Color::rgb(120, 120, 120),
                radius: 0.0,
            },
            Some(ScrollbarBackground {
                color: Color::rgb(90, 90, 90),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            scroll_value,
            min_bar_size,
            false,
            true,
            None,
            true,
            None,
        );

        let img = Image::new(
            Some(systems.resource.tilesheet[0].img),
            &mut systems.renderer,
            tileset_pos,
            tileset_size,
            Vec4::new(0.0, 0.0, tileset_size.x, tileset_size.y),
            1,
        );
        let tileset =
            systems
                .gfx
                .add_image(img, RENDER_GUI2, "Tileset", true, CameraView::SubView1);

        let tileset_name_list: Vec<String> = systems
            .resource
            .tilesheet
            .iter()
            .map(|data| data.name.clone())
            .collect();
        let list_size = tileset_name_list.len();

        let tile_list = OptionList::new(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            Vec2::new(0.0, 0.0),
            Vec2::new((tileset_size.x / systems.scale as f32).floor(), 24.0),
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
            tileset_name_list,
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
            true,
        );

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
            Vec2::new(area_size.x, start_pos.y),
            Color::rgb(130, 130, 130),
            0,
        );
        let lower_bg = systems
            .gfx
            .add_rect(rect, RENDER_GUI2, "BG", true, CameraView::SubView1);

        let mut gfx = [GfxType::default(); 4];
        for (i, gfx_slot) in gfx.iter_mut().enumerate() {
            let img = Image::new(
                Some(systems.resource.interface[GuiTexture::TilesheetSelect as usize]),
                &mut systems.renderer,
                Vec3::new(
                    tileset_pos.x
                        + (match i {
                            1 | 3 => 10.0,
                            _ => 0.0,
                        } * systems.scale as f32)
                            .floor(),
                    tileset_pos.y
                        + (match i {
                            2 | 3 => 0.0,
                            _ => 10.0,
                        } * systems.scale as f32)
                            .floor(),
                    ORDER_WINDOW_CONTENT,
                ),
                (Vec2::new(10.0, 10.0) * systems.scale as f32).floor(),
                Vec4::new(
                    match i {
                        1 | 3 => 10.0,
                        _ => 0.0,
                    },
                    match i {
                        2 | 3 => 10.0,
                        _ => 0.0,
                    },
                    10.0,
                    10.0,
                ),
                2,
            );
            *gfx_slot = systems.gfx.add_image(
                img,
                RENDER_GUI2,
                "Tileset Selection",
                true,
                CameraView::SubView1,
            );
        }

        let selection = TileSelection {
            gfx,
            start_pos: Vec2::new(0.0, 0.0),
            end_pos: Vec2::new(0.0, 0.0),
            in_hold: false,
        };

        TilesetWindow {
            visible: true,
            bg,
            tileset,
            lower_bg,
            tile_list,
            scrollbar,
            cur_tileset: 0,
            start_pos,
            area_size,
            content_y_size,
            selection,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            start_pos.x + (5.0 * systems.scale as f32).floor(),
            start_pos.y + ((area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );

        systems.gfx.set_pos(&self.bg, tileset_pos);
        systems.gfx.set_size(&self.bg, tileset_size);

        self.content_y_size = tileset_size.y + (54.0 * systems.scale as f32).floor();
        let scroll_value = (self.content_y_size - area_size.y).max(0.0) as usize;

        let bar_size = (area_size.y / systems.scale as f32).floor() - 20.0;
        let min_bar_size = (bar_size * 0.4).floor();
        self.scrollbar.set_pos(
            systems,
            start_pos + Vec2::new(area_size.x - (14.0 * systems.scale as f32).floor(), 0.0),
        );
        self.scrollbar
            .set_size(systems, bar_size, min_bar_size, 10.0);
        self.scrollbar.set_value(systems, 0);
        self.scrollbar.set_max_value(systems, scroll_value);
        systems.gfx.set_pos(&self.tileset, tileset_pos);
        self.tile_list.move_window(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );

        systems
            .gfx
            .set_size(&self.lower_bg, Vec2::new(area_size.x, start_pos.y));
        systems.gfx.set_pos(
            &self.lower_bg,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
        );

        self.start_pos = start_pos;
        self.area_size = area_size;

        self.select_tile(systems, self.selection.start_pos, self.selection.end_pos);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        systems.gfx.set_visible(&self.bg, visible);
        self.scrollbar.set_visible(systems, visible);
        systems.gfx.set_visible(&self.tileset, visible);
        self.tile_list.set_visible(systems, visible);
        for gfx in self.selection.gfx.iter() {
            systems.gfx.set_visible(gfx, visible);
        }
    }

    pub fn change_tileset(&mut self, systems: &mut SystemHolder, tileset: usize) {
        if self.cur_tileset == tileset {
            return;
        }
        self.cur_tileset = tileset;

        systems.gfx.remove_gfx(&mut systems.renderer, &self.tileset);

        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.start_pos.y
                + ((self.area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y)
                + self.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        let img = Image::new(
            Some(systems.resource.tilesheet[tileset].img),
            &mut systems.renderer,
            tileset_pos,
            tileset_size,
            Vec4::new(0.0, 0.0, tileset_size.x, tileset_size.y),
            1,
        );
        self.tileset = systems.gfx.add_image(
            img,
            RENDER_GUI2,
            "Tileset",
            self.visible,
            CameraView::SubView1,
        );
    }

    pub fn update_content(&mut self, systems: &mut SystemHolder) {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.start_pos.y
                + ((self.area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y)
                + self.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        systems.gfx.set_pos(&self.bg, tileset_pos);
        systems.gfx.set_pos(&self.tileset, tileset_pos);
        self.tile_list.move_window(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );

        self.select_tile(systems, self.selection.start_pos, self.selection.end_pos);
    }

    pub fn select_tile(&mut self, systems: &mut SystemHolder, start_pos: Vec2, end_pos: Vec2) {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.start_pos.y
                + ((self.area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y)
                + self.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        let size = Vec2::new(
            if start_pos.x > end_pos.x {
                start_pos.x - end_pos.x
            } else {
                end_pos.x - start_pos.x
            } + 1.0,
            if start_pos.y > end_pos.y {
                start_pos.y - end_pos.y
            } else {
                end_pos.y - start_pos.y
            } + 1.0,
        );
        let pos = Vec2::new(
            if start_pos.x > end_pos.x {
                end_pos.x
            } else {
                start_pos.x
            },
            if start_pos.y > end_pos.y {
                end_pos.y
            } else {
                start_pos.y
            },
        );

        let tile_size = (20.0 * systems.scale as f32).floor();
        let base_pos = Vec2::new(
            tileset_pos.x + (pos.x * tile_size),
            tileset_pos.y + (pos.y * tile_size),
        );

        for (i, gfx) in self.selection.gfx.iter().enumerate() {
            systems.gfx.set_pos(
                gfx,
                Vec3::new(
                    base_pos.x
                        + match i {
                            1 | 3 => (size.x * tile_size) - (10.0 * systems.scale as f32).floor(),
                            _ => 0.0,
                        },
                    base_pos.y
                        + match i {
                            0 | 1 => (size.y * tile_size) - (10.0 * systems.scale as f32).floor(),
                            _ => 0.0,
                        },
                    tileset_pos.z,
                ),
            );
        }
    }
}
