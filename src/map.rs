use crate::prelude::*;

const NUM_TILES: usize = (crate::SCREEN_WIDTH * crate::SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Walls,
}

/// The map the hero walks on
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    /// Create a new map
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Is this point in bounds of the map
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x <= SCREEN_WIDTH && point.y >= 0 && point.y <= SCREEN_HEIGHT
    }

    /// Checks if this tile can be entered
    pub fn can_enter_tile(&self, position: &Point) -> bool {
        self.in_bounds(position) && self.tiles[map_idx(position.x, position.y)] == TileType::Floor
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut TileType> {
        self.tiles.get_mut(map_idx(point.x, point.y))
    }
}

/// Get the index of the map
pub fn map_idx(x: i32, y: i32) -> usize {
    (crate::SCREEN_WIDTH * y + x) as usize
}

#[system]
/// System that renders the map, only render what is in view of the camera
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let world_point = Point::new(x, y);
            if map.in_bounds(&world_point) {
                let idx = map_idx(world_point.x, world_point.y);
                let position_in_screen = camera.world_to_screen(&world_point);
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(
                            position_in_screen,
                            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
                            to_cp437('.'),
                        );
                    }
                    TileType::Walls => {
                        draw_batch.set(
                            position_in_screen,
                            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
                            to_cp437('#'),
                        );
                    }
                }
            }
        }
    }
}
