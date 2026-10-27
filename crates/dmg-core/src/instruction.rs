mod decode;
mod error;

use decode::{OpReg8, OpReg16, OpReg16Mem, OpCond, OpBitIdx};

#[derive(Debug)]
pub enum Instruction {
    // Block 0
    LdR16Imm16 { dest: OpReg16, imm16: u16 },
    LdR16MemA { dest: OpReg16Mem },
    LdAR16Mem { src: OpReg16Mem },
    IncR16 { op: OpReg16 },
    IncR8 { op: OpReg8 },
    DecR8 { op: OpReg8 },
    LdR8Imm8 { dest: OpReg8, imm8: u8 },
    JrCondImm8 { cond: OpCond, imm8: u8 },

    // Block 1
    LdR8R8 { dest: OpReg8, src: OpReg8 },

    // Block 2
    XorAR8 { op: OpReg8 },

    // Block 3
    XorAImm8 { imm8: u8 },
    CpAImm8 { imm8: u8 },
    CallImm16 { imm16: u16 },
    LdhCmemA,
    LdhImm8MemA { imm8: u8 },
    LdImm16MemA { imm16: u16 },
    LdhAImm8Mem { imm8: u8 },

    // 0xCB prefix
    BitB3R8 { bit_idx: OpBitIdx, op: OpReg8 },
}

impl Instruction {
    pub fn size(&self) -> usize {
        match self {
            // Block 0
            Self::LdR16Imm16 { .. } => 3,
            Self::LdR16MemA { .. } => 1,
            Self::LdAR16Mem { .. } => 1,
            Self::IncR16 { .. } => 1,
            Self::IncR8 { .. } => 1,
            Self::DecR8 { .. } => 1,
            Self::LdR8Imm8 { .. } => 2,
            Self::JrCondImm8 { .. } => 2,

            // Block 1
            Self::LdR8R8 { .. } => 1,

            // Block 2
            Self::XorAR8 { .. } => 1,

            // Block 3
            Self::XorAImm8 { .. } => 2,
            Self::CpAImm8 { .. } => 2,
            Self::CallImm16 { .. } => 3,
            Self::LdhCmemA => 1,
            Self::LdhImm8MemA { .. } => 2,
            Self::LdImm16MemA { .. } => 3,
            Self::LdhAImm8Mem { .. } => 2,

            // 0xCB prefix
            Self::BitB3R8 { .. } => 2,
        }
    }
}
