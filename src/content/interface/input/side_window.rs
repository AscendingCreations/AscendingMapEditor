use graphics::Vec2;

use crate::{
    content::{
        Content,
        interface::side_window::SideWindow,
        widget::{Alert, Tooltip},
    },
    data_types::{Result, TabButton},
    renderer::SystemHolder,
};

mod attributes;
mod dirblocks;
mod music;
mod presets;
//mod properties;
mod tilesets;
mod weather;
mod zones;

pub use attributes::*;
use dirblocks::*;
use music::*;
pub use presets::*;
//use properties::*;
use tilesets::*;
use weather::*;
use zones::*;

impl SideWindow {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        tooltip: &mut Tooltip,
    ) {
        for button in self.tab_button.iter_mut() {
            let in_hover = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_hover);

            if in_hover && let Some(msg) = &button.tooltip {
                tooltip.init_tooltip(systems, mouse_pos, msg.clone(), false);
            }
        }

        self.attributes.hover_widgets(systems, mouse_pos, tooltip);
        self.tilesets.hover_widgets(systems, mouse_pos, tooltip);
        self.presets.hover_widgets(systems, mouse_pos, tooltip);
        self.dirblocks.hover_widgets(systems, mouse_pos, tooltip);
        self.music.hover_widgets(systems, mouse_pos, tooltip);
        //self.properties.hover_widgets(systems, mouse_pos, tooltip);
        self.weather.hover_widgets(systems, mouse_pos, tooltip);
        self.zone.hover_widgets(systems, mouse_pos, tooltip);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        for button in self.tab_button.iter_mut() {
            button.set_click(systems, false);
        }

        self.attributes.reset_widgets(systems, mouse_pos);
        self.tilesets.reset_widgets(systems, mouse_pos);
        self.presets.reset_widgets(systems, mouse_pos);
        self.dirblocks.reset_widgets(systems, mouse_pos);
        self.music.reset_widgets(systems, mouse_pos);
        //self.properties.reset_widgets(systems, mouse_pos);
        self.weather.reset_widgets(systems, mouse_pos);
        self.zone.reset_widgets(systems, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.attributes.hold_scrollbar(systems, mouse_pos)
            || self.tilesets.hold_scrollbar(systems, mouse_pos)
            || self.presets.hold_scrollbar(systems, mouse_pos)
            || self.dirblocks.hold_scrollbar(systems, mouse_pos)
            || self.music.hold_scrollbar(systems, mouse_pos)
            //|| self.properties.hold_scrollbar(systems, mouse_pos)
            || self.weather.hold_scrollbar(systems, mouse_pos)
            || self.zone.hold_scrollbar(systems, mouse_pos)
        {
            return true;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.attributes.hold_move_scrollbar(systems, mouse_pos);
        self.tilesets.hold_move_scrollbar(systems, mouse_pos);
        self.presets.hold_move_scrollbar(systems, mouse_pos);
        self.dirblocks.hold_move_scrollbar(systems, mouse_pos);
        self.music.hold_move_scrollbar(systems, mouse_pos);
        //self.properties.hold_move_scrollbar(systems, mouse_pos);
        self.weather.hold_move_scrollbar(systems, mouse_pos);
        self.zone.hold_move_scrollbar(systems, mouse_pos);
    }

    pub fn click_tab_button(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        for (index, button) in self.tab_button.iter_mut().enumerate() {
            if button.in_area(systems, mouse_pos) && !button.disabled {
                button.set_click(systems, true);
                return Some(index);
            }
        }

        None
    }
}

pub fn side_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    mouse_pos: Vec2,
) -> Result<bool> {
    {
        let gui = &mut content.interface.side_window;

        if gui.hold_scrollbar(systems, mouse_pos) {
            return Ok(true);
        }

        if let Some(index) = gui.click_tab_button(systems, mouse_pos) {
            let tool = TabButton::from_index(index);
            switch_tab(content, systems, tool);
            return Ok(true);
        }
    }

    if side_attribute_click_widget(content, systems, mouse_pos)
        || side_tileset_click_widget(content, systems, mouse_pos)
        || side_preset_click_widget(content, systems, alert,mouse_pos)?
        || side_dirblock_click_widget(content, systems, mouse_pos)
        || side_music_click_widget(content, systems, mouse_pos)?
        //|| side_properties_click_widget(content, systems, mouse_pos)
        || side_weather_click_widget(content, systems, mouse_pos)
        || side_zone_click_widget(content, systems, mouse_pos)
    {
        return Ok(true);
    }

    Ok(false)
}

pub fn side_clickdrag_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    if side_tileset_clickdrag_widget(content, systems, mouse_pos)
        || side_preset_clickdrag_widget(content, systems, mouse_pos)
    {
        return true;
    }

    false
}

pub fn switch_tab(content: &mut Content, systems: &mut SystemHolder, tool: TabButton) {
    let gui = &mut content.interface.side_window;

    if tool != gui.cur_tab {
        gui.tab_button[gui.cur_tab as usize].set_disable(systems, false);
        gui.cur_tab = tool;
        gui.tab_button[gui.cur_tab as usize].set_disable(systems, true);
    }

    gui.attributes
        .set_visible(systems, tool == TabButton::Attributes);
    gui.tilesets
        .set_visible(systems, tool == TabButton::Tileset);
    gui.presets
        .set_visible(systems, tool == TabButton::CustomTiles);
    gui.dirblocks
        .set_visible(systems, tool == TabButton::DirBlock);
    gui.music.set_visible(systems, tool == TabButton::Music);
    //gui.properties
    //    .set_visible(systems, tool == TabButton::Properties);
    gui.weather.set_visible(systems, tool == TabButton::Weather);
    gui.zone.set_visible(systems, tool == TabButton::Zones);

    match tool {
        TabButton::Zones => {
            let zone_data = content.data.mapdata.zones[gui.zone.cur_zone];

            gui.zone.textbox[0].set_text(systems, format!("{}", zone_data.0));

            for i in 0..5 {
                gui.zone.textbox[i + 1].set_text(
                    systems,
                    if let Some(data) = zone_data.1[i] {
                        format!("{data}")
                    } else {
                        String::new()
                    },
                );
            }
        }
        TabButton::CustomTiles => {
            preset_update_list(content, systems);
        }
        _ => {}
    }

    content
        .map_view
        .set_attr_visible(systems, tool == TabButton::Attributes);
    content
        .map_view
        .set_zone_visible(systems, tool == TabButton::Zones);
    content
        .map_view
        .set_dirblock_visible(systems, tool == TabButton::DirBlock);
}
