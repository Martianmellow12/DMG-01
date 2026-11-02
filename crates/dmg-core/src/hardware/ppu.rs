use crate::hardware::error::{HardwareError, Result};

pub struct Ppu {
    vram: [u8; 0x2000],
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; 0x2000],
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            other => panic!("Attempted VRAM read at 0x{:x}", other),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            other => panic!("Attempted VRAM write at 0x{:x}", other),
        }
    }
}
