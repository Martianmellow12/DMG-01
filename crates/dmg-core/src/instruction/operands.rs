use crate::hardware::Cpu;
use crate::hardware::registers::{Reg8, Reg16, RegF};

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
                cpu.memory.read8(addr)
            },
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

#[derive(Debug)]
pub enum OpReg16Stk {
    RegBC,
    RegDE,
    RegHL,
    RegAF,
}

#[derive(Debug)]
pub enum OpReg16Mem {
    RegBC,
    RegDE,
    RegIncHL,
    RegDecHL,
}

impl OpReg16Mem {
    pub fn read_addr(&self, cpu: &mut Cpu) -> u16 {
        let reg = match self {
            Self::RegBC => Reg16::BC,
            Self::RegDE => Reg16::DE,
            Self::RegIncHL | Self::RegDecHL => Reg16::HL,
        };
        let addr = cpu.registers.read_reg16(reg);
        
        // Update HL after it's read
        match self {
            Self::RegIncHL => cpu.registers.write_reg16(Reg16::HL, addr + 1),
            Self::RegDecHL => cpu.registers.write_reg16(Reg16::HL, addr - 1),
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

#[derive(Debug)]
pub struct OpBitIdx(pub u8);
