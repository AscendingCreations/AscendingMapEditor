use graphics::Vec2;

use crate::{
    content::{
        Content,
        interface::sample_window::SampleWindow,
        widget::{Alert, AlertBuilder},
    },
    renderer::SystemHolder,
};

impl SampleWindow {
    pub fn hover_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        let button_hover = self.sample_button.in_area(systems, mouse_pos);
        self.sample_button.set_hover(systems, button_hover);

        let checkbox_hover = self.sample_checkbox.in_area(systems, mouse_pos);
        self.sample_checkbox.set_hover(systems, checkbox_hover);

        let scrollbar_hover = self.sample_scrollbar.in_scroll(mouse_pos);
        self.sample_scrollbar.set_hover(systems, scrollbar_hover);

        self.sample_textlist.hover_list(systems, mouse_pos);
        self.sample_textlist.hover_scrollbar(systems, mouse_pos);

        self.sample_optionlist.hover_widget(systems, mouse_pos);
    }

    pub fn reset_widgets(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.sample_button.set_click(systems, false);
        self.sample_checkbox.set_click(systems, false);
        self.sample_scrollbar.set_hold(systems, false, mouse_pos);
        self.sample_textlist
            .scrollbar
            .set_hold(systems, false, mouse_pos);
        self.sample_optionlist.reset_widget(systems, mouse_pos);
    }

    pub fn hold_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.sample_scrollbar.in_scroll(mouse_pos) {
            self.sample_scrollbar.set_hold(systems, true, mouse_pos);
            return true;
        }

        if self.sample_textlist.scrollbar.in_scroll(mouse_pos) {
            self.sample_textlist
                .scrollbar
                .set_hold(systems, true, mouse_pos);
            return true;
        }

        if self.sample_optionlist.list.visible
            && self.sample_optionlist.list.scrollbar.in_scroll(mouse_pos)
        {
            self.sample_optionlist
                .list
                .scrollbar
                .set_hold(systems, true, mouse_pos);
            return true;
        }

        false
    }

    pub fn hold_move_scrollbar(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) {
        self.sample_scrollbar.set_move_scroll(systems, mouse_pos);

        self.sample_textlist
            .scrollbar
            .set_move_scroll(systems, mouse_pos);
        self.sample_textlist.update_list_scroll(systems);

        if self.sample_optionlist.list.visible {
            self.sample_optionlist
                .list
                .scrollbar
                .set_move_scroll(systems, mouse_pos);
            self.sample_optionlist.list.update_list_scroll(systems);
        }
    }

    pub fn click_buttons(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        if self.sample_button.in_area(systems, mouse_pos) {
            self.sample_button.set_click(systems, true);
            return Some(0);
        }
        None
    }

    pub fn click_checkbox(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.sample_checkbox.in_area(systems, mouse_pos) {
            self.sample_checkbox.set_click(systems, true);
            return true;
        }
        false
    }

    pub fn click_textbox(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> bool {
        if self.sample_textbox.in_area(systems, mouse_pos) {
            self.sample_textbox.set_select(systems, true);
            self.sample_textbox.set_hold(true);
            self.sample_textbox.select_text(systems, mouse_pos);
            true
        } else {
            false
        }
    }

    pub fn click_textlist(&mut self, systems: &mut SystemHolder, mouse_pos: Vec2) -> Option<usize> {
        self.sample_textlist.select_list(systems, mouse_pos, true)
    }

    pub fn click_optionlist(
        &mut self,
        systems: &mut SystemHolder,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let in_button_area = self.sample_optionlist.button.in_area(systems, mouse_pos);

        if self.sample_optionlist.list.visible {
            let result = self
                .sample_optionlist
                .list
                .select_list(systems, mouse_pos, true);

            if let Some(index) = result {
                self.sample_optionlist.update_label(systems, index);
                self.sample_optionlist
                    .list
                    .set_visible(systems, false, false);
                return result;
            }
        }

        if in_button_area {
            self.sample_optionlist.button.set_click(systems, true);
            self.sample_optionlist.list.set_visible(
                systems,
                !self.sample_optionlist.list.visible,
                false,
            );
        }

        None
    }
}

pub fn sample_click_widget(
    content: &mut Content,
    systems: &mut SystemHolder,
    alert: &mut Alert,
    mouse_pos: Vec2,
) -> bool {
    let gui = &mut content.interface.sample;

    if gui.click_checkbox(systems, mouse_pos) || gui.hold_scrollbar(systems, mouse_pos) {
        return true;
    }

    if gui.click_buttons(systems, mouse_pos).is_some() {
        let textbox_text = gui.sample_textbox.text.clone();
        alert.show_alert(systems, &AlertBuilder::new_info("Message", &textbox_text));
        return true;
    }

    if let Some(list_index) = gui.click_textlist(systems, mouse_pos) {
        println!("Text List Index: [{list_index}]");
        return true;
    }

    if let Some(list_index) = gui.click_optionlist(systems, mouse_pos) {
        println!("Option List Index: [{list_index}]");
        return true;
    }

    false
}
