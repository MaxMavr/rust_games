use crate::graphics::size::Size;
use crate::graphics::vectors::Vector2;
use crate::vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub top_left: Vector2,
    pub size: Size,
}

impl Rectangle {
    pub fn from_top_left(top_left: Vector2, size: Size) -> Self {
        Self { top_left, size }
    }

    pub fn from_center(center: Vector2, size: Size) -> Self {
        let half_w = (size.width as i32) / 2;
        let half_h = (size.height as i32) / 2;

        let top_left = vector2!(center.x - half_w, center.y - half_h);

        Self { top_left, size }
    }

    pub fn from_corners(top_left: Vector2, bottom_right: Vector2) -> Self {
        let width = (bottom_right.x - top_left.x).unsigned_abs();
        let height = (bottom_right.y - top_left.y).unsigned_abs();

        Self {
            top_left,
            size: Size::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= self.top_left.x
            && x < self.top_left.x + self.size.width as i32
            && y >= self.top_left.y
            && y < self.top_left.y + self.size.height as i32
    }

    pub fn contains(&self, point: Vector2) -> bool {
        self.in_bounds(point.x, point.y)
    }

    pub fn center(&self) -> Vector2 {
        vector2!(
            self.top_left.x + (self.size.width as i32) / 2,
            self.top_left.y + (self.size.height as i32) / 2
        )
    }
}
