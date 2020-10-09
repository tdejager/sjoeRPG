use bracket_lib::prelude::*;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::camera::Camera;

const NUM_TILES: usize = (crate::SCREEN_WIDTH * crate::SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Walls,
}

/// The map the hero walks on
pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    /// Create a new map
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    /// Is this point in bounds of the map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x <= SCREEN_WIDTH
            && point.y >= 0 && point.y <= SCREEN_HEIGHT
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..=camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                RGB::named(WHITE),
                                RGB::named(BLACK),
                                to_cp437('.'),
                            );
                        }
                        TileType::Walls => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                RGB::named(WHITE),
                                RGB::named(BLACK),
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Get the index of the map
pub fn map_idx(x: i32, y: i32) -> usize {
    (crate::SCREEN_WIDTH * y + x) as usize
}

