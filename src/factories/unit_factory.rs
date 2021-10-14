use crate::prelude::*;

pub struct UnitFactory;

impl UnitFactory {
    pub fn spawn_dwarf(ecs: &mut World, point: Point) {
        ecs.push((
            Player,
            point,
            Renderable {
                color: ColorPair::new(BURLYWOOD, BLACK),
                glyph: to_cp437('a'),
            },
            Selectable { is_selected: false },
        ));
    }
}
