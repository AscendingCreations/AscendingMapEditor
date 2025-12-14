use graphics::*;

use crate::{
    audio::AudioCollection,
    content::{
        interface::widget::{
            Button, ButtonChangeType, ButtonContentText, ButtonContentType, ButtonRect, ButtonType,
            CheckRect, CheckType, Checkbox, CheckboxChangeType, CheckboxRect, CheckboxText,
            CheckboxType, Scrollbar, ScrollbarBackground, ScrollbarRect, SelectionColor, TextList,
            TextListBG, TextListBGRect, TextListData, Textbox,
        },
        widget::{ButtonContentImg, OptionList, OptionListColor},
    },
    data_types::*,
    gfx_collection::GfxType,
    renderer::SystemHolder,
    resource::GuiTexture,
};

pub mod attributes;
pub mod dirblocks;
pub mod music;
pub mod presets;
//pub mod properties;
pub mod tilesets;
pub mod weather;
pub mod zones;

pub use attributes::*;
pub use dirblocks::*;
pub use music::*;
pub use presets::*;
//pub use properties::*;
pub use tilesets::*;
pub use weather::*;
pub use zones::*;

pub struct SideWindow {
    pub bg: GfxType,
    pub content_bg: GfxType,
    pub tab_button: Vec<Button>,

    pub attributes: AttributeWindow,
    pub tilesets: TilesetWindow,
    pub presets: PresetWindow,
    pub dirblocks: DirBlockWindow,
    pub music: MusicWindow,
    //pub properties: PropertiesWindow,
    pub weather: WeatherWindow,
    pub zone: ZoneWindow,

    pub cur_tab: TabButton,
}

impl SideWindow {
    pub fn new(audio_collection: &AudioCollection, systems: &mut SystemHolder) -> Self {
        let bg_pos = (Vec2::new(5.0, 5.0) * systems.scale as f32).floor();
        let bg_size = Vec2::new(
            (220.0 * systems.scale as f32).floor(),
            systems.size.height - (25.0 * systems.scale as f32).floor(),
        );

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_WINDOW),
            Vec2::new(
                bg_size.x + (34.0 * systems.scale as f32).floor(),
                systems.size.height,
            ),
            Color::rgba(130, 130, 130, 255),
            0,
        );
        let bg = systems.gfx.add_rect(
            rect,
            RENDER_GUI,
            "Side Window BG",
            true,
            CameraView::SubView1,
        );

        let rect = Rect::new(
            &mut systems.renderer,
            Vec3::new(bg_pos.x, bg_pos.y, ORDER_WINDOW),
            bg_size,
            Color::rgb(100, 100, 100),
            1,
        );
        let content_bg = systems.gfx.add_rect(
            rect,
            RENDER_GUI,
            "Side Window BG",
            true,
            CameraView::SubView1,
        );

        let button_rect = ButtonRect {
            rect_color: Color::rgb(60, 60, 60),
            got_border: false,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(80, 80, 80)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(90, 90, 90)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::ColorChange(Color::rgb(100, 100, 100)),
        };

        let tabcount = TabButton::Count as usize;
        let tab_index_count = tabcount.saturating_sub(1);
        let tab_pos = Vec2::new(
            (225.0 * systems.scale as f32).floor(),
            systems.size.height
                - ((20.0 + (24.0 * tabcount as f32) + (tab_index_count * 2) as f32)
                    * systems.scale as f32)
                    .floor(),
        );

        let mut tab_button = Vec::with_capacity(tabcount);
        for tab in 0..TabButton::Count as usize {
            tab_button.push(Button::new(
                systems,
                ButtonType::Rect(button_rect),
                ButtonContentType::Image(ButtonContentImg {
                    res: systems.resource.interface[GuiTexture::TabIcon as usize],
                    pos: Vec2::new(2.0, 2.0),
                    uv: Vec2::new(20.0 * tab as f32, 0.0),
                    size: Vec2::new(20.0, 20.0),
                    order_layer: 1,
                    buffer_layer: RENDER_GUI2,
                    hover_change: ButtonChangeType::None,
                    click_change: ButtonChangeType::None,
                    alert_change: ButtonChangeType::None,
                    disable_change: ButtonChangeType::None,
                }),
                tab_pos,
                Vec2::new(0.0, 26.0 * (tab_index_count - tab) as f32),
                ORDER_WINDOW,
                Vec2::new(24.0, 24.0),
                2,
                RENDER_GUI,
                true,
                None,
                false,
            ));
        }
        tab_button[0].set_disable(systems, true);

        Self {
            bg,
            content_bg,
            tab_button,

            attributes: AttributeWindow::new(systems, bg_pos, bg_size),
            tilesets: TilesetWindow::new(systems, bg_pos, bg_size),
            presets: PresetWindow::new(systems, bg_pos, bg_size),
            dirblocks: DirBlockWindow::new(systems, bg_pos, bg_size),
            music: MusicWindow::new(audio_collection, systems, bg_pos, bg_size),
            //properties: PropertiesWindow::new(systems, bg_pos, bg_size),
            weather: WeatherWindow::new(systems, bg_pos, bg_size),
            zone: ZoneWindow::new(systems, bg_pos, bg_size),

            cur_tab: TabButton::Tileset,
        }
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        let bg_pos = (Vec2::new(5.0, 5.0) * systems.scale as f32).floor();
        let bg_size = Vec2::new(
            (220.0 * systems.scale as f32).floor(),
            systems.size.height - (25.0 * systems.scale as f32).floor(),
        );

        systems.gfx.set_size(&self.content_bg, bg_size);
        systems.gfx.set_size(
            &self.bg,
            Vec2::new(
                bg_size.x + (34.0 * systems.scale as f32).floor(),
                systems.size.height,
            ),
        );

        let tabcount = TabButton::Count as usize;
        let tab_index_count = tabcount.saturating_sub(1);
        let tab_pos = Vec2::new(
            (225.0 * systems.scale as f32).floor(),
            systems.size.height
                - ((20.0 + (24.0 * tabcount as f32) + (tab_index_count * 2) as f32)
                    * systems.scale as f32)
                    .floor(),
        );
        for button in self.tab_button.iter_mut() {
            button.set_pos(systems, tab_pos);
        }

        self.attributes.screen_resize(systems, bg_pos, bg_size);
        self.tilesets.screen_resize(systems, bg_pos, bg_size);
        self.presets.screen_resize(systems, bg_pos, bg_size);
        self.dirblocks.screen_resize(systems, bg_pos, bg_size);
        self.music.screen_resize(systems, bg_pos, bg_size);
        //self.properties.screen_resize(systems, bg_pos, bg_size);
        self.weather.screen_resize(systems, bg_pos, bg_size);
        self.zone.screen_resize(systems, bg_pos, bg_size);
    }
}
