use crate::hardware::error::{HardwareError, Result};

pub struct Ppu {
    vram: [u8; 0x2000],
    reg_lcdc: u8,
    reg_stat: u8,
    reg_scy: u8,
    reg_scx: u8,
    reg_ly: LcdYCoord,
    reg_bgp: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; 0x2000],
            reg_lcdc: 0,
            reg_stat: 0,
            reg_scx: 0,
            reg_scy: 0,
            reg_ly: LcdYCoord(0),
            reg_bgp: 0,
        }
    }

    pub fn tick(&mut self) {
        self.reg_ly.inc();
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFF40 => self.reg_lcdc,
            0xFF41 => self.reg_stat,
            0xFF42 => self.reg_scy,
            0xFF43 => self.reg_scx,
            0xFF44 => self.reg_ly.read(),
            0xFF47 => self.reg_bgp,
            other => panic!("Attempted VRAM read at 0x{:x}", other),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFF40 => self.reg_lcdc = val,
            0xFF41 => self.reg_stat = val,
            0xFF42 => self.reg_scy = val,
            0xFF43 => self.reg_scx = val,
            0xFF44 => (),
            0xFF47 => self.reg_bgp = val,
            other => panic!("Attempted VRAM write at 0x{:x}", other),
        }
    }
}

struct LcdYCoord(u16);

impl LcdYCoord {
    pub fn inc(&mut self) {
        self.0 += 1;
        if (self.0 / 114) > 153 {
            self.0 = 0;
        }
    }

    pub fn read(&self) -> u8 {
        (self.0 / 114) as u8
    }
}
