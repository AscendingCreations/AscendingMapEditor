use crate::{
    content::widget::Textbox, data_types::*, database::MapPosition, gfx_collection::GfxType,
};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
pub enum AlertType {
    #[default]
    Inform,
    Confirm,
    Input,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum AlertIndex {
    #[default]
    None,
    ExitEditor,
    ExitSaveMap(MapPosition),
    LoadTempFile(MapPosition),
    LoadRecoveryFile,
    SavePreset,
}

pub struct AlertTextbox {
    pub bg: GfxType,
    pub textbox: Textbox,
    pub selected: bool,
    pub numeric_only: bool,
}

#[derive(Debug)]
pub struct AlertBuilder {
    pub alert_type: AlertType,
    pub msg: String,
    pub header: String,
    pub width: usize,
    pub custom_index: AlertIndex,
    pub numeric_only: bool,
    pub textbox_limit: usize,
    pub allow_action: bool,
    pub with_checkbox: bool,
    pub checkbox_text: String,
}

impl Default for AlertBuilder {
    fn default() -> Self {
        AlertBuilder {
            alert_type: AlertType::Inform,
            msg: String::new(),
            header: String::new(),
            width: 250,
            numeric_only: false,
            textbox_limit: 10,
            custom_index: AlertIndex::None,
            allow_action: false,
            with_checkbox: false,
            checkbox_text: String::new(),
        }
    }
}

impl AlertBuilder {
    pub fn new_info(header: &str, msg: &str) -> Self {
        Self {
            header: header.to_owned(),
            msg: msg.to_owned(),
            ..Default::default()
        }
    }

    pub fn new_txt_input(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            alert_type: AlertType::Input,
            numeric_only: false,
            ..Default::default()
        }
    }

    pub fn new_num_input(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            alert_type: AlertType::Input,
            numeric_only: true,
            ..Default::default()
        }
    }

    pub fn new_confirm(header: &str, msg: &str) -> Self {
        Self {
            header: header.to_owned(),
            msg: msg.to_owned(),
            alert_type: AlertType::Confirm,
            ..Default::default()
        }
    }

    pub fn with_checkbox(&mut self, msg: &str) -> &mut Self {
        self.checkbox_text = msg.to_owned();
        self.with_checkbox = true;
        self
    }

    pub fn with_msg(&mut self, msg: &str) -> &mut Self {
        self.msg.push_str(msg);
        self
    }

    pub fn with_width(&mut self, max_width: usize) -> &mut Self {
        self.width = max_width;
        self
    }

    pub fn with_numeric_only(&mut self, is_numeric: bool) -> &mut Self {
        self.numeric_only = is_numeric;
        self
    }

    pub fn with_limit(&mut self, textbox_limit: usize) -> &mut Self {
        self.textbox_limit = textbox_limit;
        self
    }

    pub fn with_index(&mut self, custom_index: AlertIndex) -> &mut Self {
        self.custom_index = custom_index;
        self
    }

    pub fn with_type(&mut self, alert_type: AlertType) -> &mut Self {
        self.alert_type = alert_type;
        self
    }

    pub fn with_allow_action(&mut self, allow_action: bool) -> &mut Self {
        self.allow_action = allow_action;
        self
    }
}
