use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit {
    pub hp: u8,

    pub dig_rate: f32,
    pub carry_max: u8,

    pub speed: f32,
    pub carry_speed: f32,
    
    pub jump_speed: f32,
    pub jump_height: f32,
    pub jump_distance: f32,

    pub climb_speed: f32,
    pub climb_carry_speed: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Selectable {
    pub is_selected: bool,
}

