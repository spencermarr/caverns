use crate::prelude::*;

#[system]
pub fn render(#[resource] map: &Map) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in 0..=SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let point = Point::new(x, y);
            let index = Map::get_index(x, y);
            let glyph = match map.tiles[index] {
                TileType::Empty => to_cp437('.'),
                TileType::Rock => to_cp437('#'),
            };
            draw_batch.set(
                point,
                ColorPair::new(get_hex_color(COLOR_EMPTY), BLACK),
                glyph,
            );
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
