use crate::graphics::fonts::bdf::errors::BdfError;

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

pub fn expect_hex_line(line: usize, hex_line: &str) -> Result<Vec<u8>, BdfError> {
    if hex_line.len() % 2 != 0 {
        return Err(BdfError::parse(
            Some(line),
            "BITMAP",
            hex_line,
            "Hex string length must be even."
        ));
    }
    
    let mut bytes = Vec::with_capacity(hex_line.len() / 2);
    
    for i in (0..hex_line.len()).step_by(2) {
        let byte_str = &hex_line[i..i + 2];
        let byte_val = u8::from_str_radix(byte_str, 16).map_err(|e| {
            BdfError::parse(
                Some(line),
                "BITMAP",
                byte_str,
                format!("Invalid hex byte: {}", e)
            )
        })?;
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
    ints.try_into().map_err(|_| {
        BdfError::syntax_in(
            Some(line),
            format!("keyword '{}'", keyword),
            format!("Expected exactly {} integers, but conversion failed.", N)
        )
    })
}