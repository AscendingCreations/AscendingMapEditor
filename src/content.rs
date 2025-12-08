use crate::{
    audio::AudioCollection,
    content::widget::{Alert, AlertBuilder, AlertIndex},
    data_types::Result,
    database::{MapPosition, Presets, save_and_clear_map, save_temp_file},
    renderer::SystemHolder,
};
use graphics::MapRenderer;

mod data;
mod editor_loop;
mod input;
pub mod interface;
mod map_view;

pub use data::*;
pub use editor_loop::*;
pub use input::*;
pub use interface::*;
pub use map_view::*;
use winit::event_loop::ActiveEventLoop;

pub struct Content {
    pub interface: Interface,
    pub map_view: MapView,
    pub audio_collection: AudioCollection,
    pub data: EditorData,
    pub preset: Presets,
}

impl Content {
    pub fn new(systems: &mut SystemHolder, map_renderer: &mut MapRenderer) -> Result<Self> {
        let audio_collection = AudioCollection::new();

        Ok(Content {
            interface: Interface::new(&audio_collection, systems),
            map_view: MapView::new(systems, map_renderer)?,
            audio_collection,
            data: EditorData::new(),
            preset: Presets::load_data()?,
        })
    }

    pub fn screen_resize(&mut self, systems: &mut SystemHolder) {
        self.interface.screen_resize(systems);
    }
}

pub fn exit_editor(
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    elwt: &ActiveEventLoop,
    repeat_command: Option<bool>,
) -> Result<()> {
    if content.data.changed && !content.data.temp_saved {
        if let Some(mappos) = content.data.pos {
            save_temp_file(
                mappos.x,
                mappos.y,
                mappos.group.try_into().unwrap(),
                &content.data.mapdata,
                true,
            )?;

            let _ = content.data.unsaved_map.insert(mappos);
        } else {
            save_temp_file(0, 0, 0, &content.data.mapdata, false)?;
        }

        content.data.temp_saved = true;
    }

    content.data.exiting_save = true;

    if !content.data.unsaved_map.is_empty() {
        if let Some(save_map) = repeat_command {
            if save_map {
                for data in content.data.unsaved_map.iter() {
                    save_and_clear_map(data.x, data.y, data.group as u64)?;
                }
            }

            elwt.exit();
        } else if let Some(data) = content.data.unsaved_map.pop() {
            let mut alert_builder = AlertBuilder::new_confirm(
                "Unsaved Map",
                &format!(
                    "Do you want to save the change on map {}_{}_{}?",
                    data.x, data.y, data.group
                ),
            );
            alert_builder
                .with_index(AlertIndex::ExitSaveMap(data))
                .with_width(600);
            if !content.data.unsaved_map.is_empty() {
                alert_builder.with_checkbox("repeat on the other maps?");
            }

            alert.show_alert(systems, &alert_builder)
        }
    } else {
        elwt.exit();
    }

    Ok(())
}
