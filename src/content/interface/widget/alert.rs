mod alert_builder;

use crate::{
    Content, MouseInputType, SystemHolder,
    content::{
        apply_link_map, apply_map_data, exit_editor,
        interface::widget::{
            button::*, checkbox::*, create_empty_label, get_screen_center, is_within_area,
            measure_string,
        },
        save_preset,
        widget::{Textbox, Tooltip},
    },
    data_types::*,
    database::{
        delete_recovery_map_file, delete_temp_map_file, load_recovery_map_file, load_temp_map_file,
        save_and_clear_map,
    },
    gfx_collection::GfxType,
};
pub use alert_builder::*;
use graphics::{cosmic_text::Attrs, *};
use input::{Key, Named};
use winit::event_loop::ActiveEventLoop;

pub struct Alert {
    window: Vec<GfxType>,
    text: Vec<GfxType>,
    pub button: Vec<Button>,
    pub checkbox: Option<Checkbox>,
    alert_type: AlertType,
    pub input_box: Option<AlertTextbox>,
    pub visible: bool,
    did_button_click: bool,
    custom_index: AlertIndex,
    pub allow_action: bool,
}

impl Default for Alert {
    fn default() -> Self {
        Alert {
            window: Vec::with_capacity(3),
            button: Vec::with_capacity(2),
            alert_type: AlertType::Inform,
            input_box: None,
            text: Vec::with_capacity(2),
            visible: false,
            did_button_click: false,
            custom_index: AlertIndex::None,
            allow_action: false,
            checkbox: None,
        }
    }
}

impl Alert {
    pub fn new() -> Self {
        Alert::default()
    }

    pub fn show_alert(&mut self, systems: &mut SystemHolder, builder: &AlertBuilder) {
        if self.visible {
            self.window.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(&mut systems.renderer, gfx_index);
            });
            self.text.iter().for_each(|gfx_index| {
                systems.gfx.remove_gfx(&mut systems.renderer, gfx_index);
            });
            self.button.iter_mut().for_each(|button| {
                button.unload(systems);
            });
            if let Some(textbox) = &mut self.input_box {
                systems.gfx.remove_gfx(&mut systems.renderer, &textbox.bg);
                textbox.textbox.unload(systems);
            }
        }

        if let Some(checkbox) = &mut self.checkbox {
            checkbox.unload(systems);
        }

        self.window.clear();
        self.text.clear();
        self.button.clear();
        self.input_box = None;
        self.custom_index = builder.custom_index;

        let checkbox_text_size = if builder.with_checkbox {
            measure_string(systems, &builder.checkbox_text, false, 16.0, 16.0).x + 37.0
        } else {
            0.0
        };

        let limit_width = (match builder.alert_type {
            AlertType::Inform => 80.0,
            AlertType::Confirm => 150.0,
            AlertType::Input => 170.0,
        } as f32)
            .max(checkbox_text_size + 20.0);

        let mut text = create_empty_label(systems, 0, Color::rgb(210, 210, 210), true);
        text.set_buffer_size(
            &mut systems.renderer,
            Some((builder.width as f32 * systems.scale as f32).floor()),
            Some(128.0),
        )
        .set_wrap(&mut systems.renderer, cosmic_text::Wrap::Word);
        text.set_text(
            &mut systems.renderer,
            &builder.msg,
            &Attrs::new(),
            Shaping::Advanced,
            None,
        );
        let text_size = text.measure().floor();

        let mut header_text = create_empty_label(systems, 0, Color::rgb(255, 255, 255), true);
        header_text.set_text(
            &mut systems.renderer,
            &builder.header,
            &Attrs::new(),
            Shaping::Advanced,
            None,
        );
        let header_text_size = header_text.measure().floor();

        let text_width = header_text_size.x.max(text_size.x);

        let center = get_screen_center(&systems.size).floor();

        let orig_size = Vec2::new(
            ((text_width / systems.scale as f32).round() + 20.0).max(limit_width),
            ((text_size.y / systems.scale as f32).round() + 90.0).max(110.0),
        );

        let w_size = Vec2::new(
            (text_width + (20.0 * systems.scale as f32).floor())
                .max((limit_width * systems.scale as f32).floor()),
            (text_size.y
                + ((90.0 + if builder.with_checkbox { 30.0 } else { 0.0 }) * systems.scale as f32)
                    .floor())
            .max(
                ((110.0 + if builder.with_checkbox { 30.0 } else { 0.0 }) * systems.scale as f32)
                    .floor(),
            ),
        );
        let w_pos = Vec3::new(
            (center.x - (w_size.x * 0.5)).floor(),
            (center.y - (w_size.y * 0.5)).floor(),
            ORDER_ALERT,
        );

        let (pos, bounds) = if builder.alert_type == AlertType::Input {
            let s_pos = Vec2::new(
                w_pos.x,
                w_pos.y + w_size.y - (30.0 * systems.scale as f32).floor(),
            );
            (
                s_pos,
                Bounds::new(
                    s_pos.x,
                    s_pos.y,
                    s_pos.x + w_size.x,
                    s_pos.y + (20.0 * systems.scale as f32).floor(),
                ),
            )
        } else {
            let s_pos = Vec2::new(
                w_pos.x + (10.0 * systems.scale as f32).floor(),
                w_pos.y + w_size.y - (25.0 * systems.scale as f32).floor(),
            );
            (
                s_pos,
                Bounds::new(
                    s_pos.x,
                    s_pos.y,
                    s_pos.x + header_text_size.x,
                    s_pos.y + (20.0 * systems.scale as f32).floor(),
                ),
            )
        };
        header_text
            .set_pos(Vec3::new(pos.x, pos.y, ORDER_ALERT))
            .set_bounds(bounds);
        header_text.size = Vec2::new(
            header_text_size.x,
            header_text_size.y + (4.0 * systems.scale as f32).floor(),
        );
        header_text.changed = true;
        let header_text_index = systems.gfx.add_text(
            header_text,
            RENDER_ALERT_TEXT,
            "Alert Header Text",
            true,
            CameraView::SubView1,
        );
        if builder.alert_type == AlertType::Input {
            systems.gfx.center_text(&header_text_index);
        }
        self.text.push(header_text_index);

        let bg = Rect::new(
            &mut systems.renderer,
            Vec3::new(0.0, 0.0, ORDER_ALERT_BG),
            Vec2::new(systems.size.width, systems.size.height),
            Color::rgba(0, 0, 0, 150),
            0,
        );

        let mut window = Rect::new(
            &mut systems.renderer,
            w_pos - Vec3::new(1.0, 1.0, 0.0),
            w_size + Vec2::new(2.0, 2.0),
            Color::rgb(100, 100, 100),
            1,
        );
        window
            .set_border_width(1.0)
            .set_border_color(Color::rgb(0, 0, 0));

        self.window.push(systems.gfx.add_rect(
            bg,
            RENDER_ALERT_GUI,
            "Alert BG",
            true,
            CameraView::SubView1,
        ));
        self.window.push(systems.gfx.add_rect(
            window,
            RENDER_ALERT_GUI,
            "Alert Window",
            true,
            CameraView::SubView1,
        ));

        if builder.alert_type != AlertType::Input {
            let pos = Vec2::new(
                w_pos.x + ((w_size.x - text_size.x) * 0.5).floor(),
                w_pos.y
                    + ((43.0 + if builder.with_checkbox { 30.0 } else { 0.0 })
                        * systems.scale as f32)
                        .floor(),
            );
            text.set_pos(Vec3::new(pos.x, pos.y, ORDER_ALERT))
                .set_bounds(Bounds::new(
                    pos.x,
                    pos.y,
                    pos.x + text_size.x,
                    pos.y + text_size.y + (10.0 * systems.scale as f32).floor(),
                ));
            text.size = Vec2::new(
                text_size.x,
                text_size.y + (10.0 * systems.scale as f32).floor(),
            );
            text.changed = true;
            self.text.push(systems.gfx.add_text(
                text,
                RENDER_ALERT_TEXT,
                "Alert Text",
                true,
                CameraView::SubView1,
            ));

            let header = Rect::new(
                &mut systems.renderer,
                Vec3::new(
                    w_pos.x,
                    w_pos.y + w_size.y - (30.0 * systems.scale as f32).floor(),
                    ORDER_ALERT,
                ),
                Vec2::new(w_size.x, (30.0 * systems.scale as f32).floor()),
                Color::rgb(140, 140, 140),
                2,
            );

            self.window.push(systems.gfx.add_rect(
                header,
                RENDER_ALERT_GUI,
                "Alert Header BG",
                true,
                CameraView::SubView1,
            ));
        }

        self.checkbox = if builder.with_checkbox {
            Some(Checkbox::new(
                systems,
                CheckboxType::Rect(CheckboxRect {
                    rect_color: Color::rgb(150, 150, 150),
                    got_border: true,
                    border_color: Color::rgb(0, 0, 0),
                    border_radius: 0.0,
                    hover_change: CheckboxChangeType::ColorChange(Color::rgb(180, 180, 180)),
                    click_change: CheckboxChangeType::ColorChange(Color::rgb(120, 120, 120)),
                    disable_change: CheckboxChangeType::None,
                }),
                CheckType::SetRect(CheckRect {
                    rect_color: Color::rgb(90, 90, 90),
                    got_border: false,
                    border_color: Color::rgb(0, 0, 0),
                    border_radius: 0.0,
                    pos: Vec2::new(5.0, 5.0),
                    size: Vec2::new(14.0, 14.0),
                }),
                Vec2::new(w_pos.x, w_pos.y),
                Vec2::new(
                    ((w_size.x - (checkbox_text_size * systems.scale as f32).floor()) * 0.5)
                        .floor(),
                    45.0,
                ),
                ORDER_ALERT,
                Vec2::new(24.0, 24.0),
                RENDER_ALERT_GUI,
                3,
                RENDER_ALERT_GUI,
                4,
                Some(CheckboxText {
                    text: builder.checkbox_text.clone(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    buffer_layer: RENDER_ALERT_TEXT,
                    order_layer: 3,
                    label_size: Vec2::new(checkbox_text_size - 27.0, 20.0),
                    color: Color::rgb(255, 255, 255),
                    hover_change: CheckboxChangeType::None,
                    click_change: CheckboxChangeType::None,
                    disable_change: CheckboxChangeType::None,
                }),
                true,
                None,
            ))
        } else {
            None
        };

        let button_detail = ButtonRect {
            rect_color: Color::rgb(150, 150, 150),
            got_border: true,
            border_color: Color::rgb(0, 0, 0),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgb(180, 180, 180)),
            click_change: ButtonChangeType::ColorChange(Color::rgb(120, 120, 120)),
            alert_change: ButtonChangeType::None,
            disable_change: ButtonChangeType::None,
        };

        match builder.alert_type {
            AlertType::Inform => {
                let pos = Vec2::new(((orig_size.x - 60.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Okay".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgb(255, 255, 255),
                        buffer_layer: RENDER_ALERT_TEXT,
                        order_layer: 3,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                        alert_change: ButtonChangeType::None,
                        disable_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos,
                    ORDER_ALERT,
                    Vec2::new(60.0, 30.0),
                    2,
                    RENDER_ALERT_GUI,
                    true,
                    None,
                    false,
                ));
            }
            AlertType::Confirm => {
                let pos = Vec2::new(((orig_size.x - 130.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Yes".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgb(255, 255, 255),
                        buffer_layer: RENDER_ALERT_TEXT,
                        order_layer: 3,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                        alert_change: ButtonChangeType::None,
                        disable_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos,
                    ORDER_ALERT,
                    Vec2::new(60.0, 30.0),
                    2,
                    RENDER_ALERT_GUI,
                    true,
                    None,
                    false,
                ));
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail),
                    ButtonContentType::Text(ButtonContentText {
                        text: "No".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgb(255, 255, 255),
                        buffer_layer: RENDER_ALERT_TEXT,
                        order_layer: 3,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                        alert_change: ButtonChangeType::None,
                        disable_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos + Vec2::new(70.0, 0.0),
                    ORDER_ALERT,
                    Vec2::new(60.0, 30.0),
                    2,
                    RENDER_ALERT_GUI,
                    true,
                    None,
                    false,
                ));
            }
            AlertType::Input => {
                let textbox_pos = Vec2::new(((orig_size.x - 100.0) * 0.5).floor(), 50.0);

                let mut textbox_bg = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(
                        w_pos.x + (textbox_pos.x * systems.scale as f32).floor(),
                        w_pos.y + (textbox_pos.y * systems.scale as f32).floor(),
                        ORDER_ALERT,
                    ),
                    (Vec2::new(104.0, 24.0) * systems.scale as f32).floor(),
                    Color::rgb(80, 80, 80),
                    2,
                );

                textbox_bg
                    .set_border_width(1.0)
                    .set_border_color(Color::rgb(0, 0, 0));

                let textbox = Textbox::new(
                    systems,
                    Vec3::new(w_pos.x, w_pos.y, ORDER_ALERT),
                    textbox_pos + Vec2::new(2.0, 2.0),
                    Vec2::new(100.0, 20.0),
                    Color::rgb(255, 255, 255),
                    RENDER_ALERT_GUI,
                    RENDER_ALERT_TEXT,
                    [3, 4, 5],
                    builder.textbox_limit,
                    Color::rgb(110, 110, 110),
                    Color::rgb(150, 150, 150),
                    false,
                    true,
                    None,
                    vec![],
                    true,
                );
                let mut alert_text_box = AlertTextbox {
                    bg: systems.gfx.add_rect(
                        textbox_bg,
                        RENDER_ALERT_GUI,
                        "Alert Input BG",
                        true,
                        CameraView::SubView1,
                    ),
                    textbox,
                    selected: false,
                    numeric_only: builder.numeric_only,
                };
                alert_text_box.textbox.set_select(systems, true);
                alert_text_box.textbox.set_hold(true);
                alert_text_box.selected = true;

                self.input_box = Some(alert_text_box);

                let pos = Vec2::new(((orig_size.x - 150.0) * 0.5).floor(), 10.0);
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Confirm".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgb(255, 255, 255),
                        buffer_layer: RENDER_ALERT_TEXT,
                        order_layer: 3,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                        alert_change: ButtonChangeType::None,
                        disable_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos,
                    ORDER_ALERT,
                    Vec2::new(70.0, 30.0),
                    2,
                    RENDER_ALERT_GUI,
                    true,
                    None,
                    false,
                ));
                self.button.push(Button::new(
                    systems,
                    ButtonType::Rect(button_detail),
                    ButtonContentType::Text(ButtonContentText {
                        text: "Cancel".into(),
                        pos: Vec2::new(0.0, 5.0),
                        color: Color::rgb(255, 255, 255),
                        buffer_layer: RENDER_ALERT_TEXT,
                        order_layer: 3,
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None,
                        alert_change: ButtonChangeType::None,
                        disable_change: ButtonChangeType::None,
                    }),
                    Vec2::new(w_pos.x, w_pos.y),
                    pos + Vec2::new(80.0, 0.0),
                    ORDER_ALERT,
                    Vec2::new(70.0, 30.0),
                    2,
                    RENDER_ALERT_GUI,
                    true,
                    None,
                    false,
                ));
            }
        }

        self.alert_type = builder.alert_type;
        self.allow_action = builder.allow_action;

        self.visible = true;
    }

    pub fn hide_alert(&mut self, systems: &mut SystemHolder) {
        if !self.visible {
            return;
        }
        self.visible = false;
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, gfx_index);
        });
        self.text.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(&mut systems.renderer, gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        if let Some(textbox) = &mut self.input_box {
            systems.gfx.remove_gfx(&mut systems.renderer, &textbox.bg);
            textbox.textbox.unload(systems);
        }
        if let Some(checkbox) = &mut self.checkbox {
            checkbox.unload(systems);
        }
        systems.caret.index = None;
        self.input_box = None;
    }

    pub fn hover_buttons(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        for button in self.button.iter_mut() {
            if button.in_area(systems, screen_pos) {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        }

        if let Some(checkbox) = &mut self.checkbox {
            if checkbox.in_area(systems, screen_pos) {
                checkbox.set_hover(systems, true);
            } else {
                checkbox.set_hover(systems, false);
            }
        }
    }

    pub fn click_buttons(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) -> Option<usize> {
        let mut button_found = None;
        for (index, button) in self.button.iter_mut().enumerate() {
            if button.in_area(systems, screen_pos) {
                button.set_click(systems, true);
                button_found = Some(index)
            }
        }
        button_found
    }

    pub fn click_checkbox(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) -> bool {
        if let Some(checkbox) = &mut self.checkbox
            && checkbox.in_area(systems, screen_pos)
        {
            checkbox.set_click(systems, true);
            return true;
        }
        false
    }

    pub fn reset_buttons(&mut self, systems: &mut SystemHolder) {
        if !self.did_button_click {
            return;
        }
        self.did_button_click = false;

        self.button.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
        if let Some(checkbox) = &mut self.checkbox {
            checkbox.set_click(systems, false);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn alert_mouse_input(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
        elwt: &ActiveEventLoop,
        input_type: MouseInputType,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
        seconds: f32,
    ) -> Result<()> {
        if !self.visible {
            return Ok(());
        }
        match input_type {
            MouseInputType::Move => {
                self.hover_buttons(systems, screen_pos);
                self.hover_textbox(systems, tooltip, screen_pos);
            }
            MouseInputType::LeftDown => {
                let button_index = self.click_buttons(systems, screen_pos);
                if let Some(index) = button_index {
                    self.did_button_click = true;
                    self.select_option(systems, content, elwt, index, seconds)?;
                }
                self.click_textbox(systems, screen_pos);

                if self.click_checkbox(systems, screen_pos) {
                    self.did_button_click = true;
                }
            }
            MouseInputType::Release => {
                self.reset_buttons(systems);
                self.release_textbox();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn alert_key_input(
        &mut self,
        content: &mut Content,
        systems: &mut SystemHolder,
        elwt: &ActiveEventLoop,
        seconds: f32,
        key: &Key,
        pressed: bool,
    ) -> Result<bool> {
        if let Some(textbox) = &mut self.input_box
            && textbox.selected
        {
            textbox
                .textbox
                .enter_text(systems, key, pressed, textbox.numeric_only);
        }

        match self.alert_type {
            AlertType::Confirm | AlertType::Input => match key {
                Key::Named(Named::Enter) => {
                    self.select_option(systems, content, elwt, 0, seconds)?;
                    return Ok(true);
                }
                Key::Named(Named::Escape) => {
                    self.select_option(systems, content, elwt, 1, seconds)?;
                    return Ok(true);
                }
                _ => {}
            },
            AlertType::Inform => {
                if let Key::Named(Named::Enter) = key {
                    self.select_option(systems, content, elwt, 0, seconds)?;
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    pub fn select_option(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
        elwt: &ActiveEventLoop,
        index: usize,
        seconds: f32,
    ) -> Result<()> {
        let checkbox_value = self.checkbox.as_ref().is_some_and(|c| c.value);

        match self.alert_type {
            AlertType::Inform =>
            {
                #[allow(clippy::match_single_binding)]
                match self.custom_index {
                    _ => self.hide_alert(systems),
                }
            }
            AlertType::Confirm => {
                match index {
                    #[allow(clippy::match_single_binding)]
                    0 => match self.custom_index {
                        AlertIndex::ExitEditor => exit_editor(content, systems, self, elwt, None)?,
                        AlertIndex::LoadTempFile(mappos) => {
                            if let Ok(mapdata) =
                                load_temp_map_file(mappos.x, mappos.y, mappos.group as u64)
                            {
                                apply_map_data(content, systems, &mapdata);
                                apply_link_map(content, mappos);
                                content.data.mapdata = mapdata;
                                content.data.pos = Some(mappos);
                                content.data.changed = true;
                                content.data.temp_saved = true;
                                content.interface.footer.set_map_pos(systems, mappos, true);

                                content.interface.notification.add_msg(
                                    systems,
                                    format!(
                                        "Map [X: {} Y: {} Group: {}] Reloaded!",
                                        mappos.x, mappos.y, mappos.group
                                    ),
                                    seconds,
                                );
                            }

                            self.hide_alert(systems)
                        }
                        AlertIndex::ExitSaveMap(mappos) => {
                            save_and_clear_map(mappos.x, mappos.y, mappos.group as u64)?;
                            exit_editor(
                                content,
                                systems,
                                self,
                                elwt,
                                if checkbox_value { Some(true) } else { None },
                            )?
                        }
                        AlertIndex::LoadRecoveryFile => {
                            if let Ok(mapdata) = load_recovery_map_file() {
                                apply_map_data(content, systems, &mapdata);
                                content.data.mapdata = mapdata;
                                content.data.pos = None;
                                content.data.changed = true;
                                content.data.temp_saved = true;
                                content.interface.footer.remove_map_pos(systems);

                                content.interface.notification.add_msg(
                                    systems,
                                    "Recovered unsaved map!".to_string(),
                                    seconds,
                                );
                            }

                            self.hide_alert(systems)
                        }
                        _ => self.hide_alert(systems),
                    }, // Yes
                    #[allow(clippy::match_single_binding)]
                    _ => match self.custom_index {
                        AlertIndex::LoadTempFile(mappos) => {
                            delete_temp_map_file(mappos.x, mappos.y, mappos.group as u64)?;
                            self.hide_alert(systems)
                        }
                        AlertIndex::ExitSaveMap(_) => exit_editor(
                            content,
                            systems,
                            self,
                            elwt,
                            if checkbox_value { Some(true) } else { None },
                        )?,
                        AlertIndex::LoadRecoveryFile => {
                            delete_recovery_map_file()?;
                            self.hide_alert(systems)
                        }
                        _ => self.hide_alert(systems),
                    }, // No
                }
            }
            AlertType::Input => {
                if let Some(textbox) = &self.input_box {
                    let input_text = textbox.textbox.text.clone();
                    match index {
                        #[allow(clippy::match_single_binding)]
                        0 => match self.custom_index {
                            AlertIndex::SavePreset => {
                                save_preset(content, systems, input_text)?;
                                self.hide_alert(systems)
                            }
                            _ => self.hide_alert(systems),
                        }, // Yes
                        #[allow(clippy::match_single_binding)]
                        _ => match self.custom_index {
                            _ => self.hide_alert(systems),
                        }, // No
                    }
                }
            }
        }
        Ok(())
    }

    pub fn hover_textbox(
        &mut self,
        systems: &mut SystemHolder,
        tooltip: &mut Tooltip,
        screen_pos: Vec2,
    ) {
        if let Some(textbox) = &mut self.input_box
            && is_within_area(
                screen_pos,
                Vec2::new(textbox.textbox.base_pos.x, textbox.textbox.base_pos.y)
                    + (textbox.textbox.adjust_pos * systems.scale as f32).floor(),
                (textbox.textbox.size * systems.scale as f32).floor(),
            )
            && let Some(msg) = &textbox.textbox.tooltip
        {
            tooltip.init_tooltip(systems, screen_pos, msg.clone(), false);
        }
    }

    pub fn click_textbox(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if let Some(textbox) = &mut self.input_box {
            if is_within_area(
                screen_pos,
                Vec2::new(textbox.textbox.base_pos.x, textbox.textbox.base_pos.y)
                    + (textbox.textbox.adjust_pos * systems.scale as f32).floor(),
                (textbox.textbox.size * systems.scale as f32).floor(),
            ) {
                textbox.textbox.set_select(systems, true);
                textbox.textbox.set_hold(true);
                textbox.textbox.select_text(systems, screen_pos);
                textbox.selected = true;
            } else {
                textbox.textbox.set_select(systems, false);
                textbox.selected = false;
            }
        }
    }

    pub fn release_textbox(&mut self) {
        if let Some(textbox) = &mut self.input_box
            && textbox.selected
        {
            textbox.textbox.set_hold(false);
        }
    }

    pub fn hold_move_textbox(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if let Some(textbox) = &mut self.input_box
            && textbox.selected
        {
            textbox.textbox.hold_move(systems, screen_pos);
        }
    }
}
