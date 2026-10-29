use crate::hardware::Cpu;
use crate::hardware::registers::{Reg8, Reg16, RegF};

use super::Instruction;
use super::error::{InstructionError, Result};

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<()>{
        // Increment the program counter before execution
        let pc = cpu.registers.read_reg16(Reg16::PC);
        cpu.registers.write_reg16(
            Reg16::PC,
            pc + (self.size() as u16)
        );

        // Perform instruction execution
        match self {
            // Block 0
            Self::LdR16Imm16 { dest, imm16 } => {
                let reg = Reg16::from(dest);
                cpu.registers.write_reg16(reg, *imm16);
            },
            Self::LdR16MemA { dest } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let addr = dest.read_addr(cpu);
                cpu.memory.write8(addr, a_val);
            },

            // Block 2
            Self::XorAR8 { op } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let reg_val = op.read(cpu);
                let result = a_val ^ reg_val;
                cpu.registers.write_reg8(Reg8::A, result);
                cpu.registers.write_flags(
                    result == 0,
                    false,
                    false,
                    false
                );
            },

            _ => todo!("Instruction not yet implemented"),
        }
        Ok(())
    }
}
