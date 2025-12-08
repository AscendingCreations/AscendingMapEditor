use cosmic_text::{Attrs, Metrics};
use graphics::*;
use indexmap::IndexMap;

use crate::{
    ConfigData, SystemHolder, audio::AudioCollection, config, data_types::*,
    database::EditorMapAttribute, gfx_collection::GfxType, resource::GuiTexture,
};

mod drawing_tool;
mod input;
mod menu_bar;
//mod sample_window;
mod footer;
mod map_pos_input;
mod notification;
mod side_window;
pub mod widget;

use drawing_tool::*;
pub use input::*;
use menu_bar::*;
//use sample_window::*;
use footer::*;
use map_pos_input::*;
use notification::*;
use side_window::*;
use widget::*;

pub struct Interface {
    //pub sample: SampleWindow,
    pub side_window: SideWindow,
    pub menu_bar: MenuBar,
    pub tool: DrawingTool,
    pub mappos_input: MapPosInput,
    pub footer: Footer,
    pub notification: Notification,

    pub selected_textbox: SelectedTextbox,
}

impl Interface {
    pub fn new(audio_collection: &AudioCollection, systems: &mut SystemHolder) -> Self {
        Self {
            //sample: SampleWindow::new(systems),
            side_window: SideWindow::new(audio_collection, systems),
            menu_bar: MenuBar::new(systems),
            tool: DrawingTool::new(systems),
            mappos_input: MapPosInput::new(systems),
            selected_textbox: SelectedTextbox::None,
            footer: Footer::new(systems),
            notification: Notification::default(),
        }
    }

    pub fn reset_textbox(&mut self) {
        match self.selected_textbox {
            SelectedTextbox::SampleTextbox => {} //self.sample.sample_textbox.set_hold(false),
            SelectedTextbox::AttrContent => match self.side_window.attributes.cur_attribute {
                EditorMapAttribute::Warp => {
                    if let Some(index) = self.side_window.attributes.attr_position.cur_textbox {
                        self.side_window.attributes.attr_position.input_box[index]
                            .textbox
                            .set_hold(false);
                    }
                }
                EditorMapAttribute::ItemSpawn => {
                    if let Some(index) = self.side_window.attributes.attr_itemspawn.cur_textbox {
                        self.side_window.attributes.attr_itemspawn.input_box[index]
                            .textbox
                            .set_hold(false);
                    }
                }
                EditorMapAttribute::Shop => {
                    self.side_window
                        .attributes
                        .attr_index
                        .input_box
                        .textbox
                        .set_hold(false);
                }
                EditorMapAttribute::Sign => {
                    self.side_window
                        .attributes
                        .attr_sign
                        .input_box
                        .textbox
                        .set_hold(false);
                }
                _ => {}
            },
            SelectedTextbox::ZoneTextbox => {
                if let Some(index) = self.side_window.zone.cur_textbox {
                    self.side_window.zone.textbox[index].set_hold(false);
                }
            }
            SelectedTextbox::MapPosTextbox => {
                if let Some(index) = self.mappos_input.cur_textbox {
                    self.mappos_input.textbox[index].set_hold(false)
                }
            }
            SelectedTextbox::None => {}
        }
    }

    pub fn hold_move_textbox(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        match self.selected_textbox {
            SelectedTextbox::SampleTextbox => {
                //self.sample.sample_textbox.hold_move(systems, screen_pos)
            }
            SelectedTextbox::AttrContent => match self.side_window.attributes.cur_attribute {
                EditorMapAttribute::Warp => {
                    if let Some(index) = self.side_window.attributes.attr_position.cur_textbox {
                        self.side_window.attributes.attr_position.input_box[index]
                            .textbox
                            .hold_move(systems, screen_pos);
                    }
                }
                EditorMapAttribute::ItemSpawn => {
                    if let Some(index) = self.side_window.attributes.attr_itemspawn.cur_textbox {
                        self.side_window.attributes.attr_itemspawn.input_box[index]
                            .textbox
                            .hold_move(systems, screen_pos);
                    }
                }
                EditorMapAttribute::Shop => {
                    self.side_window
                        .attributes
                        .attr_index
                        .input_box
                        .textbox
                        .hold_move(systems, screen_pos);
                }
                EditorMapAttribute::Sign => {
                    self.side_window
                        .attributes
                        .attr_sign
                        .input_box
                        .textbox
                        .hold_move(systems, screen_pos);
                }
                _ => {}
            },
            SelectedTextbox::ZoneTextbox => {
                if let Some(index) = self.side_window.zone.cur_textbox {
                    self.side_window.zone.textbox[index].hold_move(systems, screen_pos);
                }
            }
            SelectedTextbox::MapPosTextbox => {
                if let Some(index) = self.mappos_input.cur_textbox {
                    self.mappos_input.textbox[index].hold_move(systems, screen_pos);
                }
            }
            SelectedTextbox::None => {}
        }
    }

    pub fn unclick_textbox(&mut self, systems: &mut SystemHolder) {
        match self.selected_textbox {
            SelectedTextbox::SampleTextbox => {} //self.sample.sample_textbox.set_select(systems, false),
            SelectedTextbox::AttrContent => match self.side_window.attributes.cur_attribute {
                EditorMapAttribute::Warp => {
                    if let Some(index) = self.side_window.attributes.attr_position.cur_textbox {
                        self.side_window.attributes.attr_position.input_box[index]
                            .textbox
                            .set_select(systems, false);
                    }
                }
                EditorMapAttribute::ItemSpawn => {
                    if let Some(index) = self.side_window.attributes.attr_itemspawn.cur_textbox {
                        self.side_window.attributes.attr_itemspawn.input_box[index]
                            .textbox
                            .set_select(systems, false);
                    }
                }
                EditorMapAttribute::Shop => {
                    self.side_window
                        .attributes
                        .attr_index
                        .input_box
                        .textbox
                        .set_select(systems, false);
                }
                EditorMapAttribute::Sign => {
                    self.side_window
                        .attributes
                        .attr_sign
                        .input_box
                        .textbox
                        .set_select(systems, false);
                }
                _ => {}
            },
            SelectedTextbox::ZoneTextbox => {
                if let Some(index) = self.side_window.zone.cur_textbox {
                    self.side_window.zone.textbox[index].set_select(systems, false);
                }
            }
            SelectedTextbox::MapPosTextbox => {
                if let Some(index) = self.mappos_input.cur_textbox {
                    self.mappos_input.textbox[index].set_select(systems, false);
                }
            }
            SelectedTextbox::None => {}
        }
        self.selected_textbox = SelectedTextbox::None;
    }

    pub fn click_textbox(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
        textbox_type: SelectedTextbox,
    ) -> bool {
        let did_click = match textbox_type {
            SelectedTextbox::SampleTextbox => false, //self.sample.click_textbox(systems, screen_pos),
            SelectedTextbox::AttrContent => match self.side_window.attributes.cur_attribute {
                EditorMapAttribute::Warp => self
                    .side_window
                    .attributes
                    .attr_position
                    .click_textbox(systems, screen_pos),
                EditorMapAttribute::ItemSpawn => self
                    .side_window
                    .attributes
                    .attr_itemspawn
                    .click_textbox(systems, screen_pos),
                EditorMapAttribute::Shop => self
                    .side_window
                    .attributes
                    .attr_index
                    .click_textbox(systems, screen_pos),
                EditorMapAttribute::Sign => self
                    .side_window
                    .attributes
                    .attr_sign
                    .click_textbox(systems, screen_pos),
                _ => false,
            },
            SelectedTextbox::ZoneTextbox => {
                self.side_window.zone.click_textbox(systems, screen_pos)
            }
            SelectedTextbox::MapPosTextbox => {
                if self.mappos_input.visible {
                    self.mappos_input.click_textbox(systems, screen_pos)
                } else {
                    false
                }
            }
            SelectedTextbox::None => false,
        };

        if did_click {
            self.selected_textbox = textbox_type;
        } else {
            self.unclick_textbox(systems);
        }

        did_click
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        //self.sample.screen_resize(systems);
        self.side_window.screen_resize(systems);
        self.menu_bar.screen_resize(systems);
        self.tool.screen_resize(systems);
        self.mappos_input.screen_resize(systems);
        self.footer.screen_resize(systems);
        self.notification.screen_resize(systems);
    }
}
