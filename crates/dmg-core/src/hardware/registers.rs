#[derive(Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
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
    fn read_reg8(&self, reg: Reg8) -> u8 {
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

    fn write_reg8(&mut self, reg: Reg8, val: u8) {
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

    fn read_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => {
                let mut result = (self.reg_a as u16) << 8;
                if self.reg_fz { result &= 0x80 };
                if self.reg_fn { result &= 0x40 };
                if self.reg_fh { result &= 0x20 };
                if self.reg_fc { result &= 0x10 };
                result
            },
            Reg16::BC => ((self.reg_b as u16) << 8) | (self.reg_c as u16),
            Reg16::DE => ((self.reg_d as u16) << 8) | (self.reg_e as u16),
            Reg16::HL => ((self.reg_h as u16) << 8) | (self.reg_l as u16),
            Reg16::SP => self.reg_sp,
            Reg16::PC => self.reg_pc,
        }
    }

    fn write_reg16(&mut self, reg: Reg16, val: u16) {
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

    fn read_regf(&mut self, reg: RegF) -> bool {
        match reg {
            RegF::Fz => self.reg_fz,
            RegF::Fn => self.reg_fn,
            RegF::Fh => self.reg_fh,
            RegF::Fc => self.reg_fc,
        }
    }

    fn write_regf(&mut self, reg: RegF, val: bool) {
        match reg {
            RegF::Fz => self.reg_fz = val,
            RegF::Fn => self.reg_fn = val,
            RegF::Fh => self.reg_fh = val,
            RegF::Fc => self.reg_fc = val,
        }
    }
}
