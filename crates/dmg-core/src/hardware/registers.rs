use crate::instruction::operands;

use super::error::{HardwareError, Result};

#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl TryFrom<&operands::OpReg8> for Reg8 {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &operands::OpReg8) -> Result<Self> {
        match value {
            operands::OpReg8::Reg(reg) => Ok(*reg),
            operands::OpReg8::IndirectHL => {
                Err(HardwareError::ConversionError(
                    "Cannot convert an [hl] R8 to a Reg8"
                ).into())
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl From<&operands::OpReg16> for Reg16 {
    fn from(value: &operands::OpReg16) -> Self {
        match value {
            operands::OpReg16::RegBC => Self::BC,
            operands::OpReg16::RegDE => Self::DE,
            operands::OpReg16::RegHL => Self::HL,
            operands::OpReg16::RegSP => Self::SP,
        }
    }
}

impl From<&operands::OpReg16Stk> for Reg16 {
    fn from(value: &operands::OpReg16Stk) -> Self {
        match value {
            operands::OpReg16Stk::RegBC => Self::BC,
            operands::OpReg16Stk::RegDE => Self::DE,
            operands::OpReg16Stk::RegHL => Self::HL,
            operands::OpReg16Stk::RegAF => Self::AF,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RegF {
    Fz,
    Fn,
    Fh,
    Fc,
}

#[derive(Debug)]
pub struct RegisterFile {
    // System registers
    reg_sp: u16,
    reg_pc: u16,

    // General purpose registers
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    // Flag registers
    reg_fz: bool,
    reg_fn: bool,
    reg_fh: bool,
    reg_fc: bool,
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            reg_sp: 0,
            reg_pc: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_fz: false,
            reg_fn: false,
            reg_fh: false,
            reg_fc: false,
        }
    }

    /// Register accessors
    pub fn read_reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.reg_a,
            Reg8::B => self.reg_b,
            Reg8::C => self.reg_c,
            Reg8::D => self.reg_d,
            Reg8::E => self.reg_e,
            Reg8::H => self.reg_h,
            Reg8::L => self.reg_l,
        }
    }

    pub fn write_reg8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.reg_a = val,
            Reg8::B => self.reg_b = val,
            Reg8::C => self.reg_c = val,
            Reg8::D => self.reg_d = val,
            Reg8::E => self.reg_e = val,
            Reg8::H => self.reg_h = val,
            Reg8::L => self.reg_l = val,
        }
    }

    pub fn read_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => {
                let mut result = (self.reg_a as u16) << 8;
                if self.reg_fz { result |= 0x0080 };
                if self.reg_fn { result |= 0x0040 };
                if self.reg_fh { result |= 0x0020 };
                if self.reg_fc { result |= 0x0010 };
                result
            },
            Reg16::BC => ((self.reg_b as u16) << 8) | (self.reg_c as u16),
            Reg16::DE => ((self.reg_d as u16) << 8) | (self.reg_e as u16),
            Reg16::HL => ((self.reg_h as u16) << 8) | (self.reg_l as u16),
            Reg16::SP => self.reg_sp,
            Reg16::PC => self.reg_pc,
        }
    }

    pub fn write_reg16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AF => {
                self.reg_a = (val >> 8) as u8;
                self.reg_fz = ((val >> 7) & 1) > 0;
                self.reg_fn = ((val >> 6) & 1) > 0;
                self.reg_fh = ((val >> 5) & 1) > 0;
                self.reg_fc = ((val >> 4) & 1) > 0;
            },
            Reg16::BC => {
                self.reg_b = (val >> 8) as u8;
                self.reg_c = val as u8;
            },
            Reg16::DE => {
                self.reg_d = (val >> 8) as u8;
                self.reg_e = val as u8;
            },
            Reg16::HL => {
                self.reg_h = (val >> 8) as u8;
                self.reg_l = val as u8;
            },
            Reg16::SP => self.reg_sp = val,
            Reg16::PC => self.reg_pc = val,
        }
    }

    pub fn read_regf(&self, reg: RegF) -> bool {
        match reg {
            RegF::Fz => self.reg_fz,
            RegF::Fn => self.reg_fn,
            RegF::Fh => self.reg_fh,
            RegF::Fc => self.reg_fc,
        }
    }

    pub fn write_regf(&mut self, reg: RegF, val: bool) {
        match reg {
            RegF::Fz => self.reg_fz = val,
            RegF::Fn => self.reg_fn = val,
            RegF::Fh => self.reg_fh = val,
            RegF::Fc => self.reg_fc = val,
        }
    }

    // Convenience functions

    pub fn calc_offset_pc(&self, offset: i8) -> u16 {
        self.reg_pc.wrapping_add(offset as u16)
    }

    pub fn write_flags(
        &mut self,
        z: Option<bool>,
        n: Option<bool>,
        h: Option<bool>,
        c: Option<bool>
    ) {
        if let Some(z) = z { self.reg_fz = z }
        if let Some(n) = n { self.reg_fn = n }
        if let Some(h) = h { self.reg_fh = h }
        if let Some(c) = c { self.reg_fc = c }
    }

    pub fn set_flag(&mut self, reg: RegF) {
        self.write_regf(reg, true);
    }

    pub fn clear_flag(&mut self, reg: RegF) {
        self.write_regf(reg, false);
    }
}

impl std::fmt::Display for RegisterFile {
    fn fmt(
        &self, f: &mut std::fmt::Formatter<'_>
    ) -> std::result::Result<(), std::fmt::Error> {
        let bit_fz = if self.reg_fz { 1 } else { 0 };
        let bit_fn = if self.reg_fn { 1 } else { 0 };
        let bit_fh = if self.reg_fh { 1 } else { 0 };
        let bit_fc = if self.reg_fc { 1 } else { 0 };

        write!(f, "SP=0x{:04x}  PC=0x{:04x}\n", self.reg_sp, self.reg_pc)?;
        write!(f, "AF=0x{:04x}  A=0x{:02x}\n", self.read_reg16(Reg16::AF), self.reg_a)?;
        write!(f,
            "BC=0x{:04x}  B=0x{:02x}  C=0x{:02x}\n",
            self.read_reg16(Reg16::BC),
            self.reg_b,
            self.reg_c,
        )?;
        write!(f,
            "DE=0x{:04x}  D=0x{:02x}  E=0x{:02x}\n",
            self.read_reg16(Reg16::DE),
            self.reg_d,
            self.reg_e,
        )?;
        write!(f,
            "HL=0x{:04x}  H=0x{:02x}  L=0x{:02x}\n",
            self.read_reg16(Reg16::HL),
            self.reg_h,
            self.reg_l,
        )?;
        write!(f,
            "Flags: z={} n={} h={} c={}\n",
            bit_fz,
            bit_fn,
            bit_fh,
            bit_fc,
        )?;
        
        Ok(())
    }
}
