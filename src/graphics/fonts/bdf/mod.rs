mod builders;
mod extractors;

mod errors;
mod parser;

use crate::graphics::size::Size;
use crate::graphics::vectors::Vector2;
use errors::BdfError;
use parser::parse;

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct GlyphBDF {
    encoding: i32,    // ENCODING
    advance: Vector2, // DWIDTH
    size: Size,       // BBX
    offset: Vector2,  // BBOFF
    bitmap: Vec<u8>,  // BITMAP
}

impl GlyphBDF {
    pub fn new(
        encoding: i32,
        advance: Vector2,
        size: Size,
        offset: Vector2,
        bitmap: Vec<u8>,
    ) -> Self {
        Self {
            encoding,
            advance,
            size,
            offset,
            bitmap,
        }
    }

    pub fn encoding(&self) -> i32 {
        self.encoding
    }

    pub fn advance(&self) -> &Vector2 {
        &self.advance
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn offset(&self) -> &Vector2 {
        &self.offset
    }

    pub fn bitmap(&self) -> &[u8] {
        &self.bitmap
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> bool {
        if x >= self.size.width || y >= self.size.height {
            return false;
        }

        let row_bytes = ((self.size.width + 7) / 8) as usize;
        let byte_index = (y as usize) * row_bytes + (x as usize / 8);
        let bit_index = 7 - (x % 8) as u8;

        let byte = self.bitmap.get(byte_index).copied().unwrap_or(0);

        (byte >> bit_index) & 1 == 1
    }
}

#[derive(Debug)]
pub struct FontBDF {
    name: String,                   // FONT
    bounds_size: Size,              // BBX
    bounds_offset: Vector2,         // BBOFF
    glyphs: HashMap<i32, GlyphBDF>, // ENCODING -> GlyphBDF
}

impl FontBDF {
    pub fn new(
        name: String,
        bounds_size: Size,
        bounds_offset: Vector2,
        count_glyphs: usize,
        glyphs: HashMap<i32, GlyphBDF>,
    ) -> Result<Self, BdfError> {
        if glyphs.len() != count_glyphs {
            return Err(BdfError::integrity_in(
                None,
                format!("font '{}'", name),
                format!(
                    "Expected {} glyphs according to 'CHARS' keyword, but found {}.",
                    count_glyphs,
                    glyphs.len()
                ),
            ));
        }

        Ok(Self {
            name,
            bounds_size,
            bounds_offset,
            glyphs,
        })
    }

    pub fn load(path: &str) -> Result<Self, BdfError> {
        let source = fs::read_to_string(path)?;
        parse(&source)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line_height(&self) -> u32 {
        self.bounds_size.height
    }

    pub fn get_glyph(&self, encoding: i32) -> Option<&GlyphBDF> {
        self.glyphs.get(&encoding)
    }
}
