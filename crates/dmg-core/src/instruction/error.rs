use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum InstructionError {
    UnknownInstruction { instr: u8, cb: bool},
}

impl std::error::Error for InstructionError {}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Instruction Error: {:?}", self)
    }
}
