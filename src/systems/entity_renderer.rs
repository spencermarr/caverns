use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Renderable)]
#[read_component(Selectable)]
pub fn render(world: &SubWorld, #[resource] mouse: &mut Mouse) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let selected_fg_color = get_hex_color(COLOR_SELECTED_FG);
    let selected_bg_color = get_hex_color(COLOR_SELECTED_BG);
    let hovered_color = get_hex_color(COLOR_HOVERED);

    <(&Point, &Renderable, &Selectable)>::query()
        .iter(world)
        .for_each(|(point, render, selectable)| {
            let mut color = render.color;
            if selectable.is_selected {
                color.fg = selected_fg_color;
                color.bg = selected_bg_color;

            }
            if mouse.point.eq(point) {
                color.fg = hovered_color;
            }

            draw_batch.set(*point, color, render.glyph);
        });

    draw_batch.submit(4000).expect("Batch error");
}
