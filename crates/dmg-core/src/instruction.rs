mod decode;
mod error;

use crate::hardware::registers::{Reg8, Reg16, RegF};
use decode::{OpReg8, OpReg16};

#[derive(Debug)]
pub enum Instruction {
    // Block 0
    LdR16Imm16 { dest: OpReg16, imm16: u16 },

    // Block 2
    XorAR8 { op: OpReg8 },

    // Block 3
    XorAImm8 { imm8: u8 },
}
