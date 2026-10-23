mod registers;

use registers::{Register, RegisterFile};

struct Cpu {
    registers: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {}

    /// Register accessors
    pub fn read_reg8(reg: registers::Register) -> u8 {
        match reg {}
    }
}
