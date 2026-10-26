mod decode;
mod error;

use decode::{OpReg8, OpReg16, OpReg16Mem, OpCond, OpBitIdx};

#[derive(Debug)]
pub enum Instruction {
    // Block 0
    LdR16Imm16 { dest: OpReg16, imm16: u16 },
    LdR16MemA { dest: OpReg16Mem },
    LdAR16Mem { src: OpReg16Mem },
    IncR8 { op: OpReg8 },
    LdR8Imm8 { dest: OpReg8, imm8: u8 },
    JrCondImm8 { cond: OpCond, imm8: u8 },

    // Block 1
    LdR8R8 { dest: OpReg8, src: OpReg8 },

    // Block 2
    XorAR8 { op: OpReg8 },

    // Block 3
    XorAImm8 { imm8: u8 },
    CallImm16 { imm16: u16 },
    LdhCmemA,
    LdhImm8MemA { imm8: u8 },
    LdhAImm8Mem { imm8: u8 },

    // 0xCB prefix
    BitB3R8 { bit_idx: OpBitIdx, op: OpReg8 },
}
