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

    // Memory functions
    pub fn push16(&mut self, val: u16) -> Result<()> {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        self.memory.write16(sp.wrapping_sub(2), val);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_sub(2));
        Ok(())
    }

    pub fn pop16(&mut self) -> Result<u16> {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        let result = self.memory.read16(sp);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_add(2));
        Ok(result)
    }

    // Debug functions
    pub fn dump_state(&self) {
        println!("Registers");
        println!("=================");
        println!("{}", self.registers);
    }
}
