use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Selectable {
    pub is_selected: bool,
    pub is_hovered: bool,
}
