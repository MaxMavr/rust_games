mod parser;
mod builders;


use crate::graphics::vectors::Vector2;
use crate::graphics::size::Size;
use crate::graphics::fonts::bdf::parser::{ParserStage, expect_str, split_line,
    expect_hex_line, expect_n_ints};
use crate::graphics::fonts::bdf::builders::{FontBuilderBDF, GlyphBuilderBDF};
use crate::vector2;
use crate::size;

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
    pub fn new(encoding: i32, advance: Vector2, size: Size, offset: Vector2, bitmap: Vec<u8>) -> Self {
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
}


#[derive(Debug)]
pub struct FontBDF {
    name: String,                     // FONT
    bounds_size: Size,                // BBX
    bounds_offset: Vector2,           // BBOFF
    count_glyphs: usize,              // CHARS
    glyphs: HashMap<i32, GlyphBDF>,   // ENCODING -> GlyphBDF
}


impl FontBDF {
    pub fn new(name: String, bounds_size: Size, bounds_offset: Vector2, count_glyphs: usize, glyphs: HashMap<i32, GlyphBDF>) -> Result<Self, String> {
        if glyphs.len() != count_glyphs {
            return Err(format!(
                "BDF Integrity Error: Expected {} glyphs according to 'CHARS' keyword, but found {}.",
                count_glyphs,
                glyphs.len()
            ));
        }
        
        Ok(Self {
            name,
            bounds_size,
            bounds_offset,
            count_glyphs,
            glyphs,
        })
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let source = fs::read_to_string(path)
            .map_err(
                |e|
                format!("Failed to load BDF font file '{}': {}", path, e)
            )?;
        
        Self::parse(&source)
    }

    fn parse(data: &str) -> Result<Self, String> {        
        let mut font_builder = FontBuilderBDF::new();
        let mut stage = ParserStage::None;

        for (line_number, line) in data.lines().enumerate() {
            let line_number = line_number + 1;
            let (keyword, values) = split_line(line);
            
            println!("{:?} | {} : {:?}", stage, keyword, values);
            
            match keyword {
                "STARTFONT" => {
                    stage = ParserStage::Font;
                    continue;
                },
                "STARTCHAR" => {
                    stage = ParserStage::Glyph {
                        name: values.first().unwrap_or(&"unknown").to_string(),
                        start_line: line_number,
                        builder: GlyphBuilderBDF::new(),
                        in_bitmap: false,
                    };
                    continue;
                },
                "ENDFONT" => break,
                _ => {}
            }

            match stage.take() {
                ParserStage::Font => {
                    match keyword {
                        "FONT" => {
                            font_builder.set_font_name(expect_str(line_number, keyword, &values)?);
                        },
                        "FONTBOUNDINGBOX" => {
                            let [bbw, bbh, x_off, y_off] = expect_n_ints::<4>(line_number, keyword, &values)?;
                            font_builder.set_bounds_size(size!(bbw as u32, bbh as u32));
                            font_builder.set_bounds_offset(vector2!(x_off, y_off));
                        },
                        "CHARS" => {
                            font_builder.set_count_glyphs(expect_n_ints::<1>(line_number, keyword, &values)?[0] as usize);
                        },
                        "DWIDTH" => {
                            let [x, y] = expect_n_ints::<2>(line_number, keyword, &values)?;
                            font_builder.set_default_advance(vector2!(x, y));
                        },
                        _ => {},
                    }

                    stage = ParserStage::Font;
                },

                ParserStage::Glyph { name, start_line, mut builder, mut in_bitmap } => {
                    match keyword {
                        "ENCODING" => {
                            builder.set_encoding(expect_n_ints::<1>(line_number, keyword, &values)?[0]);
                        },
                        "BBX" => {
                            let [bbw, bbh, x_off, y_off] = expect_n_ints::<4>(line_number, keyword, &values)?;
                            builder.set_size(size!(bbw as u32, bbh as u32));
                            builder.set_offset(vector2!(x_off, y_off));
                        },
                        "DWIDTH" => {
                            let [x, y] = expect_n_ints::<2>(line_number, keyword, &values)?;
                            builder.set_advance(vector2!(x, y));
                        },
                        "BITMAP" => {
                            in_bitmap = true;
                            stage = ParserStage::Glyph { name, start_line, builder, in_bitmap };
                            continue;
                        },
                        "ENDCHAR" => {
                            let glyph = builder.build(
                                font_builder.default_advance(),
                                &name,
                                start_line,
                                line_number,
                            )?;

                            font_builder.insert_glyph(glyph);

                            stage = ParserStage::None;
                            continue;
                        },
                        _ => {},
                    }

                    if in_bitmap {
                        let bitmap_bytes = expect_hex_line(line_number, line)?;
                        print!("{bitmap_bytes:?}");
                        builder.extend_bitmap(bitmap_bytes);
                    }

                    stage = ParserStage::Glyph { name, start_line, builder, in_bitmap };
                },
                _ => {},
            }
        }
        
        font_builder.build()
    }


    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn count_glyphs(&self) -> usize {
        self.count_glyphs
    }

    pub fn get_glyph(&self, encoding: i32) -> Option<&GlyphBDF> {
        self.glyphs.get(&encoding)
    }
}