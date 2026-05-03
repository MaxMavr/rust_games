use crate::graphics::rectangles::Rectangle;
use crate::utils::to_screen_position::ToScreenPosition;
use crate::vector2;

pub struct Buffer {
    pixels: Vec<u32>,
    rectangle: Rectangle,
}

impl Buffer {
    pub fn new(width: usize, height: usize,) -> Self {
        let pixels = vec![0; width * height];
        let rectangle = 
            Rectangle::from_position_width_height(
                vector2!(0, 0),
                width as u32,
                height as u32
            );

        Self {
            pixels,
            rectangle,
        }
    }

    pub fn pixels(&self) -> &Vec<u32> { &self.pixels }
    pub fn rectangle(&self) -> &Rectangle { &self.rectangle }

    fn _put_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = y * self.rectangle.width() as usize + x;
        self.pixels[index] = color;
    }

    pub fn fill(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn put_pixel<C: ToScreenPosition>(&mut self, coords: C, color: u32) {
        let (x, y) = coords.to_coords();
        
        if self.rectangle.in_bounds(x as u32, y as u32) {
            self._put_pixel(x, y, color);
        }
    }
}