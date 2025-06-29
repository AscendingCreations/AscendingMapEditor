use graphics::MapLayers;

pub const TEXTURE_SIZE: u32 = 20;
pub const ZOOM_LEVEL: f32 = 1.0;

pub const ORDER_BG: f32 = 11.95;
pub const ORDER_ALPHA_BG: f32 = 11.94;
pub const ORDER_BG_BUTTON: f32 = 11.85;
pub const ORDER_ATTRIBUTE_BG: f32 = 11.84;
pub const ORDER_ATTRIBUTE_HEADER_BG: f32 = 11.8;
pub const ORDER_ATTRIBUTE_LABEL: f32 = 11.7;
pub const ORDER_ATTRIBUTE_TEXTBOX: f32 = 11.7;
pub const ORDER_ATTRIBUTE_RECT: f32 = 11.7;
pub const ORDER_OPTION_BUTTON: f32 = 11.75;
pub const ORDER_OPTION_BUTTON_TEXT: f32 = 11.74;
pub const ORDER_PROPERTIES_BUTTON: f32 = 11.85;
pub const ORDER_PROPERTIES_BUTTON_TEXT: f32 = 11.84;
pub const ORDER_DROPDOWN_WINDOW: f32 = 11.79;
pub const ORDER_DROPDOWN_SELECTION: f32 = 11.78;
pub const ORDER_DROPDOWN_TEXT: f32 = 11.77;
pub const ORDER_DROPDOWN_SCROLLBAR: f32 = 11.76;
pub const ORDER_BG_LABEL: f32 = 11.7;
pub const ORDER_TAB_BUTTON: f32 = 11.6;
pub const ORDER_TAB_SCROLLBAR_BG: f32 = 11.6;
pub const ORDER_TAB_SCROLLBAR: f32 = 11.5;
pub const ORDER_TAB_LABEL: f32 = 11.5;
// Lower Map Order 9.4 - 9.0
// Upper Map Order 5.1 - 5.0
pub const ORDER_MAP_ATTRIBUTE_BG: f32 = 4.9;
pub const ORDER_MAP_ZONE: f32 = 4.9;
pub const ORDER_MAP_DIRBLOCK: f32 = 4.9;
pub const ORDER_MAP_ATTRIBUTE_TEXT: f32 = 4.8;
pub const ORDER_MAP_SELECTION: f32 = 4.0;
pub const ORDER_MAP_LINK_SELECT: f32 = 4.0;
pub const ORDER_TILESET_SELECTION: f32 = 4.0;
pub const ORDER_TILESETLIST: f32 = 3.9;
pub const ORDER_TILESETLIST_SCROLL_BG: f32 = 3.8;
pub const ORDER_TILESETLIST_BUTTON: f32 = 3.7;
pub const ORDER_TILESETLIST_SCROLLBAR: f32 = 3.7;
pub const ORDER_TILESETLIST_LABEL: f32 = 3.6;
pub const ORDER_DIALOG_SHADOW: f32 = 2.9;
pub const ORDER_DIALOG_WINDOW: f32 = 2.8;
pub const ORDER_DIALOG_MSG: f32 = 2.7;
pub const ORDER_DIALOG_CONTENT_IMG1: f32 = 2.7;
pub const ORDER_DIALOG_CONTENT_IMG2: f32 = 2.6;
pub const ORDER_DIALOG_CONTENT_TEXT: f32 = 2.5;
pub const ORDER_DIALOG_SCROLLBAR: f32 = 2.5;
pub const ORDER_DIALOG_BUTTON: f32 = 2.5;
pub const ORDER_DIALOG_BUTTON_TEXT: f32 = 2.4;
pub const ORDER_PREFERENCE_SHADOW: f32 = 2.9;
pub const ORDER_PREFERENCE_WINDOW: f32 = 2.89;
pub const ORDER_PREFERENCE_MENU: f32 = 2.88;
pub const ORDER_PREFERENCE_BUTTON: f32 = 2.87;
pub const ORDER_PREFERENCE_BUTTON_TEXT: f32 = 2.86;
pub const ORDER_PREFERENCE_MENU_BUTTON: f32 = 2.87;
pub const ORDER_PREFERENCE_MENU_BUTTON_TEXT: f32 = 2.86;
pub const ORDER_PREFERENCE_SCROLLBAR: f32 = 2.85;
pub const ORDER_PREFERENCE_KEYLIST_BUTTON: f32 = 2.85;
pub const ORDER_PREFERENCE_KEYLIST_TEXT: f32 = 2.84;
pub const ORDER_PREFERENCE_SETTING_IMG1: f32 = 2.87;
pub const ORDER_PREFERENCE_SETTING_IMG2: f32 = 2.86;
pub const ORDER_PREFERENCE_SETTING_TEXT: f32 = 2.86;
pub const ORDER_COLOREDIT_WINDOW: f32 = 2.59;
pub const ORDER_COLOREDIT_TEXTBOX: f32 = 2.58;
pub const ORDER_COLOREDIT_TEXTBOX_TEXT: f32 = 2.58;
pub const ORDER_COLOREDIT_BUTTON: f32 = 2.58;
pub const ORDER_COLOREDIT_BUTTON_LABEL: f32 = 2.57;
pub const ORDER_KEYBIND_WINDOW: f32 = 2.59;
pub const ORDER_KEYBIND_TEXT: f32 = 2.56;
pub const ORDER_KEYBIND_BUTTON: f32 = 2.56;
pub const ORDER_KEYBIND_BUTTON_TEXT: f32 = 2.55;

pub const MAP_LAYERS: [MapLayers; 9] = [
    MapLayers::Ground,
    MapLayers::Mask,
    MapLayers::Mask2,
    MapLayers::Anim1,
    MapLayers::Anim2,
    MapLayers::Anim3,
    MapLayers::Anim4,
    MapLayers::Fringe,
    MapLayers::Fringe2,
];
