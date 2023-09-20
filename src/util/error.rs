use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct HexToolError {
    pub(crate) message: String,
}

impl Debug for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for HexToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HexToolError {}