use crate::{
    GfxType, SystemHolder,
    content::interface::widget::{
        ContentArea, Scrollbar, ScrollbarBackground, ScrollbarRect, create_label, is_within_area,
    },
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;

#[derive(Clone, Copy)]
pub struct TextListBGRect {
    pub color: Color,
    pub buffer_layer: usize,
    pub order_layer: u32,
    pub got_border: bool,
    pub border_color: Color,
    pub radius: f32,
}

#[derive(Clone, Copy)]
pub struct TextListData {
    pub selection_bufferlayer: usize,
    pub text_bufferlayer: usize,
    pub selection_orderlayer: u32,
    pub text_orderlayer: u32,
    pub selection_color: SelectionColor,
    pub text_color: SelectionColor,
    pub max_list: usize,
}

#[derive(Copy, Clone, Default)]
pub enum TextListBG {
    #[default]
    None,
    Rect(TextListBGRect),
}

pub struct ListData {
    pub data: GfxType,
    pub selection: GfxType,
    is_selected: bool,
    is_hover: bool,

    pub selection_color: SelectionColor,
    pub text_color: SelectionColor,
}

impl ListData {
    pub fn set_list_hover(&mut self, systems: &mut SystemHolder, is_hover: bool) {
        if self.is_hover == is_hover {
            return;
        }

        self.is_hover = is_hover;
        self.update_color(systems);
    }

    pub fn set_list_select(&mut self, systems: &mut SystemHolder, is_select: bool) {
        if self.is_selected == is_select {
            return;
        }

        self.is_selected = is_select;
        self.update_color(systems);
    }

    pub fn update_color(&mut self, systems: &mut SystemHolder) {
        if self.is_selected {
            systems.gfx.set_color(&self.data, self.text_color.selected);
            systems
                .gfx
                .set_color(&self.selection, self.selection_color.selected);
        } else if self.is_hover {
            systems.gfx.set_color(&self.data, self.text_color.hover);
            systems
                .gfx
                .set_color(&self.selection, self.selection_color.hover);
        } else {
            systems.gfx.set_color(&self.data, self.text_color.normal);
            systems
                .gfx
                .set_color(&self.selection, self.selection_color.normal);
        }
    }
}

#[derive(Clone, Copy)]
pub struct SelectionColor {
    pub normal: Color,
    pub hover: Color,
    pub selected: Color,
}

pub struct TextList {
    pub visible: bool,
    bg: GfxType,
    pub scrollbar: Scrollbar,
    pub list: Vec<ListData>,
    pub list_text: Vec<String>,
    pub max_list: usize,
    pub max_data: usize,
    pub selected_list: Option<usize>,

    pub base_pos: Vec2,
    pub adjust_pos: Vec2,
    pub z_order: f32,
    pub size: Vec2,

    text_data: TextListData,
    current_selected: Option<usize>,

    bg_type: TextListBG,
}

impl TextList {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        base_pos: Vec2,
        adjust_pos: Vec2,
        z_pos: f32,
        size: Vec2,
        visible: bool,
        bg_type: TextListBG,
        scrollbar_type: ScrollbarRect,
        scrollbar_bg_type: Option<ScrollbarBackground>,
        list_text: Vec<String>,
        text_data: TextListData,
    ) -> Self {
        let bg = match bg_type {
            TextListBG::Rect(data) => {
                let mut rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(
                        base_pos.x + (adjust_pos.x * systems.scale as f32).floor(),
                        base_pos.y + (adjust_pos.y * systems.scale as f32).floor(),
                        z_pos,
                    ),
                    (size * systems.scale as f32).floor(),
                    data.color,
                    data.order_layer,
                );
                rect.set_radius(data.radius);
                if data.got_border {
                    rect.set_border_width(1.0)
                        .set_border_color(data.border_color);
                }
                systems.gfx.add_rect(
                    rect,
                    data.buffer_layer,
                    "TextList BG",
                    visible,
                    CameraView::SubView1,
                )
            }
            TextListBG::None => GfxType::None,
        };

        let scrollbar_value = list_text.len().saturating_sub(text_data.max_list);

        let scrollbar = Scrollbar::new(
            systems,
            base_pos,
            Vec2::new(adjust_pos.x + size.x - 13.0, adjust_pos.y + 5.0),
            size.y - 10.0,
            8.0,
            true,
            z_pos,
            scrollbar_type,
            scrollbar_bg_type,
            scrollbar_value,
            20.0,
            false,
            visible,
            None,
            true,
            Some(ContentArea::new(adjust_pos, size)),
        );

        let mut list = Vec::with_capacity(list_text.len());
        for (index, data) in list_text.iter().enumerate() {
            if index < text_data.max_list {
                let text_pos = Vec2::new(
                    base_pos.x + ((adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                    base_pos.y
                        + ((adjust_pos.y + ((size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                            * systems.scale as f32)
                            .floor(),
                );
                let text_size = Vec2::new(size.x - 23.0, 20.0);

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(text_pos.x, text_pos.y, z_pos),
                    (text_size * systems.scale as f32).floor(),
                    text_data.selection_color.normal,
                    text_data.selection_orderlayer,
                );

                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, z_pos),
                    (text_size * systems.scale as f32).floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + (text_size.x * systems.scale as f32).floor(),
                        text_pos.y + (text_size.y * systems.scale as f32).floor(),
                    ),
                    text_data.text_color.normal,
                    text_data.text_orderlayer,
                    16.0,
                    16.0,
                    true,
                );
                let text_gfx = systems.gfx.add_text(
                    text,
                    text_data.text_bufferlayer,
                    "TextList Text",
                    visible,
                    CameraView::SubView1,
                );
                systems.gfx.set_text(&mut systems.renderer, &text_gfx, data);
                list.push(ListData {
                    data: text_gfx,
                    selection: systems.gfx.add_rect(
                        rect,
                        text_data.selection_bufferlayer,
                        "TextList Selection",
                        visible,
                        CameraView::SubView1,
                    ),
                    is_hover: false,
                    is_selected: false,
                    selection_color: text_data.selection_color,
                    text_color: text_data.text_color,
                })
            } else {
                break;
            }
        }

        let max_data = list_text.len();

        TextList {
            visible,
            bg,
            base_pos,
            adjust_pos,
            z_order: z_pos,
            size,
            scrollbar,
            list_text,
            list,
            max_list: text_data.max_list,
            max_data,
            current_selected: None,
            selected_list: None,
            text_data,
            bg_type,
        }
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        self.scrollbar.unload(systems);
        self.list.iter().for_each(|data| {
            systems.gfx.remove_gfx(&mut systems.renderer, &data.data);
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &data.selection);
        });
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool, clear_selected: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems.gfx.set_visible(&self.bg, visible);
        self.scrollbar.set_visible(systems, visible);
        self.list.iter_mut().for_each(|data| {
            systems.gfx.set_visible(&data.data, visible);
            systems.gfx.set_visible(&data.selection, visible);

            data.set_list_hover(systems, false);
        });

        if clear_selected {
            self.set_select(systems, None, false);
        }
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.z_order = z_order;

        let pos = systems.gfx.get_pos(&self.bg);
        systems
            .gfx
            .set_pos(&self.bg, Vec3::new(pos.x, pos.y, self.z_order));

        self.scrollbar.set_z_order(systems, self.z_order);

        self.list.iter().for_each(|data| {
            let mut pos = systems.gfx.get_pos(&data.selection);
            pos.z = self.z_order;
            systems.gfx.set_pos(&data.selection, pos);

            let mut pos = systems.gfx.get_pos(&data.data);
            pos.z = self.z_order;
            systems.gfx.set_pos(&data.data, pos);
        });
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.base_pos = new_pos;

        let pos = Vec3::new(
            self.base_pos.x + (self.adjust_pos.x * systems.scale as f32),
            self.base_pos.y + (self.adjust_pos.y * systems.scale as f32),
            self.z_order,
        );
        systems.gfx.set_pos(&self.bg, pos);

        self.scrollbar.set_pos(systems, self.base_pos);

        for (index, data) in self.list.iter().enumerate() {
            let text_pos = Vec2::new(
                self.base_pos.x + ((self.adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                self.base_pos.y
                    + ((self.adjust_pos.y + ((self.size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                        * systems.scale as f32)
                        .floor(),
            );
            let text_size = (Vec2::new(self.size.x - 23.0, 20.0) * systems.scale as f32).floor();

            systems.gfx.set_pos(
                &data.selection,
                Vec3::new(text_pos.x, text_pos.y, self.z_order),
            );

            systems
                .gfx
                .set_pos(&data.data, Vec3::new(text_pos.x, text_pos.y, self.z_order));
            systems.gfx.set_bound(
                &data.data,
                Bounds::new(
                    text_pos.x,
                    text_pos.y,
                    text_pos.x + text_size.x,
                    text_pos.y + text_size.y,
                ),
            );
        }
    }

    pub fn set_size(&mut self, systems: &mut SystemHolder, size: Vec2, max_list: usize) {
        self.size = size;
        match self.bg_type {
            TextListBG::Rect(_) => {
                systems
                    .gfx
                    .set_size(&self.bg, (size * systems.scale as f32).floor());
            }
            TextListBG::None => {}
        }
        self.text_data.max_list = max_list;

        let scrollbar_value = self.list_text.len().saturating_sub(max_list);

        let bar_size = size.y - 10.0;
        let min_bar_size = (bar_size * 0.3).floor().max(4.0);

        self.scrollbar
            .set_size(systems, bar_size, min_bar_size, 8.0);
        self.scrollbar.set_value(systems, 0);
        self.scrollbar.set_max_value(systems, scrollbar_value);

        for list in self.list.iter() {
            systems.gfx.remove_gfx(&mut systems.renderer, &list.data);
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &list.selection);
        }
        self.list.clear();
        for (index, data) in self.list_text.iter().enumerate() {
            if index < max_list {
                let text_pos = Vec2::new(
                    self.base_pos.x + ((self.adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                    self.base_pos.y
                        + ((self.adjust_pos.y + ((size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                            * systems.scale as f32)
                            .floor(),
                );
                let text_size = Vec2::new(size.x - 23.0, 20.0);

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(text_pos.x, text_pos.y, self.z_order),
                    (text_size * systems.scale as f32).floor(),
                    self.text_data.selection_color.normal,
                    self.text_data.selection_orderlayer,
                );

                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, self.z_order),
                    (text_size * systems.scale as f32).floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + (text_size.x * systems.scale as f32).floor(),
                        text_pos.y + (text_size.y * systems.scale as f32).floor(),
                    ),
                    self.text_data.text_color.normal,
                    self.text_data.text_orderlayer,
                    16.0,
                    16.0,
                    true,
                );
                let text_gfx = systems.gfx.add_text(
                    text,
                    self.text_data.text_bufferlayer,
                    "TextList Text",
                    self.visible,
                    CameraView::SubView1,
                );
                systems.gfx.set_text(&mut systems.renderer, &text_gfx, data);
                self.list.push(ListData {
                    data: text_gfx,
                    selection: systems.gfx.add_rect(
                        rect,
                        self.text_data.selection_bufferlayer,
                        "TextList Selection",
                        self.visible,
                        CameraView::SubView1,
                    ),
                    is_hover: false,
                    is_selected: false,
                    selection_color: self.text_data.selection_color,
                    text_color: self.text_data.text_color,
                })
            } else {
                break;
            }
        }

        self.max_list = max_list;

        self.current_selected = None;
        self.selected_list = None;
    }

    pub fn hover_scrollbar(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if !self.visible {
            return;
        }

        if self.scrollbar.in_scroll(screen_pos) {
            self.scrollbar.set_hover(systems, true);
        } else {
            self.scrollbar.set_hover(systems, false);
        }
    }

    pub fn hover_list(&mut self, systems: &mut SystemHolder, screen_pos: Vec2) {
        if !self.visible {
            return;
        }

        let mut did_hover = false;
        for (index, list) in self.list.iter_mut().enumerate() {
            let text_pos = Vec2::new(
                self.base_pos.x + ((self.adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                self.base_pos.y
                    + ((self.adjust_pos.y + ((self.size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                        * systems.scale as f32)
                        .floor(),
            );
            let text_size = Vec2::new(self.size.x - 23.0, 20.0);

            if is_within_area(
                screen_pos,
                Vec2::new(text_pos.x, text_pos.y),
                (text_size * systems.scale as f32).floor(),
            ) && !did_hover
            {
                list.set_list_hover(systems, true);
                did_hover = true;
            } else {
                list.set_list_hover(systems, false);
            }
        }
    }

    pub fn update_list_scroll(&mut self, systems: &mut SystemHolder) {
        if let Some(index) = self.current_selected
            && let Some(list_data) = self.list.get_mut(index)
        {
            list_data.set_list_select(systems, false);
        }

        for (index, list) in self.list.iter_mut().enumerate() {
            let itemindex = self.scrollbar.value + index;
            if itemindex < self.max_data {
                systems.gfx.set_text(
                    &mut systems.renderer,
                    &list.data,
                    &self.list_text[itemindex],
                );
            }
        }

        if let Some(selectedindex) = self.selected_list {
            let current_index = selectedindex as i32 - self.scrollbar.value as i32;
            let max_data = self.max_list.min(self.max_data) as i32;
            if (0..max_data).contains(&current_index) {
                self.current_selected = Some(current_index as usize);
                self.list[current_index as usize].set_list_select(systems, true);
            }
        }
    }

    pub fn set_select(&mut self, systems: &mut SystemHolder, index: Option<usize>, forced: bool) {
        if !self.visible && !forced {
            return;
        }

        if let Some(index) = self.current_selected
            && let Some(list_data) = self.list.get_mut(index)
        {
            list_data.set_list_select(systems, false);
        }

        if let Some(setindex) = index {
            if let Some(selectedindex) = self.selected_list
                && selectedindex == setindex
            {
                return;
            }

            let current_index = setindex as i32 - self.scrollbar.value as i32;
            let max_data = self.max_list.min(self.max_data) as i32;
            if (0..max_data).contains(&current_index) {
                self.current_selected = Some(current_index as usize);
                self.list[current_index as usize].set_list_select(systems, true);
            }
        } else {
            self.current_selected = None;
        }
        self.selected_list = index;
    }

    pub fn unselect_list(&mut self, systems: &mut SystemHolder) {
        if let Some(curindex) = self.current_selected
            && let Some(list_data) = self.list.get_mut(curindex)
        {
            list_data.set_list_select(systems, false);
        }
        self.selected_list = None;
        self.current_selected = None;
    }

    pub fn select_list_by_pos(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
        set_select: bool,
    ) -> Option<usize> {
        if !self.visible {
            return None;
        }

        let mut selected_list = None;
        for (index, _) in self.list.iter().enumerate() {
            let text_pos = Vec2::new(
                self.base_pos.x + ((self.adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                self.base_pos.y
                    + ((self.adjust_pos.y + ((self.size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                        * systems.scale as f32)
                        .floor(),
            );
            let text_size = Vec2::new(self.size.x - 23.0, 20.0);

            if is_within_area(
                screen_pos,
                Vec2::new(text_pos.x, text_pos.y),
                (text_size * systems.scale as f32).floor(),
            ) {
                selected_list = Some(index);
                break;
            }
        }

        if let Some(index) = selected_list {
            if set_select {
                if let Some(curindex) = self.current_selected
                    && let Some(list_data) = self.list.get_mut(curindex)
                {
                    list_data.set_list_select(systems, false);
                }

                self.current_selected = Some(index);
                self.selected_list = Some(self.scrollbar.value + index);
                self.list[index].set_list_select(systems, true);
                self.selected_list
            } else {
                Some(self.scrollbar.value + index)
            }
        } else {
            None
        }
    }

    pub fn update_list(
        &mut self,
        systems: &mut SystemHolder,
        list: Vec<String>,
        selected: Option<usize>,
    ) {
        self.scrollbar.set_value(systems, 0);
        self.current_selected = selected;
        self.max_data = list.len();
        let scrollbar_value = list.len().saturating_sub(self.max_list);
        self.scrollbar.set_max_value(systems, scrollbar_value);

        self.list.iter().for_each(|data| {
            systems.gfx.remove_gfx(&mut systems.renderer, &data.data);
            systems
                .gfx
                .remove_gfx(&mut systems.renderer, &data.selection);
        });
        self.list.clear();

        self.list = Vec::with_capacity(list.len());
        for (index, data) in list.iter().enumerate() {
            if index < self.max_list {
                let text_pos = Vec2::new(
                    self.base_pos.x + ((self.adjust_pos.x + 5.0) * systems.scale as f32).floor(),
                    self.base_pos.y
                        + ((self.adjust_pos.y
                            + ((self.size.y - 5.0) - ((index as f32 + 1.0) * 20.0)))
                            * systems.scale as f32)
                            .floor(),
                );
                let text_size = Vec2::new(self.size.x - 23.0, 20.0);

                let rect = Rect::new(
                    &mut systems.renderer,
                    Vec3::new(text_pos.x, text_pos.y, self.z_order),
                    (text_size * systems.scale as f32).floor(),
                    self.text_data.selection_color.normal,
                    self.text_data.selection_orderlayer,
                );

                let text = create_label(
                    systems,
                    Vec3::new(text_pos.x, text_pos.y, self.z_order),
                    (text_size * systems.scale as f32).floor(),
                    Bounds::new(
                        text_pos.x,
                        text_pos.y,
                        text_pos.x + (text_size.x * systems.scale as f32).floor(),
                        text_pos.y + (text_size.y * systems.scale as f32).floor(),
                    ),
                    self.text_data.text_color.normal,
                    self.text_data.text_orderlayer,
                    16.0,
                    16.0,
                    true,
                );
                let text_gfx = systems.gfx.add_text(
                    text,
                    self.text_data.text_bufferlayer,
                    "TextList Text",
                    self.visible,
                    CameraView::SubView1,
                );
                systems.gfx.set_text(&mut systems.renderer, &text_gfx, data);
                self.list.push(ListData {
                    data: text_gfx,
                    selection: systems.gfx.add_rect(
                        rect,
                        self.text_data.selection_bufferlayer,
                        "TextList Selection",
                        self.visible,
                        CameraView::SubView1,
                    ),
                    is_hover: false,
                    is_selected: false,
                    selection_color: self.text_data.selection_color,
                    text_color: self.text_data.text_color,
                })
            } else {
                break;
            }
        }

        self.list_text = list;
        self.set_select(systems, selected, true);

        self.update_list_scroll(systems);
    }
}
