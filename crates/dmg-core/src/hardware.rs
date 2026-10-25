mod registers;
mod memory;
mod error;

use error::HardwareError;
use registers::{RegisterFile};

pub struct Cpu {
    pub registers: RegisterFile,
    pub memory: memory::Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: memory::Memory::new(),
        }
    }
}
