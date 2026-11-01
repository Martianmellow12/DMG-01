use crate::hardware::cpu::Cpu;
use crate::hardware::cpu::registers::{Reg8, Reg16, RegF};

use super::error::{InstructionError, Result};
use super::Instruction;

use super::{OpReg8, OpReg16, OpReg16Stk, OpReg16Mem, OpCond, OpBitIdx};

//
// Instruction definitions
//

struct InstrMap {
    mask: u8,
    value: u8,
    cb: bool,
}

impl InstrMap {
    fn check(&self, instr: u8, cb: bool) -> bool {
        let masked = instr & self.mask;
        (masked == self.value) && (cb == self.cb)
    }
}

// Block 0
const LD_R16_IMM16: InstrMap = InstrMap { mask: 0xCF, value: 0x01, cb: false };
const LD_R16MEM_A: InstrMap = InstrMap { mask: 0xCF, value: 0x02, cb: false };
const LD_A_R16MEM: InstrMap = InstrMap { mask: 0xCF, value: 0x0A, cb: false };
const INC_R16:  InstrMap = InstrMap { mask: 0xCF, value: 0x03, cb: false };
const INC_R8: InstrMap = InstrMap { mask: 0xC7, value: 0x04, cb: false };
const DEC_R8: InstrMap = InstrMap { mask: 0xC7, value: 0x05, cb: false };
const LD_R8_IMM8: InstrMap = InstrMap { mask: 0xC7, value: 0x06, cb: false };
const RLA: InstrMap = InstrMap { mask: 0xFF, value: 0x17, cb: false };
const JR_IMM8: InstrMap = InstrMap { mask: 0xFF, value: 0x18, cb: false };
const JR_COND_IMM8: InstrMap = InstrMap { mask: 0xE7, value: 0x20, cb: false };

// Block 1
const LD_R8_R8: InstrMap = InstrMap { mask: 0xC0, value: 0x40, cb: false };

// Block 2
const ADD_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0x80, cb: false };
const SUB_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0x90, cb: false };
const XOR_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0xA8, cb: false };
const CP_A_R8: InstrMap = InstrMap { mask: 0xF8, value: 0xB8, cb: false };

// Block 3
const XOR_A_IMM8: InstrMap = InstrMap { mask: 0xFF, value: 0xEE, cb: false };
const CP_A_IMM8: InstrMap = InstrMap { mask: 0xFF, value: 0xFE, cb: false };
const RET: InstrMap = InstrMap { mask: 0xFF, value: 0xC9, cb: false };
const CALL_IMM16: InstrMap = InstrMap { mask: 0xFF, value: 0xCD, cb: false };
const POP_R16STK: InstrMap = InstrMap { mask: 0xCF, value: 0xC1, cb: false };
const PUSH_R16STK: InstrMap = InstrMap { mask: 0xCF, value: 0xC5, cb: false };
const LDH_CMEM_A: InstrMap = InstrMap { mask: 0xFF, value: 0xE2, cb: false };
const LDH_IMM8MEM_A: InstrMap = InstrMap { mask: 0xFF, value: 0xE0, cb: false };
const LD_IMM16MEM_A: InstrMap = InstrMap { mask: 0xFF, value: 0xEA, cb: false };
const LDH_A_IMM8MEM: InstrMap = InstrMap { mask: 0xFF, value: 0xF0, cb: false };

// 0xCB prefix
const RL_R8: InstrMap = InstrMap { mask: 0xF8, value: 0x10, cb: true };
const BIT_B3_R8: InstrMap = InstrMap { mask: 0xC0, value: 0x40, cb: true };

//
// Instruction and operand decoding
//

impl Instruction {
    pub fn decode(cpu: &Cpu) -> Result<Instruction> {
        let instr;
        let cb;
        let idx; // Points to instr (skips 0xCB)

        // idx always points at the instr (skipping 0xCB)
        let pc = cpu.registers.read_reg16(Reg16::PC);
        if cpu.bus.read8(pc) == 0xCB {
            idx = pc.wrapping_add(1);
            cb = true;
        } else {
            idx = pc;
            cb = false;
        }
        instr = cpu.bus.read8(idx);

        // Block 0

        Ok(if LD_R16_IMM16.check(instr, cb) {
            let dest = Self::dec_r16(instr, 4);
            let imm16 = cpu.bus.read16(idx.wrapping_add(1));
            Instruction::LdR16Imm16 { dest, imm16 }
        }

        else if LD_R16MEM_A.check(instr, cb) {
            let dest = Self::dec_r16mem(instr, 4);
            Instruction::LdR16MemA { dest }
        }

        else if LD_A_R16MEM.check(instr, cb) {
            let src = Self::dec_r16mem(instr, 4);
            Instruction::LdAR16Mem { src }
        }

        else if INC_R16.check(instr, cb) {
            let op = Self::dec_r16(instr, 4);
            Instruction::IncR16 { op }
        }

        else if INC_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 3);
            Instruction::IncR8 { op }
        }

        else if DEC_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 3);
            Instruction::DecR8 { op }
        }

        else if LD_R8_IMM8.check(instr, cb) {
            let dest = Self::dec_r8(instr, 3);
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::LdR8Imm8 { dest, imm8 }
        }

        else if RLA.check(instr, cb) {
            Instruction::Rla
        }

        else if JR_IMM8.check(instr, cb) {
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::JrImm8 { imm8 }
        }

        else if JR_COND_IMM8.check(instr, cb) {
            let cond = Self::dec_cond(instr, 3);
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::JrCondImm8 { cond, imm8 }
        }

        // Block 1

        else if LD_R8_R8.check(instr, cb) {
            let dest = Self::dec_r8(instr, 3);
            let src = Self::dec_r8(instr, 0);
            Instruction::LdR8R8 { dest, src }
        }

        // Block 2

        else if ADD_A_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            Instruction::AddAR8 { op }
        }

        else if SUB_A_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            Instruction::SubAR8 { op }
        }

        else if XOR_A_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            Instruction::XorAR8 { op }
        }

        else if CP_A_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            Instruction::CpAR8 { op }
        }

        // Block 3

        else if XOR_A_IMM8.check(instr, cb) {
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::XorAImm8 { imm8 }
        }

        else if CP_A_IMM8.check(instr, cb) {
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::CpAImm8 { imm8 }
        }

        else if RET.check(instr, cb) {
            Instruction::Ret
        }

        else if CALL_IMM16.check(instr, cb) {
            let imm16 = cpu.bus.read16(idx.wrapping_add(1));
            Instruction::CallImm16 { imm16 }
        }

        else if POP_R16STK.check(instr, cb) {
            let reg = Self::dec_r16stk(instr, 4);
            Instruction::PopR16Stk { reg }
        }

        else if PUSH_R16STK.check(instr, cb) {
            let reg = Self::dec_r16stk(instr, 4);
            Instruction::PushR16Stk { reg }
        }

        else if LDH_CMEM_A.check(instr, cb) {
            Instruction::LdhCmemA
        }

        else if LDH_IMM8MEM_A.check(instr, cb) {
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::LdhImm8MemA { imm8 }
        }

        else if LD_IMM16MEM_A.check(instr, cb) {
            let imm16 = cpu.bus.read16(idx.wrapping_add(1));
            Instruction::LdImm16MemA { imm16 }
        }

        else if LDH_A_IMM8MEM.check(instr, cb) {
            let imm8 = cpu.bus.read8(idx.wrapping_add(1));
            Instruction::LdhAImm8Mem { imm8 }
        }

        // 0xCB prefix

        else if RL_R8.check(instr, cb) {
            let op = Self::dec_r8(instr, 0);
            Instruction::RlR8 { op }
        }

        else if BIT_B3_R8.check(instr, cb) {
            let bit_idx = Self::dec_b3(instr, 3);
            let op = Self::dec_r8(instr, 0);
            Instruction::BitB3R8 { bit_idx, op }
        }

        else {
            return Err(InstructionError::UnknownInstruction { instr, cb }.into())
        })
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

    fn dec_r16stk(instr: u8, shift: usize) -> OpReg16Stk {
        let code = (instr >> shift) & 0x3;
        match code {
            0 => OpReg16Stk::RegBC,
            1 => OpReg16Stk::RegDE,
            2 => OpReg16Stk::RegHL,
            3 => OpReg16Stk::RegAF,
            _ => panic!("dec_r16stk values > 3 are unreachable"),
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

    fn dec_b3(instr: u8, shift: usize) -> OpBitIdx {
        OpBitIdx((instr >> shift) & 0x7)
    }
}
