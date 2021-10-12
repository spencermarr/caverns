use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Rock,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_WIDTH) as usize;

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Empty; Map::NUM_TILES],
        }
    }

    pub fn get_index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn is_tile_empty(&self, point: Point) -> bool {
        self.is_in_bounds(point) && self.tiles[Map::get_index(point.x, point.y)] == TileType::Empty
    }

    pub fn try_get_index(&self, point: Point) -> Option<usize> {
        if self.is_in_bounds(point) {
            Some(Map::get_index(point.x, point.y))
        } else {
            None
        }
    }
}