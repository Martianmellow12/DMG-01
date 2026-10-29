pub mod registers;
pub mod memory;
pub mod error;

use super::instruction::Instruction;

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

    pub fn step(&mut self) -> () {
        // TODO: debug code, fix this later
        let instr = Instruction::decode(self).unwrap();
        let mut pc = self.registers.read_reg16(registers::Reg16::PC);
        println!("PC=0x{:x?} - [{}] {:x?}", pc, instr.size(), instr);
        instr.execute(self);
    }

    // Debug functions
    pub fn dump_state(&self) {
        println!("Registers");
        println!("=================");
        println!("{}", self.registers);
        println!()
    }
}
