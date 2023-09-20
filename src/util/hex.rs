use crate::util::error::HexToolError;

pub fn convert(input: &str, numeric: bool, split_byte: bool) -> String {
    let result;
    if numeric {
        result = hex_numeric(input);
    } else {
        result = hex_string(input);
    }

    match result {
        Ok(str) => {
            if split_byte {
                let mut result = String::new();
                for (i, c) in str.chars().enumerate() {
                    if i % 2 == 0 && i != 0 {
                        result.push_str(" ");
                    }
                    result.push(c);
                }
                format!("{}", result)
            } else {
                format!("{}", str)
            }
        }
        Err(e) => e.to_string(),
    }
}

fn hex_string(input: &str) -> Result<String, HexToolError> {
    return Ok(input.as_bytes().iter().map(|b| format!("{:02x}", b)).collect());
}

fn hex_numeric(input: &str) -> Result<String, HexToolError> {
    let is_all_digit = input.chars().all(|c| c.is_digit(10));
    if !is_all_digit {
        return Err(HexToolError {
            message: String::from("Input is not valid for 'hex' with numeric flag (-n).")
        });
    }
    let to_str: i32 = input.parse().unwrap();
    Ok(format!("{:02x}", to_str))
}