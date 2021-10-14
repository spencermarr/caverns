use bracket_lib::prelude::MAGENTA;

use crate::prelude::*;

pub const COLOR_EMPTY: &str = "#3D3D3DFF";
pub const COLOR_SELECTED_FG: &str = "#000000FF";
pub const COLOR_SELECTED_BG: &str = "#D3D3D3FF";
pub const COLOR_HOVERED: &str = "#FFFFFFFF";

pub fn get_hex_color(hex: &str) -> RGBA {
    match RGBA::from_hex(hex) {
        Ok(color) => color,
        Err(_e) => RGBA::named(MAGENTA),
    }
}
