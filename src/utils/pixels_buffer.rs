use crate::graphics::colors::ToColor;
use crate::graphics::fonts::bdf::FontBDF;
use crate::graphics::rectangles::Rectangle;
use crate::graphics::size::Size;
use crate::graphics::vectors::Vector2;
use crate::utils::to_screen_position::ToScreenPosition;
use crate::vector2;

pub struct Buffer {
    pixels: Vec<u32>,
    rectangle: Rectangle,
}

impl Buffer {
    pub fn new(size: Size) -> Self {
        let pixels = vec![0; size.area() as usize];
        let rectangle = Rectangle::from_top_left(vector2!(0, 0), size);

        Self { pixels, rectangle }
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
    pub fn rectangle(&self) -> &Rectangle {
        &self.rectangle
    }

    fn _put_pixel<C: ToColor>(&mut self, x: usize, y: usize, color: C) {
        let index = y * self.rectangle.width() as usize + x;
        self.pixels[index] = color.to_color();
    }

    pub fn fill<C: ToColor>(&mut self, color: C) {
        self.pixels.fill(color.to_color());
    }

    pub fn put_pixel<P: ToScreenPosition, C: ToColor>(&mut self, coords: P, color: C) {
        let (x, y) = coords.to_coords();

        if self.rectangle.in_bounds(x as i32, y as i32) {
            self._put_pixel(x, y, color);
        }
    }

    pub fn print<C: ToColor>(
        &mut self,
        start_point: Vector2,
        text: &str,
        font: &FontBDF,
        color: C,
        multiplier: Vector2,
    ) {
        let target_color = color.to_color();
        let mut cursor_x = start_point.x;
        let mut cursor_y = start_point.y;

        let buf_width = self.rectangle().width() as i32;
        let buf_height = self.rectangle().height() as i32;
        
        let multiplier_x = multiplier.x.max(1);
        let multiplier_y = multiplier.y.max(1);

        for ch in text.chars() {
            if let Some(glyph) = font.get_glyph(ch as i32) {
                let size = glyph.size();
                let bitmap = glyph.bitmap();
                let offset = glyph.offset();
                let advance = glyph.advance();

                let width = size.width as usize;
                let height = size.height as usize;

                if width == 0 || height == 0 {
                    cursor_x += advance.x * multiplier_x;
                    cursor_y += advance.y * multiplier_y;
                    continue;
                }

                let width_bytes = (width + 7) / 8;

                let start_x = cursor_x + offset.x * multiplier_x;
                let start_y = cursor_y - (offset.y + height as i32) * multiplier_y;

                for row in 0..height {
                    let screen_y = start_y + row as i32 * multiplier_y;

                    let y0 = screen_y.max(0);
                    let y1 = (screen_y + multiplier_y).min(buf_height);

                    if y0 >= y1 {
                        continue;
                    }

                    let row_offset = row * width_bytes;

                    let row_data = if row_offset + width_bytes <= bitmap.len() {
                        &bitmap[row_offset..row_offset + width_bytes]
                    } else {
                        break;
                    };

                    for byte_idx in 0..width_bytes {
                        let byte = row_data[byte_idx];
                        if byte == 0 {
                            continue;
                        }

                        let base_screen_x = start_x + (byte_idx * 8) as i32 * multiplier_x;

                        for bit in 0..8 {
                            let col = byte_idx * 8 + bit;
                            if col >= width {
                                break;
                            }

                            if (byte >> (7 - bit)) & 1 != 0 {
                                let screen_x = base_screen_x + bit as i32 * multiplier_x;
                                
                                let x0 = screen_x.max(0);
                                let x1 = (screen_x + multiplier_x).min(buf_width);

                                if x0 >= x1 {
                                    continue;
                                }

                                for y in y0..y1 {
                                    let row_start = y * buf_width;

                                    for x in x0..x1 {
                                        self.pixels[(row_start + x) as usize] = target_color;
                                    }
                                }
                            }
                        }
                    }
                }

                cursor_x += advance.x * multiplier_x;
                cursor_y += advance.y * multiplier_y;
            }
        }
    }
}
