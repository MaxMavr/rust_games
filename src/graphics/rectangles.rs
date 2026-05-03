use crate::graphics::vectors::Vector2;
use crate::vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    top_left: Vector2,
    bottom_right: Vector2,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn from_corners(top_left: Vector2, bottom_right: Vector2) -> Self {
        
        let width = if bottom_right.x > top_left.x {
            (bottom_right.x - top_left.x) as u32
            } else { 0 };
        let height = if bottom_right.y > top_left.y {
                (bottom_right.y - top_left.y) as u32
            } else { 0 };
        
        Self {
            top_left,
            bottom_right,
            width: width,
            height: height,
        }
    }

    pub fn from_position_width_height(top_left: Vector2, width: u32, height: u32) -> Self {
        let bottom_right = vector2!(top_left.x + width as i32, top_left.y + height as i32);

        Self {
            top_left,
            bottom_right,
            width,
            height,
        }
    }

    // pub fn top_left(&self) -> Vector2 { self.top_left }
    // pub fn bottom_right(&self) -> Vector2 { self.bottom_right }
    // pub fn top_right(&self) -> Vector2 {
    //     vector2!(self.bottom_right.x, self.top_left.y)
    // }
    // pub fn bottom_left(&self) -> Vector2 {
    //     vector2!(self.top_left.x, self.bottom_right.y) 
    // }
    
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width as u32 && y < self.height as u32
    }

    pub fn center(&self) -> Vector2 {
        vector2!((self.width / 2) as i32, (self.height / 2) as i32)
    }
}