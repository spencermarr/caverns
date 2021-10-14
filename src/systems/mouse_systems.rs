use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Selectable)]
pub fn update(world: &mut SubWorld, #[resource] mouse: &mut Mouse) {
    mouse.update();
    mouse.is_hovering_selectable = is_mouse_hovering_selectable(&world, &mouse.point);

    if mouse.selection.state == SelectionState::Released {
        update_selection(world, mouse.selection.get_abs_rect());
    }
}

fn is_mouse_hovering_selectable(world: &SubWorld, mouse_point: &Point) -> bool {
    <(&Point, &Selectable)>::query()
        .iter(world)
        .any(|(&point, _)| point.eq(mouse_point))
}

fn update_selection(world: &mut SubWorld, rect: Rect) {
    let mut selection_rect = rect;
    selection_rect.x2 += 1;
    selection_rect.y2 += 1;

    <(&Point, &mut Selectable)>::query()
        .iter_mut(world)
        .for_each(|(point, mut selectable)| {
            selectable.is_selected = selection_rect.point_in_rect(*point);
        });
}

#[system]
pub fn render(#[resource] mouse: &Mouse) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    if mouse.selection.state == SelectionState::Drag {
        draw_batch.draw_hollow_box(mouse.selection.get_abs_rect(), mouse.color);
    } else if !mouse.is_hovering_selectable {
        draw_batch.set(mouse.point, mouse.color, mouse.cursor_glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}
