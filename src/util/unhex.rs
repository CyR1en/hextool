use crate::util::error::HexToolError;

fn un_hex(input: &str, numeric: bool, split_byte: bool) {
    let result;
    if numeric {
        result = un_hex_numeric(input);
    } else {
        result = un_hex_string(input);
    }
}

fn un_hex_numeric(input: &str) -> Result<String, HexToolError> {
    todo!()
}

fn un_hex_string(input: &str) -> Result<String, HexToolError> {
    todo!()
}