use std::ptr::replace;
use regex::Regex;
use crate::util::error::HexToolError;
use colored::Colorize;

pub(crate) fn convert(input: &str, numeric: bool, split_byte: bool) -> String {
    let mut valid_input = String::new();
    match validate_hex(input) {
        Ok(str) => { valid_input = str; }
        Err(e) => return e.to_string()
    }

    let result;
    if numeric {
        result = un_hex_numeric(&valid_input);
    } else {
        result = un_hex_string(&valid_input);
    }

    match result {
        Ok(str) => {
            if !split_byte { return format!("{}", str); }
            return str.chars().map(|c| format!("{}", c))
                .collect::<Vec<String>>().join(" ");
        }
        Err(e) => { e.to_string() }
    }
}

fn un_hex_numeric(input: &str) -> Result<String, HexToolError> {
    let parsed = i64::from_str_radix(input, 16);
    match parsed {
        Ok(i) => Ok(format!("{}", i)),
        Err(e) => Err(HexToolError {
            message: format!("Input is not valid for 'unhex' with numeric flag (-n). {}", e)
        })
    }
}

fn un_hex_string(input: &str) -> Result<String, HexToolError> {
    // Go through each hex byte and convert to a string
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let byte = &input[i..i + 2];
        let parsed = u8::from_str_radix(byte, 16);
        match parsed {
            Ok(i) => result.push(i as char),
            Err(e) => return Err(HexToolError {
                message: format!("Could not parse {}", e)
            })
        }
        i += 2;
    }
    Ok(result)
}

fn validate_hex(input: &str) -> Result<String, HexToolError> {
    let re = Regex::new("0[x|X]").unwrap();
    let cleaned = re.replace_all(input, "");

    let mut is_valid = true;
    let mut proc_input = String::new();
    for c in cleaned.chars() {
        if !c.is_digit(16) {
            if is_valid { is_valid = false };
            proc_input.push_str(&format!("\x1b[31m{}\x1b[0m", c));
            continue;
        }
        proc_input.push(c);
    }
    if !is_valid {
        return Err(HexToolError {
            message: format!("The highlighted chars can't be converted:\n{}.", proc_input)
        });
    }

    if proc_input.len() % 2 != 0 {
        proc_input = format!("0{}", proc_input)
    };
    Ok(proc_input.to_string())
}