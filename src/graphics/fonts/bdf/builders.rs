use std::collections::HashMap;
use crate::graphics::fonts::bdf::{GlyphBDF, FontBDF};
use crate::graphics::fonts::bdf::errors::BdfError;
use crate::graphics::vectors::Vector2;
use crate::graphics::size::Size;

#[derive(Debug, Clone)]
pub struct FontBuilderBDF {
    font_name: Option<String>,
    count_glyphs: Option<usize>,
    default_advance: Option<Vector2>,
    bounds_size: Option<Size>,
    bounds_offset: Option<Vector2>,
    glyphs: HashMap<i32, GlyphBDF>,
}

impl FontBuilderBDF {
    pub fn new() -> Self { 
        Self {
            font_name: None,
            count_glyphs: None,
            default_advance: None,
            bounds_size: None,
            bounds_offset: None,
            glyphs: HashMap::new(),
        }
    }
    
    pub fn set_font_name(&mut self, name: String) {
        self.font_name = Some(name);
    }

    pub fn set_count_glyphs(&mut self, count: usize) {
        self.count_glyphs = Some(count);
    }

    pub fn set_default_advance(&mut self, advance: Vector2) {
        self.default_advance = Some(advance);
    }

    pub fn default_advance(&self) -> Option<Vector2> {
        self.default_advance
    }

    pub fn set_bounds_size(&mut self, size: Size) {
        self.bounds_size = Some(size);
    }

    pub fn set_bounds_offset(&mut self, offset: Vector2) {
        self.bounds_offset = Some(offset);
    }

    pub fn insert_glyph(&mut self, glyph: GlyphBDF) {
        self.glyphs.insert(glyph.encoding(), glyph);
    }

    fn require<T>(name: &str, value: Option<T>) -> Result<T, BdfError> {
        value.ok_or_else(|| 
            return BdfError::syntax(
                None,
                format!("Required font keyword '{}' is missing.", name)
            )
        )
    }

    pub fn build(self) -> Result<FontBDF, BdfError> { 
        FontBDF::new(
            Self::require::<String>("FONT", self.font_name)?,
            Self::require::<Size>("FONTBOUNDINGBOX", self.bounds_size)?,
            Self::require::<Vector2>("FONTBOUNDINGBOX", self.bounds_offset)?,
            Self::require::<usize>("CHARS", self.count_glyphs)?,
            self.glyphs,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphBuilderBDF {
    encoding: Option<i32>,
    advance: Option<Vector2>,
    size: Option<Size>,
    offset: Option<Vector2>,
    bitmap: Vec<u8>,
}

impl GlyphBuilderBDF {
    pub fn new() -> Self {
        Self {
            encoding: None,
            advance: None,
            size: None,
            offset: None,
            bitmap: Vec::new(),
        }
    }

    pub fn set_encoding(&mut self, encoding: i32) {
        self.encoding = Some(encoding);
    }

    pub fn set_advance(&mut self, advance: Vector2) {
        self.advance = Some(advance);
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = Some(size);
    }

    pub fn set_offset(&mut self, offset: Vector2) {
        self.offset = Some(offset);
    }

    pub fn extend_bitmap(&mut self, bitmap: Vec<u8>) {
        self.bitmap.extend(bitmap);
    }

    fn require<T>(name: &str, start_line: usize, field_name: &str, field_value: Option<T>) -> Result<T, BdfError> {
        field_value.ok_or_else(|| 
            BdfError::syntax_in(
                Some(start_line),
                format!("glyph '{}'", name),
                format!("Required glyph keyword '{}' is missing.", field_name)
            )
        )
    }

    pub fn build(self, default_advance: Option<Vector2>, name: &str, start_line: usize) -> Result<GlyphBDF, BdfError> {
        
        let final_advance = self.advance.or(default_advance)
            .ok_or_else(|| 
                BdfError::syntax_in(
                    Some(start_line),
                    format!("glyph '{}'", name),
                    "Glyph is missing DWIDTH, and no font DWIDTH is defined."
                    )
                )
            ?;

        let size = Self::require::<Size>(name, start_line, "BBX", self.size)?;
        let expected_len = ((size.width + 7) / 8) * size.height;

        if self.bitmap.len() != expected_len as usize {
            return Err(BdfError::integrity_in(
                Some(start_line),
                format!("glyph '{}'", name),
                format!(
                    "Bitmap size mismatch. Expected {} bytes for BBX {}x{}, but got {} bytes.",
                    expected_len, size.width, size.height, self.bitmap.len()
                )
            ));
        }
        
        Ok(GlyphBDF::new(
                Self::require::<i32>(name, start_line, "ENCODING", self.encoding)?,
                final_advance,
                size,
                Self::require::<Vector2>(name, start_line, "BBX", self.offset)?,
                self.bitmap,
            ))
    }
}