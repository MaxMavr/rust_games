use crate::graphics::fonts::bdf::builders::GlyphBuilderBDF;
use crate::graphics::fonts::bdf::errors::BdfError;


#[derive(Debug, Clone)]
pub enum ParserStage {
    None,
    Font,
    Glyph {
        name: String,
        start_line: usize,
        builder: GlyphBuilderBDF,
        in_bitmap: bool,
    }
}

impl ParserStage {
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, ParserStage::None)
    }
}

pub fn split_line(line: &str) -> (&str, Vec<&str>) {
    let mut parts = line.split_whitespace();
    let keyword = parts.next().unwrap_or("");
    let values = parts.collect();
    (keyword, values)
}

pub fn expect_str(line: usize, keyword: &str, values: &[&str]) -> Result<String, BdfError> {
    
    if values.is_empty() {
        return Err(BdfError::parse(
            Some(line),
            keyword,
            "<missing>",
            "requires at least 1 argument, but found none"
        ));
    }

    if values.len() == 1 {
        Ok(values[0].to_string())
    } else {
        Ok(values.join(" "))
    }
}

fn expect_hex(line: usize, value: &str) -> Result<u8, BdfError> {
    u8::from_str_radix(value, 16).map_err(|e| {
        BdfError::parse(
            Some(line),
            "",
            value,
            e.to_string()
        )
    })
}

pub fn expect_hex_line(line: usize, hex_line: &str) -> Result<Vec<u8>, BdfError> {
    let mut bytes = Vec::new();
    
    for i in (0..hex_line.len()).step_by(2) {
        if i + 2 > hex_line.len() {
            return Err(BdfError::parse(
                Some(line),
                "",
                hex_line,
                "incomplete hex byte."
            ));
        }

        let byte_str = &hex_line[i..i+2];
        let byte_val = expect_hex(line, byte_str)?;
        bytes.push(byte_val);
    }

    Ok(bytes)
}

fn expect_int(line: usize, keyword: &str, value: &str) -> Result<i32, BdfError> {
    value.parse::<i32>().map_err(|e| {
        return BdfError::parse(
            Some(line),
            keyword,
            value,
            e.to_string()
        )
    })
}


fn expect_ints(line: usize, keyword: &str, values: &[&str], count: usize) -> Result<Vec<i32>, BdfError> {
    if values.len() != count {
        return Err(BdfError::syntax_in(
            Some(line),
            format!("keyword '{}'", keyword),
            format!(
                "requires exactly {} arguments, but found {}. Values: {:?}",
                count, values.len(), values
            )
        ));
    }

    values.iter()
        .map(|v| expect_int(line, keyword, v))
        .collect()
}

pub fn expect_n_ints<const N: usize>(line: usize, keyword: &str, values: &[&str]) -> Result<[i32; N], BdfError> {
    let ints = expect_ints(line, keyword, values, N)?;
    Ok(ints.try_into().unwrap())
}