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
            pc.wrapping_add(self.size() as u16)
        );

        // Perform instruction execution
        Ok(match self {
            // Block 0
            Self::LdR16Imm16 { dest, imm16 } => {
                dest.write(cpu, *imm16);
                3
            },
            Self::LdR16MemA { dest } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                dest.write_and_step(cpu, a_val);
                2
            },
            Self::LdAR16Mem { src } => {
                let val = src.read_and_step(cpu);
                cpu.registers.write_reg8(Reg8::A, val);
                2
            },
            Self::IncR16 { op } => {
                let val = op.read(cpu);
                op.write(cpu, val.wrapping_add(1));
                2
            },
            Self::IncR8 { op } => {
                let reg_val = op.read(cpu);
                op.write(cpu, reg_val.wrapping_add(1));
                
                cpu.registers.write_regf(RegF::Fz, reg_val == 0xFF);
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.write_regf(RegF::Fh, (reg_val & 0xF) == 0xF);
                
                if op.is_indirect() { 3 } else { 1 }
            },
            Self::DecR8 { op } => {
                let val = op.read(cpu);
                op.write(cpu, val.wrapping_sub(1));

                cpu.registers.write_regf(RegF::Fz, val == 0x01);
                cpu.registers.set_flag(RegF::Fn);
                cpu.registers.write_regf(RegF::Fh, (val & 0xF) == 0x0);
                                
                if op.is_indirect() { 3 } else { 1 }
            },
            Self::LdR8Imm8 { dest, imm8 } => {
                dest.write(cpu, *imm8);
                if dest.is_indirect() { 3 } else { 2 }
            },
            Self::Rla => {
                let val = cpu.registers.read_reg8(Reg8::A);
                let c_new = (val >> 7) > 0;
                let bit0_new = if cpu.registers.read_regf(RegF::Fc) { 1 } else { 0 };
                let val_new = (val << 1) | bit0_new;
                cpu.registers.write_reg8(Reg8::A, val_new);
                cpu.registers.write_regf(RegF::Fc, c_new);

                cpu.registers.clear_flag(RegF::Fz);
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.clear_flag(RegF::Fh);
                
                1
            },
            Self::JrImm8 { imm8 } => {
                let jump_addr = cpu.registers.calc_offset_pc(*imm8 as i8);
                cpu.registers.write_reg16(Reg16::PC, jump_addr);
                3
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

                cpu.registers.write_regf(RegF::Fz, result == 0);
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.clear_flag(RegF::Fh);
                cpu.registers.clear_flag(RegF::Fc);
                
                if op.is_indirect() { 2 } else { 1 }
            },

            // Block 3
            Self::CpAImm8 { imm8 } => {
                let a_val = cpu.registers.read_reg8(Reg8::A);
                let f_h = (a_val & 0xF) < (*imm8 & 0xF);
                cpu.registers.write_regf(RegF::Fz, a_val == *imm8);
                cpu.registers.write_regf(RegF::Fn, true);
                cpu.registers.write_regf(RegF::Fh, f_h);
                cpu.registers.write_regf(RegF::Fc, *imm8 > a_val);
                2
            },
            Self::Ret => {
                let val = cpu.pop16();
                cpu.registers.write_reg16(Reg16::PC, val);
                4
            },
            Self::CallImm16 { imm16 } => {
                let pc = cpu.registers.read_reg16(Reg16::PC);
                cpu.push16(pc);
                cpu.registers.write_reg16(Reg16::PC, *imm16);
                6
            },
            Self::PopR16Stk { reg } => {
                let val = cpu.pop16();
                reg.write(cpu, val);
                3
            },
            Self::PushR16Stk { reg } => {
                let val = reg.read(cpu);
                cpu.push16(val);
                4
            },
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
            Self::LdImm16MemA { imm16 } => {
                let val = cpu.registers.read_reg8(Reg8::A);
                cpu.memory.write8(*imm16, val);
                4
            },
            Self::LdhAImm8Mem { imm8 } => {
                let val = cpu.memory.read8(0xFF00 + (*imm8 as u16));
                cpu.registers.write_reg8(Reg8::A, val);
                3
            },

            // 0xCB prefix
            Self::RlR8 { op } => {
                let val = op.read(cpu);
                let c_new = (val >> 7) > 0;
                let bit0_new = if cpu.registers.read_regf(RegF::Fc) { 1 } else { 0 };
                let val_new = (val << 1) | bit0_new;
                cpu.registers.write_regf(RegF::Fc, c_new);
                op.write(cpu, val_new);

                cpu.registers.write_regf(RegF::Fz, val_new == 0);
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.clear_flag(RegF::Fh);

                if op.is_indirect() { 4 } else { 2 }
            },
            Self::BitB3R8 { bit_idx, op } => {
                let val = op.read(cpu);
                let mask = bit_idx.mask();
                cpu.registers.write_regf(RegF::Fz, (val & mask) == 0);
                cpu.registers.clear_flag(RegF::Fn);
                cpu.registers.set_flag(RegF::Fh);
                
                if op.is_indirect() { 3 } else { 2 }
            }

            _ => todo!("Instruction not yet implemented"),
        })
    }
}
