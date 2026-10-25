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

// Block 2
const XOR_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0xA8, cb: false, size: 1 };

// Block 3
const XOR_A_IMM8: InstrMap = InstrMap { mask: 0xFF, value: 0xEE, cb: false, size: 2 };

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
    Reg(Reg16),
}

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
            0 => OpReg16::Reg(Reg16::BC),
            1 => OpReg16::Reg(Reg16::DE),
            2 => OpReg16::Reg(Reg16::HL),
            3 => OpReg16::Reg(Reg16::SP),
            _ => panic!("dec_r16 values > 3 are unreachable"),
        }
    }
}
