use bracket_lib::prelude::MAGENTA;

use crate::prelude::*;

pub const COLOR_EMPTY: &str = "#3D3D3D";
pub const COLOR_SELECTED: &str = "#1D1D1D";

pub fn get_hex_color(hex: &str) -> RGB {
    match RGB::from_hex(hex) {
        Ok(color) => color,
        Err(_e) => RGB::named(MAGENTA),
    }
}