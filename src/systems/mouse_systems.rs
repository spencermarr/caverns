use crate::prelude::*;

#[system]
pub fn update(#[resource] mouse: &mut Mouse) {
    mouse.update();
}

#[system]
pub fn render(#[resource] mouse: &Mouse) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    if mouse.selection.state == SelectionState::Drag {
               
        draw_batch.draw_hollow_box(
            mouse.selection.get_drawable_rect(),
            mouse.color,
        );
    } else {
        draw_batch.set(mouse.point, mouse.color, mouse.cursor_glyph);
    }

    draw_batch.submit(4000).expect("Batch error");
}
