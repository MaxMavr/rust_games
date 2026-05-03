#[derive(Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

use std::ops::Add;
impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

use std::ops::Sub;
impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

use std::fmt;
impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec2({}, {})", self.x, self.y)
    }
}

use crate::utils::to_screen_position::ToScreenPosition;
impl ToScreenPosition for Vector2 {
    fn to_coords(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

#[macro_export]
macro_rules! vector2 {
    ($x:expr, $y:expr) => {
        $crate::graphics::vectors::Vector2::new($x, $y)
    };
}