use bracket_lib::prelude::MAGENTA;

use crate::prelude::*;

pub const COLOR_EMPTY: &str = "#3D3D3D";
pub const COLOR_SELECTED: &str = "#D3D3D3";
pub const COLOR_HOVERED: &str = "#C4BD35";

pub fn get_hex_color(hex: &str) -> RGB {
    match RGB::from_hex(hex) {
        Ok(color) => color,
        Err(_e) => RGB::named(MAGENTA),
    }
}