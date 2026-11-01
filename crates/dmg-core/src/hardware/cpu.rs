pub mod registers;

use crate::software::instruction::Instruction;

use super::error::Result;
use super::bus::Bus;
use super::mbc::Mbc;
use registers::{RegisterFile};

pub struct Cpu {
    pub registers: RegisterFile,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(en_log: bool) -> Self {
        Self {
            registers: RegisterFile::new(),
            bus: Bus::new(4, 1),
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
    pub fn push16(&mut self, val: u16) {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        self.bus.write16(sp.wrapping_sub(2), val);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_sub(2));
    }

    pub fn pop16(&mut self) -> u16 {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        let result = self.bus.read16(sp);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_add(2));
        result
    }

    // Debug functions
    pub fn dump_state(&self) {
        println!("Registers");
        println!("=================");
        println!("{}", self.registers);
    }
}
