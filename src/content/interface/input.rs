use camera::controls::FlatControls;
use input::Key;
use winit::{event::KeyEvent, event_loop::ActiveEventLoop};

use crate::{
    content::{
        Content,
        interface::widget::measure_string,
        widget::{Alert, AlertBuilder, Tooltip},
    },
    data_types::{MouseInputType, Result, SelectedTextbox},
    renderer::{Graphics, SystemHolder},
};
use graphics::Vec2;

mod drawing_tool;
mod menu_bar;
//mod sample_window;
mod map_pos_input;
mod side_window;

use drawing_tool::*;
use menu_bar::*;
//use sample_window::*;
pub use map_pos_input::*;
pub use side_window::*;

#[allow(clippy::too_many_arguments)]
pub fn interface_input(
    systems: &mut SystemHolder,
    graphics: &mut Graphics<FlatControls>,
    inputtype: MouseInputType,
    mouse_pos: Vec2,
    content: &mut Content,
    tooltip: &mut Tooltip,
    alert: &mut Alert,
    elwt: &ActiveEventLoop,
    seconds: f32,
) -> Result<bool> {
    tooltip.check_tooltip(systems, mouse_pos);

    if alert.visible {
        alert.alert_mouse_input(
            systems, content, elwt, inputtype, tooltip, mouse_pos, seconds,
        )?;
        return Ok(true);
    }

    if content.interface.mappos_input.visible {
        mappos_mouse_input(
            systems, content, alert, inputtype, tooltip, mouse_pos, seconds,
        )?;
        return Ok(true);
    }

    match inputtype {
        MouseInputType::LeftDown => {
            if menu_bar_click_widget(content, systems, alert, mouse_pos, seconds)?
                || tool_click_widget(content, systems, mouse_pos)
                || side_click_widget(content, systems, alert, mouse_pos)?
            //|| sample_click_widget(content, systems, alert, mouse_pos)
            {
                return Ok(true);
            }

            for textbox in 0..=SelectedTextbox::MapPosTextbox as usize {
                if content.interface.click_textbox(
                    systems,
                    mouse_pos,
                    SelectedTextbox::from_index(textbox),
                ) {
                    return Ok(true);
                }
            }
        }
        MouseInputType::LeftDownMove => {
            //content
            //    .interface
            //    .sample
            //    .hold_move_scrollbar(systems, mouse_pos);

            if side_clickdrag_widget(content, systems, mouse_pos) {
                return Ok(true);
            }

            {
                let gui = &mut content.interface;
                gui.side_window.hold_move_scrollbar(systems, mouse_pos);
                gui.menu_bar.hold_move_scrollbar(systems, mouse_pos);
                gui.hold_move_textbox(systems, mouse_pos);
            }
            drawingtool_hold_move_scrollbar(content, systems, graphics, mouse_pos);
        }
        MouseInputType::Move => {
            let gui = &mut content.interface;
            //gui.sample.hover_widgets(systems, mouse_pos);
            gui.side_window.hover_widgets(systems, mouse_pos, tooltip);
            gui.menu_bar.hover_widgets(systems, mouse_pos);
            gui.tool.hover_widgets(systems, mouse_pos, tooltip);
        }
        MouseInputType::Release => {
            let gui = &mut content.interface;
            //gui.sample.reset_widgets(systems, mouse_pos);
            gui.side_window.reset_widgets(systems, mouse_pos);
            gui.menu_bar.reset_widgets(systems, mouse_pos);
            gui.tool.reset_widgets(systems, mouse_pos);
            gui.reset_textbox();
        }
        MouseInputType::DoubleLeftDown => {}
        MouseInputType::DoubleRightDown => {}
        MouseInputType::RightDownMove => {}
        MouseInputType::RightDown => {}
        MouseInputType::MiddleDown => {}
        MouseInputType::MiddleDownMove => {}
    }
    Ok(false)
}
