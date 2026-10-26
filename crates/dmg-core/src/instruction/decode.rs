use crate::hardware::Cpu;
use crate::hardware::registers::{Reg8, Reg16, RegF};

use super::error::{InstructionError, Result};
use super::Instruction;

#[derive(Debug)]
pub struct DecodedInstr {
    pub instr: Instruction,
    pub size: usize,
}

//
// Instruction definitions
//

struct InstrMap {
    mask: u8,
    value: u8,
    cb: bool,
    size: usize,
}

impl InstrMap {
    fn check(&self, instr: u8, cb: bool) -> bool {
        let masked = instr & self.mask;
        (masked == self.value) && (cb == self.cb)
    }
}

// Block 0
const LD_R16_IMM16: InstrMap = InstrMap { mask: 0xCF, value: 0x01, cb: false, size: 3 };
const LD_R16MEM_A: InstrMap = InstrMap { mask: 0xCF, value: 0x02, cb: false, size: 1 };
const LD_A_R16MEM: InstrMap = InstrMap { mask: 0xCF, value: 0x0A, cb: false, size: 1 };
const INC_R8: InstrMap = InstrMap { mask: 0xC7, value: 0x04, cb: false, size: 1 };
const LD_R8_IMM8: InstrMap = InstrMap { mask: 0xC7, value: 0x06, cb: false, size: 2 };
const JR_COND_IMM8: InstrMap = InstrMap { mask: 0xE7, value: 0x20, cb: false, size: 2 };

// Block 1
const LD_R8_R8: InstrMap = InstrMap { mask: 0xC0, value: 0x40, cb: false, size: 1 };

// Block 2
const XOR_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0xA8, cb: false, size: 1 };

// Block 3
const XOR_A_IMM8: InstrMap = InstrMap { mask: 0xFF, value: 0xEE, cb: false, size: 2 };
const CALL_IMM16: InstrMap = InstrMap { mask: 0xFF, value: 0xCD, cb: false, size: 3 };
const LDH_CMEM_A: InstrMap = InstrMap { mask: 0xFF, value: 0xE2, cb: false, size: 1 };
const LDH_IMM8MEM_A: InstrMap = InstrMap { mask: 0xFF, value: 0xE0, cb: false, size: 2 };
const LDH_A_IMM8MEM: InstrMap = InstrMap { mask: 0xFF, value: 0xF0, cb: false, size: 2 };

// 0xCB prefix
const BIT_B3_R8: InstrMap = InstrMap { mask: 0xC0, value: 0x40, cb: true, size: 2 };

//
// Instruction and operand decoding
//

#[derive(Debug)]
pub enum OpReg8 {
    Reg(Reg8),
    IndirectHL,
}

#[derive(Debug)]
pub enum OpReg16 {
    RegBC,
    RegDE,
    RegHL,
    RegSP,
}

#[derive(Debug)]
pub enum OpReg16Mem {
    RegBC,
    RegDE,
    RegIncHL,
    RegDecHL,
}

#[derive(Debug)]
pub enum OpCond {
    CondNZ,
    CondZ,
    CondNC,
    CondC,
}

#[derive(Debug)]
pub struct OpBitIdx(u8);

impl Instruction {
    pub fn decode(cpu: &Cpu) -> Result<DecodedInstr> {
        let instr;
        let cb;
        let idx; // Points to instr (skips 0xCB)

        // idx always points at the instr (skipping 0xCB)
        let pc = cpu.registers.read_reg16(Reg16::PC);
        if cpu.memory.read8(pc) == 0xCB {
            idx = pc + 1;
            cb = true;
        } else {
            idx = pc;
            cb = false;
        }
        instr = cpu.memory.read8(idx);

        // Block 0

        if LD_R16_IMM16.check(instr, cb) {
            let dest = Self::dec_r16(instr, 4);
            let imm16 = cpu.memory.read16(idx + 1)?;
            return Ok(DecodedInstr {
                instr: Instruction::LdR16Imm16 { dest, imm16 },
                size: LD_R16_IMM16.size
            });
        }

        if LD_R16MEM_A.check(instr, cb) {
            let dest = Self::dec_r16mem(instr, 4);
            return Ok(DecodedInstr {
                instr: Instruction::LdR16MemA { dest },
                size: LD_R16MEM_A.size
            });
        }

        if LD_A_R16MEM.check(instr, cb) {
            let src = Self::dec_r16mem(instr, 4);
            return Ok(DecodedInstr {
                instr: Instruction::LdAR16Mem { src },
                size: LD_A_R16MEM.size
            });
        }

        if INC_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 3);
            return Ok(DecodedInstr {
                instr: Instruction::IncR8 { op },
                size: INC_R8.size
            });
        }

        if LD_R8_IMM8.check(instr, cb) {
            let dest = Self::dec_r8(instr, 3);
            let imm8 = cpu.memory.read8(idx + 1);
            return Ok(DecodedInstr {
                instr: Instruction::LdR8Imm8 { dest, imm8 },
                size: LD_R8_IMM8.size
            });
        }

        if JR_COND_IMM8.check(instr, cb) {
            let cond = Self::dec_cond(instr, 3);
            let imm8 = cpu.memory.read8(idx + 1);
            return Ok(DecodedInstr {
                instr: Instruction::JrCondImm8 { cond, imm8 },
                size: JR_COND_IMM8.size
            });
        }

        // Block 1

        if LD_R8_R8.check(instr, cb) {
            let dest = Self::dec_r8(instr, 3);
            let src = Self::dec_r8(instr, 0);
            return Ok(DecodedInstr {
                instr: Instruction::LdR8R8 { dest, src },
                size: LD_R8_R8.size
            });
        }

        // Block 2

        if XOR_A_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            return Ok(DecodedInstr {
                instr: Instruction::XorAR8 { op },
                size: XOR_A_R8.size
            });
        }

        // Block 3

        if XOR_A_IMM8.check(instr, cb) {
            let imm8 = cpu.memory.read8(idx + 1);
            return Ok(DecodedInstr {
                instr: Instruction::XorAImm8 { imm8 },
                size: XOR_A_IMM8.size
            });
        }

        if CALL_IMM16.check(instr, cb) {
            let imm16 = cpu.memory.read16(idx + 1)?;
            return Ok(DecodedInstr {
                instr: Instruction::CallImm16 { imm16 },
                size: CALL_IMM16.size
            });
        }

        if LDH_CMEM_A.check(instr, cb) {
            return Ok(DecodedInstr {
                instr: Instruction::LdhCmemA,
                size: LDH_CMEM_A.size
            });
        }

        if LDH_IMM8MEM_A.check(instr, cb) {
            let imm8 = cpu.memory.read8(idx + 1);
            return Ok(DecodedInstr {
                instr: Instruction::LdhImm8MemA { imm8 },
                size: LDH_A_IMM8MEM.size
            });
        }

        if LDH_A_IMM8MEM.check(instr, cb) {
            let imm8 = cpu.memory.read8(idx + 1);
            return Ok(DecodedInstr {
                instr: Instruction::LdhAImm8Mem { imm8 },
                size: LDH_A_IMM8MEM.size
            });
        }

        // 0xCB prefix

        if BIT_B3_R8.check(instr, cb) {
            let bit_idx = Self::dec_b3(instr, 3);
            let op = Self::dec_r8(instr, 0);
            return Ok(DecodedInstr {
                instr: Instruction::BitB3R8 { bit_idx, op },
                size: BIT_B3_R8.size
            });
        }

        Err(InstructionError::UnknownInstruction { instr, cb })
    }

    // Decode helper functions

    fn dec_r8(instr: u8, shift: usize) -> OpReg8 {
        let code = (instr >> shift) & 0x7;
        match code {
            0 => OpReg8::Reg(Reg8::B),
            1 => OpReg8::Reg(Reg8::C),
            2 => OpReg8::Reg(Reg8::D),
            3 => OpReg8::Reg(Reg8::E),
            4 => OpReg8::Reg(Reg8::H),
            5 => OpReg8::Reg(Reg8::L),
            6 => OpReg8::IndirectHL,
            7 => OpReg8::Reg(Reg8::A),
            _ => panic!("dec_r8 values > 7 are unreachable"),
        }
    }
    
    fn dec_r16(instr: u8, shift: usize) -> OpReg16 {
        let code = (instr >> shift) & 0x3;
        match code {
            0 => OpReg16::RegBC,
            1 => OpReg16::RegDE,
            2 => OpReg16::RegHL,
            3 => OpReg16::RegSP,
            _ => panic!("dec_r16 values > 3 are unreachable"),
        }
    }

    fn dec_cond(instr: u8, shift: usize) -> OpCond {
        let code = (instr >> shift) & 0x3;
        match code {
            0 => OpCond::CondNZ,
            1 => OpCond::CondZ,
            2 => OpCond::CondNC,
            3 => OpCond::CondC,
            _ => panic!("dec_cond values > 3 are unreachable"),
        }
    }

    fn dec_r16mem(instr: u8, shift: usize) -> OpReg16Mem {
        let code = (instr >> shift) & 0x3;
        match code {
            0 => OpReg16Mem::RegBC,
            1 => OpReg16Mem::RegDE,
            2 => OpReg16Mem::RegIncHL,
            3 => OpReg16Mem::RegDecHL,
            _ => panic!("dec_r16mem values > 3 are unreachable"),
        }
    }

    fn dec_b3(instr: u8, shift: usize) -> OpBitIdx {
        OpBitIdx((instr >> shift) & 0x7)
    }
}
