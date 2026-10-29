use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, HardwareError>;

#[derive(Debug)]
pub enum HardwareError {
    OutOfBoundsAccess(&'static str),
    ConversionError(&'static str),
    IoError(std::io::Error),
}

impl std::error::Error for HardwareError {}

impl Display for HardwareError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Hardware Error: {:?}", self)
    }
}

impl From<std::io::Error> for HardwareError {
    fn from(err: std::io::Error) -> Self {
        HardwareError::IoError(err)
    }
}
