use std::fmt::{Display, Formatter};
use crate::hardware::error::HardwareError;

pub type Result<T> = std::result::Result<T, InstructionError>;

#[derive(Debug)]
pub enum InstructionError {
    UnknownInstruction { instr: u8, cb: bool},
    HwError(HardwareError),
}

impl std::error::Error for InstructionError {}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Instruction Error: {:?}", self)
    }
}

impl From<HardwareError> for InstructionError {
    fn from(err: HardwareError) -> Self {
        InstructionError::HwError(err)
    }
}
