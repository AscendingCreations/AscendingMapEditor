use graphics::MapLayers;

#[derive(Clone, Copy)]
pub enum MouseInputType {
    DoubleLeftDown,
    DoubleRightDown,
    LeftDown,
    LeftDownMove,
    RightDown,
    RightDownMove,
    MiddleDown,
    MiddleDownMove,
    Move,
    Release,
}

pub enum SelectedTextbox {
    None,
    SampleTextbox,
    AttrContent,
    ZoneTextbox,
    MapPosTextbox,
}

impl SelectedTextbox {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => SelectedTextbox::None,
            1 => SelectedTextbox::SampleTextbox,
            2 => SelectedTextbox::AttrContent,
            3 => SelectedTextbox::ZoneTextbox,
            4 => SelectedTextbox::MapPosTextbox,
            _ => SelectedTextbox::None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ToolType {
    Move,
    Paint,
    Eraser,
    Fill,
    Picker,
    Count,
}

impl ToolType {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => ToolType::Paint,
            2 => ToolType::Eraser,
            3 => ToolType::Fill,
            4 => ToolType::Picker,
            _ => ToolType::Move,
        }
    }
}

pub fn convert_index_to_maplayers(index: usize) -> MapLayers {
    match index {
        1 => MapLayers::Mask,
        2 => MapLayers::Mask2,
        3 => MapLayers::Anim1,
        4 => MapLayers::Anim2,
        5 => MapLayers::Anim3,
        6 => MapLayers::Anim4,
        7 => MapLayers::Fringe,
        8 => MapLayers::Fringe2,
        _ => MapLayers::Ground,
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TabButton {
    Tileset,
    CustomTiles,
    Attributes,
    Zones,
    DirBlock,
    Weather,
    Music,
    //Properties,
    Count,
}

impl TabButton {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => TabButton::CustomTiles,
            2 => TabButton::Attributes,
            3 => TabButton::Zones,
            4 => TabButton::DirBlock,
            5 => TabButton::Weather,
            6 => TabButton::Music,
            //7 => TabButton::Properties,
            _ => TabButton::Tileset,
        }
    }
}
