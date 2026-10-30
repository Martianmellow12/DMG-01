use crate::hardware::Cpu;
use crate::hardware::registers::{Reg8, Reg16, RegF};

use super::Instruction;
use super::error::{InstructionError, Result};

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
                let reg = Reg16::from(dest);
                cpu.registers.write_reg16(reg, *imm16);
                3
            },
            Self::LdR16MemA { dest } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let addr = dest.read_addr(cpu);
                cpu.memory.write8(addr, a_val);
                2
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

            // Block 2
            Self::XorAR8 { op } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let reg_val = op.read(cpu);
                let result = a_val ^ reg_val;
                cpu.registers.write_reg8(Reg8::A, result);
                cpu.registers.write_flags(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some(false)
                );
                1
            },

            // 0xCB prefix
            Self::BitB3R8 { bit_idx, op } => {
                let reg_val = cpu.registers.read_reg8(Reg8::try_from(op)?);
                let mask = bit_idx.mask();
                cpu.registers.write_flags(
                    Some((reg_val & mask) == 0),
                    Some(false),
                    Some(true),
                    None,
                );
                2
            }

            _ => todo!("Instruction not yet implemented"),
        })
    }
}
