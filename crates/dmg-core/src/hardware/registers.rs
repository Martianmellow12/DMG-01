use crate::Result;

#[derive(Debug)]
pub enum Register {
    // 8-bit registers
    A,
    B,
    C,
    D,
    E,
    H,
    L,

    // 16-bit registers
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,

    // Flag registers
    Fz,
    Fn,
    Fh,
    Fc,
}

#[derive(Debug)]
pub enum RegisterValue {
    Reg8(u8),
    Reg16(u16),
    RegFlag(bool),
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
    pub fn read_reg(&self, reg: Register) -> RegisterValue {
        match reg {
            // 8-bit
            Register::A => RegisterValue::Reg8(self.reg_a),
            Register::B => RegisterValue::Reg8(self.reg_b),
            Register::C => RegisterValue::Reg8(self.reg_c),
            Register::D => RegisterValue::Reg8(self.reg_d),
            Register::E => RegisterValue::Reg8(self.reg_e),
            Register::H => RegisterValue::Reg8(self.reg_h),
            Register::L => RegisterValue::Reg8(self.reg_l),

            // 16-bit
            Register::SP => RegisterValue::Reg16(self.reg_sp),
            Register::PC => RegisterValue::Reg16(self.reg_pc),
            Register::AF => {
                let reg_f = ((self.reg_fz as u8) << 7)
                    | ((self.reg_fn as u8) << 6)
                    | ((self.reg_fh as u8) << 5)
                    | ((self.reg_fc as u8) << 4) & 0xF0;
                RegisterValue::Reg16(u16::from_be_bytes([self.reg_a, reg_f]))
            }
            Register::BC => RegisterValue::Reg16(u16::from_be_bytes([self.reg_b, self.reg_c])),
            Register::DE => RegisterValue::Reg16(u16::from_be_bytes([self.reg_d, self.reg_e])),
            Register::HL => RegisterValue::Reg16(u16::from_be_bytes([self.reg_h, self.reg_l])),

            // Flags
            Register::Fz => RegisterValue::RegFlag(self.reg_fz),
            Register::Fn => RegisterValue::RegFlag(self.reg_fn),
            Register::Fh => RegisterValue::RegFlag(self.reg_fh),
            Register::Fc => RegisterValue::RegFlag(self.reg_fc),
        }
    }

    pub fn write_reg(&self, reg: Register, val: RegisterValue) -> Result<()> {}

    fn write_reg8(&mut self, reg: Register, val: RegisterValue) -> Result<()> {
        let RegisterValue::Reg8(val) = val else {
            return Err("Tried to write {:?} to an 8-bit register".into());
        };
        match reg {
            Register::A => self.reg_a = val,
            Register::B => self.reg_b = val,
            Register::C => self.reg_c = val,
            Register::D => self.reg_d = val,
            Register::E => self.reg_e = val,
            Register::H => self.reg_h = val,
            Register::L => self.reg_l = val,
            _ => return Err(format!("Tried to write {:?} to {:?}", val, reg).into()),
        };
        Ok(())
    }

    fn write_reg16(&mut self, reg: Register, val: RegisterValue) -> Result<()> {
        let RegisterValue::Reg16(val) = val else {
            return Err("Tried to write {:?} to an 8-bit register".into());
        };
        match reg {
            Register::AF => {
                self.reg_a = (val >> 8) as u8;
                // TODO: finish the flags and continue here
            },
            _ => return Err(format!("Tried to write {:?} to {:?}", val, reg).into()),
        };
        Ok(())
    }
}
