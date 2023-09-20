use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct HexToolError {
    pub(crate) message: String,
}

impl Debug for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for HexToolError {}