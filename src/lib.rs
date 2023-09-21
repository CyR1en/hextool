//! This crate provides functions that allows you to easily convert strings from and to hex.
//!
//!
//! ## Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/hextool) and can be added to your project.
//! ```toml
//! [dependencies]
//! hextool = { version = "version", default-features = false }
//! ```
//! ## Examples
//!
//! ### Example: converting string to hex
//!
//! To convert a string to hex, use the `Hex` and `Convert` trait:
//!
//! ```
//! use hextool::{Hex, Convert};
//!
//! // Convert a string to hex
//! let hex = Hex::convert("hello", false, false);
//! println!("hello in hex: {}", hex); // #=> "hello in hex: 68656c6c6f"
//!
//! // You can also split the output in bytes
//! let hex = Hex::convert("hello", false, true);
//! println!("hello in hex: {}", hex); // #=> "hello in hex: 68 65 6c 6c 6f"
//!
//! // Convert a string with numeric flag. This will take the numerical value of the string.
//! // If the string is not a number, it will return an error.
//! let hex = Hex::convert("255", true, false);
//! println!("255 in hex: {}", hex); // #=> "255 in hex: {ff}"
//! ```
//! ### Example: converting hex to string
//!
//! To convert a hex string to a string, use the `UnHex` and `Convert` trait:
//!
//! ```
//! use hextool::{UnHex, Convert};
//!
//! // Convert a hex string to a string
//! let unhex = UnHex::convert("68656c6c6f", false, false);
//! println!("68656c6c6f in string: {}", unhex); // #=> "68656c6c6f in string: hello"
//!
//! // Convert a hex string to a string with numeric flag. This will take the numerical value of the string.
//! let unhex = UnHex::convert("cafebabe", true, false);
//! println!("cafebabe in string: {}", unhex); // #=> "cafebabe in string: 3405691582"
//!
//! // If numeric is set to false, only valid strings [0-9a-f] is accepted. If the string is not valid,
//! // it will return the string with the invalid string highlighted to red.
//! let unhex = UnHex::convert("aga", true, false);
//! println!("{}", unhex); // #=> "The highlighted chars can't be converted:\nag\u{1b}[31ma\u{1b}[0m."
//!```

#![doc(html_root_url = "https://docs.rs/hextool/0.1.1")]
#![deny(missing_docs)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use regex::Regex;

/// Error type for hextool
pub struct HexToolError {
    /// Error message
    pub(crate) message: String,
}

/// Implement Debug, Display and Error for HexToolError
impl Debug for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implement Debug, Display and Error for HexToolError
impl Display for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implement Debug, Display and Error for HexToolError
impl Error for HexToolError {}


/// Convert trait
pub trait Convert {
    /// Convert input to hex or string depending on the implementation
    ///
    /// ## Arguments
    /// * `input` - The input to convert
    /// * `numeric` - If set to true, the input will be considered as a numeric value.
    /// * `split` - If set to true, the output will be split in bytes
    /// ## Returns
    /// A string with the converted input
    fn convert(input: &str, numeric: bool, split: bool) -> String;
}

/// Hex struct
///
/// This struct implements the Convert trait and can be used to convert a string to hex.
///
/// ## Example
/// ```
/// use hextool::{Hex, Convert};
///
/// // Convert a string to hex
/// let hex = Hex::convert("hello", false, false);
/// println!("hello in hex: {}", hex); // #=> "hello in hex: 68656c6c6f"
/// ```
pub struct Hex;

impl Hex {
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
        let to_str: i64 = input.parse().unwrap();
        Ok(format!("{:02x}", to_str))
    }
}

impl Convert for Hex {
    fn convert(input: &str, numeric: bool, split_byte: bool) -> String {
        let result;
        if numeric {
            result = Hex::hex_numeric(input);
        } else {
            result = Hex::hex_string(input);
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
}

/// UnHex struct
///
/// This struct implements the Convert trait and can be used to convert a hex string to a string.
///
/// ## Example
/// ```
/// use hextool::{UnHex, Convert};
///
/// // Convert a hex string to a string
/// let unhex = UnHex::convert("68656c6c6f", false, false);
/// println!("68656c6c6f in string: {}", unhex); // #=> "68656c6c6f in string: hello"
/// ```
pub struct UnHex;

impl UnHex {
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
}

impl Convert for UnHex {
    fn convert(input: &str, numeric: bool, split_byte: bool) -> String {
        let valid_input;
        match UnHex::validate_hex(input) {
            Ok(str) => { valid_input = str; }
            Err(e) => return e.to_string()
        }

        let result;
        if numeric {
            result = UnHex::un_hex_numeric(&valid_input);
        } else {
            result = UnHex::un_hex_string(&valid_input);
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
}