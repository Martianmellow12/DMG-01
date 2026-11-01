use super::error::Result;

use crate::hardware::cpu::Cpu;
use crate::hardware::cpu::registers::{Reg8, Reg16, RegF};

#[derive(Debug)]
pub enum OpReg8 {
    Reg(Reg8),
    IndirectHL,
}

impl OpReg8 {
    pub fn read(&self, cpu: &Cpu) -> u8 {
        match self {
            Self::Reg(reg) => cpu.registers.read_reg8(*reg),
            Self::IndirectHL => {
                let addr = cpu.registers.read_reg16(Reg16::HL);
                cpu.bus.read8(addr)
            },
        }
    }

    pub fn write(&self, cpu: &mut Cpu, val: u8) {
        match self {
            Self::Reg(reg) => cpu.registers.write_reg8(*reg, val),
            Self::IndirectHL => {
                let addr = cpu.registers.read_reg16(Reg16::HL);
                cpu.bus.write8(addr, val);
            },
        }
    }

    pub fn is_indirect(&self) -> bool {
        match self {
            Self::IndirectHL => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum OpReg16 {
    RegBC,
    RegDE,
    RegHL,
    RegSP,
}

impl OpReg16 {
    pub fn read(&self, cpu: &Cpu) -> u16 {
        cpu.registers.read_reg16(self.into())
    }

    pub fn write(&self, cpu: &mut Cpu, val: u16) {
        cpu.registers.write_reg16(self.into(), val);
    }
}

#[derive(Debug)]
pub enum OpReg16Stk {
    RegBC,
    RegDE,
    RegHL,
    RegAF,
}

impl OpReg16Stk {
    pub fn read(&self, cpu: &Cpu) -> u16 {
        cpu.registers.read_reg16(self.into())
    }

    pub fn write(&self, cpu: &mut Cpu, val: u16) {
        cpu.registers.write_reg16(self.into(), val);
    }
}

#[derive(Debug)]
pub enum OpReg16Mem {
    RegBC,
    RegDE,
    RegIncHL,
    RegDecHL,
}

impl OpReg16Mem {
    pub fn read_and_step(&self, cpu: &mut Cpu) -> u8 {
        let addr = self.to_addr(cpu);
        cpu.bus.read8(addr)
    }

    pub fn write_and_step(&self, cpu: &mut Cpu, val: u8) {
        let addr = self.to_addr(cpu);
        cpu.bus.write8(addr, val);
    }
    
    fn to_addr(&self, cpu: &mut Cpu) -> u16 {
        let reg = match self {
            Self::RegBC => Reg16::BC,
            Self::RegDE => Reg16::DE,
            Self::RegIncHL | Self::RegDecHL => Reg16::HL,
        };
        let addr = cpu.registers.read_reg16(reg);
        
        // Update HL after it's read
        match self {
            Self::RegIncHL => cpu.registers.write_reg16(Reg16::HL, addr.wrapping_add(1)),
            Self::RegDecHL => cpu.registers.write_reg16(Reg16::HL, addr.wrapping_sub(1)),
            _ => {},
        };

        addr
    }
}

#[derive(Debug)]
pub enum OpCond {
    CondNZ,
    CondZ,
    CondNC,
    CondC,
}

impl OpCond {
    pub fn check(&self, cpu: &Cpu) -> bool {
        match self {
            Self::CondNZ => cpu.registers.read_regf(RegF::Fz) == false,
            Self::CondZ => cpu.registers.read_regf(RegF::Fz) == true,
            Self::CondNC => cpu.registers.read_regf(RegF::Fc) == false,
            Self::CondC => cpu.registers.read_regf(RegF::Fc) == true,
        }
    }
}

#[derive(Debug)]
pub struct OpBitIdx(pub u8);

impl OpBitIdx {
    pub fn mask(&self) -> u8 {
        1 << self.0
    }
}
