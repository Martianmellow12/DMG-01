pub mod registers;
pub mod memory;
pub mod error;

use super::instruction::Instruction;

use error::Result;
use registers::{RegisterFile};

pub struct Cpu {
    pub registers: RegisterFile,
    pub memory: memory::Memory,
}

impl Cpu {
    pub fn new(en_log: bool) -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: memory::Memory::new(en_log),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // TODO: debug code, fix this later
        let instr = Instruction::decode(self).unwrap();
        let pc = self.registers.read_reg16(registers::Reg16::PC);
        println!("PC=0x{:x?} - [{}] {:x?}", pc, instr.size(), instr);
        instr.execute(self)?;
        Ok(())
    }

    // Debug functions
    pub fn dump_state(&self) {
        println!("Registers");
        println!("=================");
        println!("{}", self.registers);
    }
}
