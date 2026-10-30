use crate::hardware::Cpu;
use crate::hardware::registers::{Reg8, Reg16, RegF};

use super::Instruction;
use super::error::Result;

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<usize>{
        // Increment the program counter before execution
        let pc = cpu.registers.read_reg16(Reg16::PC);
        cpu.registers.write_reg16(
            Reg16::PC,
            pc + (self.size() as u16)
        );

        // Perform instruction execution
        Ok(match self {
            // Block 0
            Self::LdR16Imm16 { dest, imm16 } => {
                cpu.registers.write_reg16(dest.into(), *imm16);
                3
            },
            Self::LdR16MemA { dest } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let addr = dest.to_addr(cpu);
                cpu.memory.write8(addr, a_val);
                2
            },
            Self::LdAR16Mem { src } => {
                let addr = src.to_addr(cpu);
                let val = cpu.memory.read8(addr);
                cpu.registers.write_reg8(Reg8::A, val);
                2
            },
            Self::IncR8 { op } => {
                let reg_val = op.read(cpu);
                op.write(cpu, reg_val + 1);
                
                if reg_val == 0xFF { cpu.registers.set_flag(RegF::Fz) }
                cpu.registers.clear_flag(RegF::Fn);
                if (reg_val & 0xF) == 0xF { cpu.registers.set_flag(RegF::Fh);}
                
                if op.is_indirect() { 3 } else { 1 }
            },
            Self::LdR8Imm8 { dest, imm8 } => {
                dest.write(cpu, *imm8);
                if dest.is_indirect() { 3 } else { 2 }
            },
            Self::JrCondImm8 { cond, imm8 } => {
                let jump_addr = cpu.registers.calc_offset_pc(*imm8 as i8);
                let do_jump = cond.check(cpu);
                if do_jump {
                    cpu.registers.write_reg16(Reg16::PC, jump_addr);
                    3
                } else {
                    2
                }
            },

            // Block 1
            Self::LdR8R8 { dest, src } => {
                let val = src.read(cpu);
                dest.write(cpu, val);
                if dest.is_indirect() || src.is_indirect() { 2 } else { 1 }
            },

            // Block 2
            Self::XorAR8 { op } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let reg_val = op.read(cpu);
                let result = a_val ^ reg_val;
                cpu.registers.write_reg8(Reg8::A, result);

                if result == 0 { cpu.registers.set_flag(RegF::Fz) }
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.clear_flag(RegF::Fh);
                cpu.registers.clear_flag(RegF::Fc);
                
                if op.is_indirect() { 2 } else { 1 }
            },

            // Block 3
            Self::LdhCmemA => {
                let addr = 0xFF00 + cpu.registers.read_reg8(Reg8::C) as u16;
                let val = cpu.registers.read_reg8(Reg8::A);
                cpu.memory.write8(addr, val);
                2
            },
            Self::LdhImm8MemA { imm8 } => {
                let addr = 0xFF00 + (*imm8 as u16);
                let val = cpu.registers.read_reg8(Reg8::A);
                cpu.memory.write8(addr, val);
                3
            },

            // 0xCB prefix
            Self::BitB3R8 { bit_idx, op } => {
                let reg_val = cpu.registers.read_reg8(Reg8::try_from(op)?);
                let mask = bit_idx.mask();
                if (reg_val & mask) == 0 { cpu.registers.set_flag(RegF::Fz) }
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.set_flag(RegF::Fh);
                
                if op.is_indirect() { 3 } else { 2 }
            }

            _ => todo!("Instruction not yet implemented"),
        })
    }
}
