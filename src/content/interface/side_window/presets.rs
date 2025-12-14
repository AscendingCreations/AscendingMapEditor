use graphics::*;

use crate::{
    content::widget::*,
    data_types::*,
    database::{PresetFrames, PresetPos, PresetTypeList},
    gfx_collection::GfxType,
    renderer::SystemHolder,
    resource::GuiTexture,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PresetWindowType {
    Base,
    Editor,
}

pub struct PresetTileSelection {
    gfx: [GfxType; 4],
    blocker: GfxType,
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub in_hold: bool,
}

pub struct PresetBase {
    seperator: GfxType,
    pub preview_name: GfxType,
    pub preview_bg: GfxType,
    pub preview_info: GfxType,
    pub edit_button: Button,
    pub preset_list: TextList,
    pub preview: [GfxType; 4],
    pub preset_type: PresetTypeList,
    pub frames: [PresetFrames; 4],
    pub cur_frame: usize,
}

pub struct PresetEditor {
    bg: GfxType,
    tileset: GfxType,
    lower_bg: GfxType,
    pub tile_list: OptionList,
    pub type_list: OptionList,
    pub scrollbar: Scrollbar,
    pub cur_type: PresetTypeList,
    pub frame_scroll: Scrollbar,
    pub frame_label: GfxType,
    pub frames: [PresetFrames; 4],

    content_y_size: f32,
    pub cur_tileset: usize,
    pub start_pos: Vec2,
    pub area_size: Vec2,

    pub save_button: Button,
    pub cancel_button: Button,
    pub selection: PresetTileSelection,
}

pub struct PresetWindow {
    pub visible: bool,
    pub window_type: PresetWindowType,
    pub base: PresetBase,
    pub editor: PresetEditor,
    pub selected_index: usize,
}

impl PresetWindow {
    pub fn new(systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) -> Self {
        let preset_preview_size = Vec2::new(area_size.x, (200.0 * systems.scale as f32).floor());
        let bg_pos = Vec3::new(
            start_pos.x,
            start_pos.y + (area_size.y - preset_preview_size.y),
            ORDER_WINDOW_CONTENT,
        );

        let text_pos = Vec3::new(
            start_pos.x,
            (bg_pos.y + preset_preview_size.y) - (30.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let text_size = Vec2::new(area_size.x, (20.0 * systems.scale as f32).floor());
        let text = create_label(
            systems,
            text_pos,
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
            true,
        );
        let preview_name = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Preview Name",
            false,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &preview_name, "Name");
        systems.gfx.center_text(&preview_name);

        let text_pos = Vec3::new(
            start_pos.x,
            bg_pos.y + (50.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let text_size = Vec2::new(area_size.x, (20.0 * systems.scale as f32).floor());
        let text = create_label(
            systems,
            text_pos,
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
            true,
        );
        let preview_info = systems.gfx.add_text(
            text,
            RENDER_GUI_TEXT,
            "Preview Info",
            false,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &preview_info, "Info Text");
        systems.gfx.center_text(&preview_info);

        let preview_pos = Vec2::new(
            bg_pos.x + ((preset_preview_size.x - 104.0) * 0.5).floor(),
            bg_pos.y
                + ((preset_preview_size.y - 64.0) * 0.5).floor()
                + (20.0 * systems.scale as f32).floor(),
        );
        let img = Image::new(
            Some(systems.resource.interface[GuiTexture::PresetPreview as usize]),
            &mut systems.renderer,
            Vec3::new(preview_pos.x, preview_pos.y, ORDER_WINDOW_CONTENT),
            Vec2::new(104.0, 64.0),
            Vec4::new(0.0, 0.0, 104.0, 64.0),
            1,
        );
        let preview_bg =
            systems
                .gfx
                .add_image(img, RENDER_GUI2, "Preview BG", false, CameraView::SubView1);

        let edit_button = Button::new(
            systems,
            ButtonType::Rect(ButtonRect {
                rect_color: Color::rgb(80, 80, 80),
                got_border: true,
                border_color: Color::rgb(0, 0, 0),
                border_radius: 0.0,
                hover_change: ButtonChangeType::ColorChange(Color::rgb(50, 50, 50)),
                click_change: ButtonChangeType::ColorChange(Color::rgb(80, 80, 80)),
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            ButtonContentType::Text(ButtonContentText {
                text: "Edit".to_string(),
                pos: Vec2::new(0.0, 0.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 2,
                buffer_layer: RENDER_GUI_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            Vec2::new(bg_pos.x, bg_pos.y),
            Vec2::new(10.0, 5.0),
            ORDER_WINDOW_CONTENT,
            Vec2::new((area_size.x / systems.scale as f32).floor() - 20.0, 24.0),
            1,
            RENDER_GUI,
            false,
            None,
            false,
        );

        let separator_rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(
                start_pos.x + (5.0 * systems.scale as f32).floor(),
                bg_pos.y - (5.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT,
            ),
            Vec2::new(
                area_size.x - (10.0 * systems.scale as f32).floor(),
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

        let mut list_size = Vec2::new(
            (area_size.x / systems.scale as f32).floor() - 20.0,
            area_size.y - preset_preview_size.y - (20.0 * systems.scale as f32).floor(),
        );
        let max_visible_list = ((list_size.y - (10.0 * systems.scale as f32).floor())
            / (20.0 * systems.scale as f32).floor())
        .floor() as usize;
        list_size.y = (max_visible_list as f32 * 20.0) + 10.0;

        let mut preset_text = Vec::with_capacity(MAX_PRESETS);
        for i in 0..MAX_PRESETS {
            preset_text.push(format!("{}: -", i + 1));
        }

        let mut preset_list = TextList::new(
            systems,
            Vec2::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_pos.y + (10.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW_CONTENT,
            list_size,
            false,
            TextListBG::Rect(TextListBGRect {
                color: Color::rgb(85, 85, 85),
                buffer_layer: RENDER_GUI,
                order_layer: 2,
                got_border: false,
                border_color: Color::rgb(0, 0, 0),
                radius: 0.0,
            }),
            ScrollbarRect {
                color: Color::rgb(150, 150, 150),
                buffer_layer: RENDER_GUI,
                order_layer: 3,
                got_border: false,
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
            preset_text,
            TextListData {
                selection_bufferlayer: RENDER_GUI,
                text_bufferlayer: RENDER_GUI_TEXT,
                selection_orderlayer: 4,
                text_orderlayer: 5,
                selection_color: SelectionColor {
                    normal: Color::rgb(85, 85, 85),
                    hover: Color::rgb(120, 120, 120),
                    selected: Color::rgb(60, 60, 60),
                },
                text_color: SelectionColor {
                    normal: Color::rgb(255, 255, 255),
                    hover: Color::rgb(255, 255, 255),
                    selected: Color::rgb(255, 255, 255),
                },
                max_list: max_visible_list,
            },
        );
        preset_list.set_select(systems, Some(0), true);

        let base = PresetBase {
            edit_button,
            seperator,
            preview_name,
            preview_info,
            preview_bg,
            preset_list,
            preview: [GfxType::None; 4],
            preset_type: PresetTypeList::Normal,
            frames: [PresetFrames::default(); 4],
            cur_frame: 0,
        };

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
            .add_rect(rect, RENDER_GUI, "Tileset BG", false, CameraView::SubView1);

        let content_y_size = tileset_size.y + (165.0 * systems.scale as f32).floor();
        let scroll_value = (content_y_size - area_size.y).max(0.0) as usize;

        let scrollbar_rect = ScrollbarRect {
            color: Color::rgb(150, 150, 150),
            buffer_layer: RENDER_GUI,
            order_layer: 3,
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            hover_color: Color::rgb(180, 180, 180),
            hold_color: Color::rgb(120, 120, 120),
            radius: 0.0,
        };
        let scrollbar_bg = ScrollbarBackground {
            color: Color::rgb(90, 90, 90),
            buffer_layer: RENDER_GUI,
            order_layer: 2,
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            radius: 0.0,
        };

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
            scrollbar_rect,
            Some(scrollbar_bg),
            scroll_value,
            min_bar_size,
            false,
            false,
            None,
            true,
            None,
        );

        let bar_size =
            (tileset_size.x / systems.scale as f32).floor() - (50.0 * systems.scale as f32).floor();
        let min_bar_size = (bar_size * 0.4).floor();
        let frame_scroll = Scrollbar::new(
            systems,
            Vec2::new(
                tileset_pos.x + (50.0 * systems.scale as f32).floor(),
                tileset_pos.y - (35.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 10.0),
            bar_size,
            20.0,
            false,
            ORDER_WINDOW_CONTENT,
            scrollbar_rect,
            Some(scrollbar_bg),
            3,
            min_bar_size,
            false,
            false,
            None,
            true,
            None,
        );

        let frame_text_pos = Vec3::new(
            tileset_pos.x,
            tileset_pos.y - (25.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let frame_text_size = (Vec2::new(50.0, 20.0) * systems.scale as f32).floor();
        let frame_text = create_label(
            systems,
            frame_text_pos,
            frame_text_size,
            Bounds::new(
                frame_text_pos.x,
                frame_text_pos.y,
                frame_text_pos.x + frame_text_size.x,
                frame_text_pos.y + frame_text_size.y,
            ),
            Color::rgb(255, 255, 255),
            1,
            16.0,
            16.0,
            true,
        );
        let frame_label = systems.gfx.add_text(
            frame_text,
            RENDER_GUI_TEXT,
            "Frame Label",
            false,
            CameraView::SubView1,
        );
        systems
            .gfx
            .set_text(&mut systems.renderer, &frame_label, "Frm: 1");

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
                .add_image(img, RENDER_GUI2, "Tileset", false, CameraView::SubView1);

        let tileset_name_list: Vec<String> = systems
            .resource
            .tilesheet
            .iter()
            .map(|data| data.name.clone())
            .collect();
        let list_size = tileset_name_list.len();

        let option_list_color = OptionListColor {
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
        };

        let tile_list = OptionList::new(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            Vec2::new(0.0, 0.0),
            Vec2::new((tileset_size.x / systems.scale as f32).floor(), 24.0),
            option_list_color,
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
            false,
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
            .add_rect(rect, RENDER_GUI2, "BG", false, CameraView::SubView1);

        let mut gfx = [GfxType::None; 4];
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
                false,
                CameraView::SubView1,
            );
        }

        let type_list = OptionList::new(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (58.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            Vec2::new((tileset_size.x / systems.scale as f32).floor(), 24.0),
            option_list_color,
            vec![
                "Normal".to_string(),
                "Animated".to_string(),
                "Auto Tile".to_string(),
                "Auto Tile Animated".to_string(),
            ],
            4,
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
            false,
        );

        let button_rect = ButtonRect {
            rect_color: Color::rgb(80, 80, 80),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(50, 50, 50)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(80, 80, 80)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::None,
        };

        let save_button = Button::new(
            systems,
            ButtonType::Rect(button_rect),
            ButtonContentType::Text(ButtonContentText {
                text: "Save".to_string(),
                pos: Vec2::new(0.0, 0.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 2,
                buffer_layer: RENDER_GUI_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (85.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW_CONTENT,
            Vec2::new((tileset_size.x / systems.scale as f32).floor(), 24.0),
            1,
            RENDER_GUI,
            false,
            None,
            false,
        );

        let cancel_button = Button::new(
            systems,
            ButtonType::Rect(button_rect),
            ButtonContentType::Text(ButtonContentText {
                text: "Cancel".to_string(),
                pos: Vec2::new(0.0, 0.0),
                color: Color::rgb(255, 255, 255),
                order_layer: 2,
                buffer_layer: RENDER_GUI_TEXT,
                hover_change: ButtonChangeType::None,
                click_change: ButtonChangeType::None,
                alert_change: ButtonChangeType::None,
                disable_change: ButtonChangeType::None,
            }),
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (110.0 * systems.scale as f32).floor(),
            ),
            Vec2::new(0.0, 0.0),
            ORDER_WINDOW_CONTENT,
            Vec2::new((tileset_size.x / systems.scale as f32).floor(), 24.0),
            1,
            RENDER_GUI,
            false,
            None,
            false,
        );

        let mut gfx = [GfxType::None; 4];
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
                false,
                CameraView::SubView1,
            );
        }

        let img = Image::new(
            Some(systems.resource.interface[GuiTexture::PreviewBlocker as usize]),
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_WINDOW_CONTENT),
            (Vec2::new(40.0, 20.0) * systems.scale as f32).floor(),
            Vec4::new(0.0, 0.0, 40.0, 20.0),
            2,
        );
        let blocker =
            systems
                .gfx
                .add_image(img, RENDER_GUI2, "Blocker", false, CameraView::SubView1);

        let selection = PresetTileSelection {
            gfx,
            blocker,
            start_pos: Vec2::new(0.0, 0.0),
            end_pos: Vec2::new(0.0, 0.0),
            in_hold: false,
        };

        let editor = PresetEditor {
            bg,
            tileset,
            lower_bg,
            tile_list,
            type_list,
            scrollbar,
            cur_tileset: 0,
            start_pos,
            area_size,
            content_y_size,
            save_button,
            cancel_button,
            selection,
            cur_type: PresetTypeList::Normal,
            frame_scroll,
            frame_label,
            frames: [PresetFrames::default(); 4],
        };

        PresetWindow {
            visible: false,
            window_type: PresetWindowType::Base,
            base,
            editor,
            selected_index: 0,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder, start_pos: Vec2, area_size: Vec2) {
        let preset_preview_size = Vec2::new(area_size.x, (200.0 * systems.scale as f32).floor());
        let bg_pos = Vec3::new(
            start_pos.x,
            start_pos.y + (area_size.y - preset_preview_size.y),
            ORDER_WINDOW_CONTENT,
        );

        self.base
            .edit_button
            .set_pos(systems, Vec2::new(bg_pos.x, bg_pos.y));

        systems.gfx.set_size(
            &self.base.seperator,
            Vec2::new(
                area_size.x - (10.0 * systems.scale as f32).floor(),
                (2.0 * systems.scale as f32).floor(),
            ),
        );
        systems.gfx.set_pos(
            &self.base.seperator,
            Vec3::new(
                start_pos.x + (5.0 * systems.scale as f32).floor(),
                bg_pos.y - (5.0 * systems.scale as f32).floor(),
                ORDER_WINDOW_CONTENT,
            ),
        );

        let text_pos = Vec3::new(
            start_pos.x,
            (bg_pos.y + preset_preview_size.y) - (30.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let text_size = Vec2::new(area_size.x, (20.0 * systems.scale as f32).floor());
        systems.gfx.set_pos(&self.base.preview_name, text_pos);
        systems.gfx.set_bound(
            &self.base.preview_name,
            Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            ),
        );
        systems.gfx.center_text(&self.base.preview_name);

        let text_pos = Vec3::new(
            start_pos.x,
            bg_pos.y + (50.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let text_size = Vec2::new(area_size.x, (20.0 * systems.scale as f32).floor());
        systems.gfx.set_pos(&self.base.preview_info, text_pos);
        systems.gfx.set_bound(
            &self.base.preview_info,
            Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + text_size.x,
                text_pos.y + text_size.y,
            ),
        );
        systems.gfx.center_text(&self.base.preview_info);

        let preview_pos = Vec2::new(
            bg_pos.x + ((preset_preview_size.x - 104.0) * 0.5).floor(),
            bg_pos.y
                + ((preset_preview_size.y - 64.0) * 0.5).floor()
                + (20.0 * systems.scale as f32).floor(),
        );
        systems.gfx.set_pos(
            &self.base.preview_bg,
            Vec3::new(preview_pos.x, preview_pos.y, ORDER_WINDOW_CONTENT),
        );

        let tile_size = (20.0 * systems.scale as f32).floor();
        for (i, gfx) in self.base.preview.iter_mut().enumerate() {
            let p_type = self.base.preset_type;
            let frame = self.base.frames[i];

            if i > 0
                && matches!(
                    p_type,
                    PresetTypeList::Animated | PresetTypeList::AutotileAnimated
                )
            {
                continue;
            }

            let size = if matches!(
                p_type,
                PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
            ) {
                Vec2::new(5.0, 3.0)
            } else {
                Vec2::new(
                    frame.start.x.abs_diff(frame.end.x) as f32 + 1.0,
                    frame.start.y.abs_diff(frame.end.y) as f32 + 1.0,
                )
            };

            let preview_pos = systems.gfx.get_pos(&self.base.preview_bg);

            let offset_pos = {
                let preview_size = (size - Vec2::ONE).max(Vec2::ONE);
                (Vec2::new(3.0, 2.0) - preview_size).max(Vec2::ZERO)
            };

            systems.gfx.set_pos(
                gfx,
                Vec3::new(
                    preview_pos.x
                        + (2.0 * systems.scale as f32).floor()
                        + (offset_pos.x * tile_size),
                    preview_pos.y
                        + (2.0 * systems.scale as f32).floor()
                        + (offset_pos.y * tile_size),
                    ORDER_WINDOW_CONTENT,
                ),
            );
        }

        let mut list_size = Vec2::new(
            (area_size.x / systems.scale as f32).floor() - 20.0,
            area_size.y - preset_preview_size.y - (20.0 * systems.scale as f32).floor(),
        );
        let max_visible_list = ((list_size.y - (10.0 * systems.scale as f32).floor())
            / (20.0 * systems.scale as f32).floor())
        .floor() as usize;
        list_size.y = (max_visible_list as f32 * 20.0) + 10.0;

        self.base.preset_list.set_pos(
            systems,
            Vec2::new(
                start_pos.x + (10.0 * systems.scale as f32).floor(),
                start_pos.y + (10.0 * systems.scale as f32).floor(),
            ),
        );
        self.base
            .preset_list
            .set_size(systems, list_size, max_visible_list);
        self.base
            .preset_list
            .set_select(systems, Some(self.selected_index), true);

        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            start_pos.x + (5.0 * systems.scale as f32).floor(),
            start_pos.y + ((area_size.y - (34.0 * systems.scale as f32).floor()) - tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );

        systems.gfx.set_pos(&self.editor.bg, tileset_pos);
        systems.gfx.set_size(&self.editor.bg, tileset_size);

        self.editor.content_y_size = tileset_size.y + (165.0 * systems.scale as f32).floor();
        let scroll_value = (self.editor.content_y_size - area_size.y).max(0.0) as usize;

        let bar_size = (area_size.y / systems.scale as f32).floor() - 20.0;
        let min_bar_size = (bar_size * 0.4).floor();
        self.editor.scrollbar.set_pos(
            systems,
            start_pos + Vec2::new(area_size.x - (14.0 * systems.scale as f32).floor(), 0.0),
        );
        self.editor
            .scrollbar
            .set_size(systems, bar_size, min_bar_size, 10.0);
        self.editor.scrollbar.set_value(systems, 0);
        self.editor.scrollbar.set_max_value(systems, scroll_value);

        let bar_size =
            (tileset_size.x / systems.scale as f32).floor() - (50.0 * systems.scale as f32).floor();
        let min_bar_size = (bar_size * 0.4).floor();
        self.editor.frame_scroll.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x + (50.0 * systems.scale as f32).floor(),
                tileset_pos.y - (35.0 * systems.scale as f32).floor(),
            ),
        );
        self.editor
            .frame_scroll
            .set_size(systems, bar_size, min_bar_size, 20.0);
        self.editor.frame_scroll.set_value(systems, 0);

        let frame_text_pos = Vec3::new(
            tileset_pos.x,
            tileset_pos.y - (25.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let frame_text_size = (Vec2::new(50.0, 20.0) * systems.scale as f32).floor();
        systems
            .gfx
            .set_pos(&self.editor.frame_label, frame_text_pos);
        systems.gfx.set_bound(
            &self.editor.frame_label,
            Bounds::new(
                frame_text_pos.x,
                frame_text_pos.y,
                frame_text_pos.x + frame_text_size.x,
                frame_text_pos.y + frame_text_size.y,
            ),
        );

        systems.gfx.set_pos(&self.editor.tileset, tileset_pos);
        self.editor.tile_list.move_window(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );
        self.editor.type_list.move_window(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (58.0 * systems.scale as f32).floor(),
            ),
            ORDER_WINDOW_CONTENT,
        );
        self.editor.save_button.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (85.0 * systems.scale as f32).floor(),
            ),
        );
        self.editor.cancel_button.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (110.0 * systems.scale as f32).floor(),
            ),
        );

        systems
            .gfx
            .set_size(&self.editor.lower_bg, Vec2::new(area_size.x, start_pos.y));
        systems.gfx.set_pos(
            &self.editor.lower_bg,
            Vec3::new(start_pos.x, 0.0, ORDER_WINDOW_CONTENT2),
        );

        self.editor.start_pos = start_pos;
        self.editor.area_size = area_size;

        let end_pos = if matches!(
            self.editor.cur_type,
            PresetTypeList::AutotileAnimated | PresetTypeList::AutoTile
        ) {
            self.editor.selection.start_pos + Vec2::new(4.0, 2.0)
        } else {
            self.editor.selection.start_pos
        };
        self.select_tile(systems, self.editor.selection.start_pos, end_pos);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
        self.base.edit_button.set_visible(systems, visible);
        systems.gfx.set_visible(&self.base.seperator, visible);
        systems.gfx.set_visible(&self.base.preview_name, visible);
        systems.gfx.set_visible(&self.base.preview_info, visible);
        systems.gfx.set_visible(&self.base.preview_bg, visible);
        self.base.preset_list.set_visible(systems, visible, false);
        self.window_type = PresetWindowType::Base;

        systems.gfx.set_visible(&self.editor.bg, false);
        self.editor.scrollbar.set_visible(systems, false);
        self.editor.frame_scroll.set_visible(systems, false);
        systems.gfx.set_visible(&self.editor.tileset, false);
        systems.gfx.set_visible(&self.editor.frame_label, false);
        self.editor.tile_list.set_visible(systems, false);
        self.editor.save_button.set_visible(systems, false);
        self.editor.cancel_button.set_visible(systems, false);
        for gfx in self.editor.selection.gfx.iter() {
            systems.gfx.set_visible(gfx, false);
        }
        systems
            .gfx
            .set_visible(&self.editor.selection.blocker, false);
        self.editor.type_list.set_visible(systems, false);
        for gfx in self.base.preview.iter() {
            systems.gfx.set_visible(gfx, false);
        }
    }

    pub fn switch_state(&mut self, systems: &mut SystemHolder, state: PresetWindowType) {
        self.window_type = state;

        match state {
            PresetWindowType::Base => {
                self.base.edit_button.set_visible(systems, self.visible);
                systems.gfx.set_visible(&self.base.seperator, self.visible);
                systems
                    .gfx
                    .set_visible(&self.base.preview_name, self.visible);
                systems
                    .gfx
                    .set_visible(&self.base.preview_info, self.visible);
                systems.gfx.set_visible(&self.base.preview_bg, self.visible);
                self.base
                    .preset_list
                    .set_visible(systems, self.visible, false);
                for gfx in self.base.preview.iter() {
                    systems.gfx.set_visible(gfx, self.visible);
                }

                systems.gfx.set_visible(&self.editor.bg, false);
                self.editor.scrollbar.set_visible(systems, false);
                self.editor.frame_scroll.set_visible(systems, false);
                systems.gfx.set_visible(&self.editor.tileset, false);
                self.editor.tile_list.set_visible(systems, false);
                self.editor.type_list.set_visible(systems, false);
                self.editor.save_button.set_visible(systems, false);
                self.editor.cancel_button.set_visible(systems, false);
                for gfx in self.editor.selection.gfx.iter() {
                    systems.gfx.set_visible(gfx, false);
                }
                systems.gfx.set_visible(&self.editor.frame_label, false);
                systems
                    .gfx
                    .set_visible(&self.editor.selection.blocker, false);
            }
            PresetWindowType::Editor => {
                systems.gfx.set_visible(&self.editor.bg, self.visible);
                self.editor.scrollbar.set_visible(systems, self.visible);
                self.editor.frame_scroll.set_visible(systems, self.visible);
                systems.gfx.set_visible(&self.editor.tileset, self.visible);
                self.editor.tile_list.set_visible(systems, self.visible);
                self.editor.type_list.set_visible(systems, self.visible);
                self.editor.save_button.set_visible(systems, self.visible);
                self.editor.cancel_button.set_visible(systems, self.visible);
                systems
                    .gfx
                    .set_visible(&self.editor.selection.blocker, false);
                for gfx in self.editor.selection.gfx.iter() {
                    systems.gfx.set_visible(gfx, self.visible);
                }
                systems
                    .gfx
                    .set_visible(&self.editor.frame_label, self.visible);

                for gfx in self.base.preview.iter() {
                    systems.gfx.set_visible(gfx, false);
                }
                self.base.edit_button.set_visible(systems, false);
                systems.gfx.set_visible(&self.base.seperator, false);
                systems.gfx.set_visible(&self.base.preview_name, false);
                systems.gfx.set_visible(&self.base.preview_info, false);
                systems.gfx.set_visible(&self.base.preview_bg, false);
                self.base.preset_list.set_visible(systems, false, false);
            }
        }
    }

    pub fn update_editor_content(&mut self, systems: &mut SystemHolder) {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.editor.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.editor.start_pos.y
                + ((self.editor.area_size.y - (34.0 * systems.scale as f32).floor())
                    - tileset_size.y)
                + self.editor.scrollbar.value as f32,
            ORDER_WINDOW_CONTENT,
        );

        systems.gfx.set_pos(&self.editor.bg, tileset_pos);
        systems.gfx.set_pos(&self.editor.tileset, tileset_pos);
        self.editor.tile_list.move_window(
            systems,
            Vec2::new(tileset_pos.x, tileset_pos.y + tileset_size.y),
            ORDER_WINDOW_CONTENT,
        );
        self.editor.type_list.move_window(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (58.0 * systems.scale as f32).floor(),
            ),
            ORDER_WINDOW_CONTENT,
        );
        self.editor.frame_scroll.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x + (50.0 * systems.scale as f32).floor(),
                tileset_pos.y - (35.0 * systems.scale as f32).floor(),
            ),
        );
        self.editor.save_button.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (85.0 * systems.scale as f32).floor(),
            ),
        );
        self.editor.cancel_button.set_pos(
            systems,
            Vec2::new(
                tileset_pos.x,
                tileset_pos.y - (110.0 * systems.scale as f32).floor(),
            ),
        );

        let frame_text_pos = Vec3::new(
            tileset_pos.x,
            tileset_pos.y - (25.0 * systems.scale as f32).floor(),
            ORDER_WINDOW_CONTENT,
        );
        let frame_text_size = (Vec2::new(50.0, 20.0) * systems.scale as f32).floor();
        systems
            .gfx
            .set_pos(&self.editor.frame_label, frame_text_pos);
        systems.gfx.set_bound(
            &self.editor.frame_label,
            Bounds::new(
                frame_text_pos.x,
                frame_text_pos.y,
                frame_text_pos.x + frame_text_size.x,
                frame_text_pos.y + frame_text_size.y,
            ),
        );

        let end_pos = if matches!(
            self.editor.cur_type,
            PresetTypeList::AutotileAnimated | PresetTypeList::AutoTile
        ) {
            self.editor.selection.start_pos + Vec2::new(4.0, 2.0)
        } else {
            self.editor.selection.start_pos
        };
        self.select_tile(systems, self.editor.selection.start_pos, end_pos);
    }

    pub fn change_tileset(&mut self, systems: &mut SystemHolder, tileset: usize) {
        if self.editor.cur_tileset == tileset {
            return;
        }
        self.editor.cur_tileset = tileset;

        systems
            .gfx
            .remove_gfx(&mut systems.renderer, &self.editor.tileset);

        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.editor.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.editor.start_pos.y
                + ((self.editor.area_size.y - (34.0 * systems.scale as f32).floor())
                    - tileset_size.y)
                + self.editor.scrollbar.value as f32,
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
        self.editor.tileset = systems.gfx.add_image(
            img,
            RENDER_GUI2,
            "Tileset",
            self.visible,
            CameraView::SubView1,
        );
    }

    pub fn select_tile(&mut self, systems: &mut SystemHolder, start_pos: Vec2, end_pos: Vec2) {
        let tileset_size = Vec2::new(
            ((TILESET_COUNT_X * 20) as f32 * systems.scale as f32).floor(),
            ((TILESET_COUNT_Y * 20) as f32 * systems.scale as f32).floor(),
        );
        let tileset_pos = Vec3::new(
            self.editor.start_pos.x + (5.0 * systems.scale as f32).floor(),
            self.editor.start_pos.y
                + ((self.editor.area_size.y - (34.0 * systems.scale as f32).floor())
                    - tileset_size.y)
                + self.editor.scrollbar.value as f32,
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

        let is_visible = systems.gfx.get_visible(&self.editor.selection.gfx[0]);

        if matches!(
            self.editor.cur_type,
            PresetTypeList::AutoTile | PresetTypeList::AutotileAnimated
        ) {
            systems
                .gfx
                .set_visible(&self.editor.selection.blocker, is_visible);
            systems.gfx.set_pos(
                &self.editor.selection.blocker,
                Vec3::new(
                    base_pos.x + ((3.0 * tile_size) * systems.scale as f32).floor(),
                    base_pos.y,
                    tileset_pos.z,
                ),
            );
        } else {
            systems
                .gfx
                .set_visible(&self.editor.selection.blocker, false);
        }

        for (i, gfx) in self.editor.selection.gfx.iter().enumerate() {
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

    pub fn update_frames(&mut self, systems: &mut SystemHolder) {
        if !self.visible || self.window_type != PresetWindowType::Base {
            return;
        }

        if !matches!(
            self.base.preset_type,
            PresetTypeList::Animated | PresetTypeList::AutotileAnimated
        ) {
            return;
        }

        let old_frame = self.base.cur_frame;
        self.base.cur_frame += 1;
        if self.base.cur_frame > 3 {
            self.base.cur_frame = 0;
        }

        systems
            .gfx
            .set_visible(&self.base.preview[old_frame], false);
        systems
            .gfx
            .set_visible(&self.base.preview[self.base.cur_frame], true);
    }
}
