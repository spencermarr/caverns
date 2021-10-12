use crate::prelude::*;

pub struct MapFactory;

impl MapFactory {
    const SKY_HEIGHT: i32 = 4;

    pub fn build_map() -> Map {
        let mut map = Map::new();
        MapFactory::fill(&mut map, TileType::Rock);
        MapFactory::fill_rect(
            &mut map,
            TileType::Empty,
            Rect {
                x1: 0,
                x2: SCREEN_WIDTH,
                y1: 0,
                y2: MapFactory::SKY_HEIGHT - 1,
            },
        );
        map
    }

    fn fill(map: &mut Map, tile: TileType) {
        map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn fill_rect(map: &mut Map, tile: TileType, rect: Rect) {
        for y in rect.y1..=rect.y2 {
            for x in rect.x1..=rect.x2 {
                if let Some(idx) = map.try_get_index(Point::new(x, y)) {
                    map.tiles[idx as usize] = tile;
                }
            }
        }
    }
}
