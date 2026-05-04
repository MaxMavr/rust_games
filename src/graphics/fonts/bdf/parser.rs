use crate::graphics::fonts::bdf::FontBDF;
use crate::graphics::fonts::bdf::builders::{FontBuilderBDF, GlyphBuilderBDF};
use crate::graphics::fonts::bdf::errors::BdfError;
use crate::graphics::fonts::bdf::extractors::{
    expect_hex_line, expect_n_ints, expect_str, split_line,
};

use crate::size;
use crate::vector2;

enum ParserStage {
    Idle,
    Font,
    Glyph(GlyphState),
    End,
}

struct GlyphState {
    name: String,
    start_line: usize,
    in_bitmap: bool,
    builder: GlyphBuilderBDF,
}

pub fn parse(data: &str) -> Result<FontBDF, BdfError> {
    let mut font = FontBuilderBDF::new();
    let mut state = ParserStage::Idle;

    for (i, line) in data.lines().enumerate() {
        let line_number = i + 1;
        let (keyword, values) = split_line(line);

        state = match state {
            ParserStage::Idle => parse_idle(line_number, keyword, &values),
            ParserStage::Font => parse_font(line_number, keyword, &values, &mut font)?,
            ParserStage::Glyph(glyph) => {
                parse_glyph(line_number, line, keyword, &values, glyph, &mut font)?
            }
            ParserStage::End => break,
        };
    }

    match state {
        ParserStage::Idle => {
            return Err(BdfError::syntax(
                None,
                "Required keyword 'STARTFONT' is missing.",
            ));
        }
        ParserStage::Font => {
            return Err(BdfError::syntax(
                None,
                "Required keyword 'ENDFONT' is missing.",
            ));
        }
        ParserStage::Glyph(_) => {
            return Err(BdfError::syntax(
                None,
                "Required keyword 'ENDCHAR' is missing.",
            ));
        }
        ParserStage::End => font.build(),
    }
}

fn parse_idle(line: usize, keyword: &str, values: &[&str]) -> ParserStage {
    match keyword {
        "STARTFONT" => ParserStage::Font,

        "STARTCHAR" => ParserStage::Glyph(GlyphState {
            name: values.first().unwrap_or(&"unknown").to_string(),
            start_line: line,
            in_bitmap: false,
            builder: GlyphBuilderBDF::new(),
        }),

        _ => ParserStage::Idle,
    }
}

fn parse_font(
    line: usize,
    keyword: &str,
    values: &[&str],
    font: &mut FontBuilderBDF,
) -> Result<ParserStage, BdfError> {
    match keyword {
        "FONT" => {
            font.set_font_name(expect_str(line, keyword, values)?);
        }

        "FONTBOUNDINGBOX" => {
            let [w, h, x, y] = expect_n_ints::<4>(line, keyword, values)?;
            font.set_bounds_size(size!(w as u32, h as u32));
            font.set_bounds_offset(vector2!(x, y));
        }

        "CHARS" => {
            let [count] = expect_n_ints::<1>(line, keyword, values)?;
            font.set_count_glyphs(count as usize);
        }

        "DWIDTH" => {
            let [x, y] = expect_n_ints::<2>(line, keyword, values)?;
            font.set_default_advance(vector2!(x, y));
        }

        "STARTCHAR" => {
            return Ok(ParserStage::Glyph(GlyphState {
                name: values.first().unwrap_or(&"unknown").to_string(),
                start_line: line,
                in_bitmap: false,
                builder: GlyphBuilderBDF::new(),
            }));
        }

        "ENDFONT" => return Ok(ParserStage::End),

        _ => {}
    }

    Ok(ParserStage::Font)
}

fn parse_glyph(
    line: usize,
    raw: &str,
    keyword: &str,
    values: &[&str],
    mut glyph: GlyphState,
    font: &mut FontBuilderBDF,
) -> Result<ParserStage, BdfError> {
    match keyword {
        "ENCODING" => {
            let [code] = expect_n_ints::<1>(line, keyword, values)?;
            glyph.builder.set_encoding(code);
        }

        "BBX" => {
            let [w, h, x, y] = expect_n_ints::<4>(line, keyword, values)?;
            glyph.builder.set_size(size!(w as u32, h as u32));
            glyph.builder.set_offset(vector2!(x, y));
        }

        "DWIDTH" => {
            let [x, y] = expect_n_ints::<2>(line, keyword, values)?;
            glyph.builder.set_advance(vector2!(x, y));
        }

        "BITMAP" => {
            glyph.in_bitmap = true;
        }

        "ENDCHAR" => {
            let built =
                glyph
                    .builder
                    .build(font.default_advance(), &glyph.name, glyph.start_line)?;

            font.insert_glyph(built);
            return Ok(ParserStage::Font);
        }

        _ if glyph.in_bitmap => {
            let bytes = expect_hex_line(line, raw)?;
            glyph.builder.extend_bitmap(bytes);
        }

        _ => {}
    }

    Ok(ParserStage::Glyph(glyph))
}
