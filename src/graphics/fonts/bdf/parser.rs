use crate::graphics::fonts::bdf::builders::GlyphBuilderBDF;
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

pub fn expect_str(line_number: usize, keyword: &str, values: &[&str]) -> Result<String, String> {
    if values.is_empty() {
        return Err(format!(
            "BDF Syntax Error at line {}: Keyword '{}' requires at least 1 argument, but found none.",
            line_number, keyword
        ));
    }

    Ok(values.join(" "))
}

fn expect_hex(line_number: usize, value: &str) -> Result<u8, String> {
    u8::from_str_radix(value, 16).map_err(|e| {
        format!(
            "BDF Parse Error at line {}: Failed to parse hex byte. Value: '{}'. Error: {}",
            line_number, value, e
        )
    })
}

pub fn expect_hex_line(line_number: usize, hex_line: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    
    for i in (0..hex_line.len()).step_by(2) {
        if i + 2 > hex_line.len() {
            return Err(format!(
                "BDF Parse Error at line {}: Incomplete hex byte. Line: '{}'",
                line_number, hex_line
            ));
        }

        let byte_str = &hex_line[i..i+2];
        let byte_val = expect_hex(line_number, byte_str)?;
        bytes.push(byte_val);
    }

    Ok(bytes)
}

fn expect_int(line_number: usize, keyword: &str, value: &str) -> Result<i32, String> {
    value.parse::<i32>().map_err(|e| {
        format!(
            "BDF Parse Error at line {}: Failed to parse integer in '{}'. Value: '{}'. Error: {}",
            line_number, keyword, value, e
        )
    })
}


fn expect_ints(line_number: usize, keyword: &str, values: &[&str], count: usize) -> Result<Vec<i32>, String> {
    if values.len() != count {
        return Err(format!(
            "BDF Syntax Error at line {}: Keyword '{}' requires exactly {} arguments, but found {}. Values: {:?}",
            line_number, keyword, count, values.len(), values
        ));
    }

    values.iter()
        .map(|v| expect_int(line_number, keyword, v))
        .collect()
}

pub fn expect_n_ints<const N: usize>(line_number: usize, keyword: &str, values: &[&str]) -> Result<[i32; N], String> {
    let ints = expect_ints(line_number, keyword, values, N)?;
    Ok(ints.try_into().unwrap())
}