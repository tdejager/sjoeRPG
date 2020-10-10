use crate::prelude::*;

/// Camera centered on the player
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
    /// Offset of the top left corner of the camera
    pub top_left_corner: Point,
}

impl Camera {
    /// Create a new camera
    pub fn new(center_point: Point) -> Self {
        let left_x = center_point.x - DISPLAY_WIDTH / 2;
        let top_y = center_point.y - DISPLAY_HEIGHT / 2;
        let right_x = center_point.x + DISPLAY_WIDTH / 2;
        let bottom_y = center_point.y + DISPLAY_HEIGHT / 2;
        Self {
            left_x,
            right_x,
            top_y,
            bottom_y,
            top_left_corner: Point::new(left_x, top_y),
        }
    }

    /// Move the camera centered on the player
    pub fn recenter(&mut self, center_point: Point) {
        self.left_x = center_point.x - DISPLAY_WIDTH / 2;
        self.right_x = center_point.x + DISPLAY_WIDTH / 2;
        self.top_y = center_point.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = center_point.y + DISPLAY_HEIGHT / 2;
        self.top_left_corner = Point::new(self.left_x, self.top_y);
    }

    /// Transforms world point to screen
    pub fn world_to_screen(&self, point_world: &Point) -> Point {
        *point_world - self.top_left_corner
    }
}
