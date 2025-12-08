// General
pub const TEXTURE_SIZE: u32 = 20;
pub const TILESET_COUNT_X: u32 = 10;
pub const TILESET_COUNT_Y: u32 = 20;
pub const MAX_TILE: usize = 1024;
pub const MAX_CHANGES: usize = 64;
pub const MAX_PRESETS: usize = 100;

// Editor
pub const MAX_VISIBLE_ATTRIBUTE: usize = 12;

// GFX Z Order
pub const ORDER_LINKED_TILE_BG: f32 = 10.0;
pub const ORDER_TILE_BG: f32 = 10.0;
pub const ORDER_TILE_SELECT: f32 = 9.95;
pub const ORDER_ATTR_PREVIEW: f32 = 9.94;
pub const ORDER_WINDOW: f32 = 9.9;
pub const ORDER_WINDOW_CONTENT: f32 = 9.8;
pub const ORDER_WINDOW_CONTENT2: f32 = 9.7;
pub const ORDER_TILESET: f32 = 9.7;
pub const ORDER_ABOVE_WINDOW: f32 = 9.6;
pub const ORDER_MENU_BAR: f32 = 8.9;
pub const ORDER_MAPPOS_BG: f32 = 0.9;
pub const ORDER_MAPPOS: f32 = 0.8;
pub const ORDER_ALERT_BG: f32 = 0.7;
pub const ORDER_ALERT: f32 = 0.6;
pub const ORDER_TOOLTIP: f32 = 0.5;
pub const ORDER_NOTIFICATION: f32 = 0.4;

// GFX Render Order
pub const RENDER_IMG: usize = 0;
pub const RENDER_TOP_MAP: usize = 1;
pub const RENDER_LIGHT: usize = 2;
pub const RENDER_GUI: usize = 3;
pub const RENDER_GUI2: usize = 4;
pub const RENDER_GUI3: usize = 5;
pub const RENDER_GUI_TEXT: usize = 6;
pub const RENDER_GUI_TEXT2: usize = 7;
pub const RENDER_GUI_TEXT3: usize = 8;
pub const RENDER_MAPPOS_GUI: usize = 9;
pub const RENDER_MAPPOS_TEXT: usize = 10;
pub const RENDER_ALERT_GUI: usize = 11;
pub const RENDER_ALERT_TEXT: usize = 12;
pub const RENDER_TOOLTIP: usize = 13;
pub const RENDER_TOOLTIP_TEXT: usize = 14;
pub const RENDER_NOTIFICATION: usize = 15;
pub const RENDER_NOTIFICATION_TEXT: usize = 16;
pub const MAX_RENDER: usize = 16;
