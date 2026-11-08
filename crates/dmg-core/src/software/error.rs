use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//
// Instruction
//

#[derive(Debug)]
pub enum InstructionError {
    UnknownInstruction { instr: u8, cb: bool},
    ExecNotImplemented, // TODO: Debug for development, remove
}

impl std::error::Error for InstructionError {}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Instruction Error: {:?}", self)
    }
}

//
// Game ROM
// 

#[derive(Debug)]
pub enum GameRomError {
    // General ROM errors
    BadHeaderSize(usize),

    // Header decode errors
    BadCartridgeTypeByte(u8),
    BadRomSizeByte(u8),
    BadRamSizeByte(u8),
}

impl std::error::Error for GameRomError {}

impl Display for GameRomError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Game ROM error: {:?}", self)
    }
}
