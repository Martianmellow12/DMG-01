pub mod registers;

use crate::software::gamerom::GameRom;
use crate::software::instruction::Instruction;

use super::error::Result;
use super::bus::Bus;
use super::mbc::Mbc;
use super::ppu::Frame;
use registers::{RegisterFile};

pub struct Cpu {
    pub registers: RegisterFile,
    pub bus: Bus,
    pub gamerom: GameRom,

    en_log: bool,
}

impl Cpu {
    pub fn new(gamerom: GameRom, en_log: bool) -> Self {
        Self {
            registers: RegisterFile::new(),
            bus: Bus::new(&gamerom, en_log),
            gamerom,

            en_log,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // TODO: debug code, fix this later
        let instr = Instruction::decode(self).unwrap();
        let pc = self.registers.read_reg16(registers::Reg16::PC);
        self.log(format!("PC=0x{:x?} - [{}] {:x?}", pc, instr.size(), instr));
        let cycles = match instr.execute(self) {
            Ok(cycles) => cycles,
            Err(err) => {
                println!("Fault when executing instruction");
                println!("PC=0x{:x?} - [{}] {:x?}", pc, instr.size(), instr);
                return Err(err);
            },
        };
        self.bus.tick(cycles);
        Ok(())
    }

    // Memory functions
    pub fn push16(&mut self, val: u16) {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        self.bus.write16(sp.wrapping_sub(2), val);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_sub(2));
    }

    pub fn pop16(&mut self) -> u16 {
        let sp = self.registers.read_reg16(registers::Reg16::SP);
        let result = self.bus.read16(sp);
        self.registers.write_reg16(registers::Reg16::SP, sp.wrapping_add(2));
        result
    }

    // Graphics functions
    pub fn get_frame_buf(&mut self) -> Option<Vec<u32>> {
        // TODO: made PPU public; find a better way
        self.bus.ppu.get_frame_buf()
    }

    // Debug functions
    pub fn dump_state(&self) {
        if !self.en_log { return }
        println!("Registers");
        println!("=================");
        println!("{}", self.registers);
    }

    fn log(&self, msg: String) {
        if self.en_log {
            println!("{}", msg);
        }
    }
}
