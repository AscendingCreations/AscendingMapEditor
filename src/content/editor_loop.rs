use graphics::MapRenderer;

use crate::{
    content::Content,
    data_types::Result,
    database::{delete_recovery_map_file, is_recovery_map_file_exist, save_temp_file},
    renderer::SystemHolder,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LoopTimer {
    interface_tmr: f32,
    notification_tmr: f32,
    file_tmr: f32,
    tileset_frame: f32,
    attr_preview_tmr: f32,
}

pub fn editor_loop(
    systems: &mut SystemHolder,
    _map_renderer: &mut MapRenderer,
    content: &mut Content,
    seconds: f32,
    loop_timer: &mut LoopTimer,
) -> Result<()> {
    if seconds > loop_timer.interface_tmr {
        content.map_view.update_tile_frame(systems);
        loop_timer.interface_tmr = seconds + 0.4;
    }

    if seconds > loop_timer.attr_preview_tmr {
        content
            .map_view
            .attr_preview
            .update_visible(systems, seconds);
        loop_timer.attr_preview_tmr = seconds + 0.4;
    }

    if seconds > loop_timer.notification_tmr {
        content.interface.notification.update(systems, seconds);
        loop_timer.notification_tmr = seconds + 0.01;
    }

    if seconds > loop_timer.tileset_frame {
        content.interface.side_window.presets.update_frames(systems);
        loop_timer.tileset_frame = seconds + 0.2;
    }

    if seconds > loop_timer.file_tmr {
        if !content.data.temp_saved {
            if let Some(mappos) = content.data.pos {
                save_temp_file(
                    mappos.x,
                    mappos.y,
                    mappos.group.try_into().unwrap(),
                    &content.data.mapdata,
                    true,
                )?;

                content.interface.notification.add_msg(
                    systems,
                    format!(
                        "Temp Map [X: {} Y: {} Group: {}] Saved!",
                        mappos.x, mappos.y, mappos.group
                    ),
                    seconds,
                );

                let _ = content.data.unsaved_map.insert(mappos);
            } else {
                save_temp_file(0, 0, 0, &content.data.mapdata, false)?;

                content.interface.notification.add_msg(
                    systems,
                    "Temp recovery map file saved!".to_string(),
                    seconds,
                );
            }

            content.data.temp_saved = true;
        }

        loop_timer.file_tmr = seconds + 60.0;
    }

    Ok(())
}
