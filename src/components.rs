use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Selectable {
    pub is_selected: &mut bool,
}

impl Selectable {
    pub fn color(&self) -> &str {
        get_hex_color(COLOR_SELECTED)
    }
}