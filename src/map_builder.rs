use crate::prelude::*;
const NUM_ROOMS: usize = 10;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles = vec![tile; self.map.tiles.len()];
    }

    fn random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // Generate a room for the number of rooms
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 20),
                rng.range(2, 20),
            );

            // Check if the rooms overlaps with an existing one
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // We can place it
            if !overlap {
                // Create a floor if it falls within bounds
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                })
            }
            self.rooms.push(room);
        }
    }

    /// Creates a vertical tunnel
    fn create_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(tile) = self.map.get_mut(&Point::new(x, y)) {
                *tile = TileType::Floor;
            }
        }
    }

    /// Creates a horizontal tunnel
    fn create_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(tile) = self.map.get_mut(&Point::new(x, y)) {
                *tile = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // Sort by x-coordinate
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.create_horizontal_tunnel(prev.x, new.x, prev.y);
                self.create_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.create_vertical_tunnel(prev.y, new.y, prev.x);
                self.create_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn build(rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Default::default(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Walls);
        mb.random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }
}
