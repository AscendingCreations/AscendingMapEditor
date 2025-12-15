use camera::controls::FlatControls;
use graphics::Vec2;

use crate::{
    content::{Content, interface::drawing_tool::DrawingTool, widget::Tooltip},
    data_types::ToolType,
    renderer::{Graphics, SystemHolder},
};

impl DrawingTool {
    pub fn hover_widgets(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
        tooltip: &mut Tooltip,
    ) {
        for button in self.tool_button.iter_mut() {
            let in_hover = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_hover);

            if in_hover && let Some(msg) = &button.tooltip {
                tooltip.init_tooltip(systems, mouse_pos, msg.clone(), false);
            }
        }

        let mut got_hover = false;
        for button in self.layer_button.iter_mut() {
            let in_hover = button.in_area(systems, mouse_pos);
            button.set_hover(systems, in_hover && !got_hover);
            if in_hover {
                got_hover = true;
            }

            if in_hover && let Some(msg) = &button.tooltip {
                tooltip.init_tooltip(systems, mouse_pos, msg.clone(), false);
            }
        }

        let scrollbar_hover = self.zoom_scroll.in_scroll(mouse_pos);
        self.zoom_scroll.set_hover(systems, scrollbar_hover);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        for button in self.tool_button.iter_mut() {
            button.set_click(systems, false);
        }
        for button in self.layer_button.iter_mut() {
            button.set_click(systems, false);
        }
        self.zoom_scroll.set_hold(systems, false, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.zoom_scroll.in_scroll(mouse_pos) {
            self.zoom_scroll.set_hold(systems, true, mouse_pos);
            return true;
        }

        false
    }

    pub fn click_tool_button(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        for (index, button) in self.tool_button.iter_mut().enumerate() {
            if button.in_area(systems, mouse_pos) && !button.disabled {
                button.set_click(systems, true);
                return Some(index);
            }
        }

        None
    }

    pub fn click_layer_button(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        for (index, button) in self.layer_button.iter_mut().enumerate() {
            if button.in_area(systems, mouse_pos) && !button.disabled {
                button.set_click(systems, true);
                return Some(index);
            }
        }

        None
    }
}

pub fn drawingtool_hold_move_scrollbar(
    content: &mut Content,
    systems: &mut SystemHolder,
    graphics: &mut Graphics<FlatControls>,
    mouse_pos: Vec2,
) {
    content
        .interface
        .tool
        .zoom_scroll
        .set_move_scroll(systems, mouse_pos);

    let zoom_level = 100 + (10 * content.interface.tool.zoom_scroll.value);
    systems.gfx.set_text(
        &mut systems.renderer,
        &content.interface.tool.zoom_label,
        &format!("{zoom_level}%"),
    );

    let set_zoom = zoom_level as f32 * 0.01;
    let rounded_value = (set_zoom * 10.0).round() / 10.0;
    content
        .map_view
        .adjust_map_by_zoom(systems, graphics, rounded_value);
    graphics.system.controls_mut().settings_mut().zoom = rounded_value;
    systems.config.zoom = rounded_value;
}

pub fn tool_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    mouse_pos: Vec2,
) -> bool {
    let gui = &mut content.interface.tool;

    if gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if let Some(index) = gui.click_tool_button(systems, mouse_pos) {
        let tool = ToolType::from_index(index);
        if tool != gui.cur_tool {
            gui.tool_button[gui.cur_tool as usize].set_disable(systems, false);
            gui.cur_tool = tool;
            gui.tool_button[gui.cur_tool as usize].set_disable(systems, true);
        }
        return true;
    }

    if let Some(index) = gui.click_layer_button(systems, mouse_pos) {
        if gui.cur_layer != index {
            gui.layer_button[gui.cur_layer].set_disable(systems, false);
            gui.cur_layer = index;
            gui.layer_button[gui.cur_layer].set_disable(systems, true);
        }
        return true;
    }

    false
}
