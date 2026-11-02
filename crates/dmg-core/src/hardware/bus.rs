use super::error;

use crate::software::gamerom::GameRom;

use super::mbc::Mbc;
use super::ppu::Ppu;

pub struct Bus {
    brom: Option<[u8; 256]>,
    mbc: Mbc,
    ppu: Ppu,
    hram: [u8; 0x007F],
    
    // TODO: dummy spaces, replace
    dummy_io: [u8; 0x0080],
}
// TODO: implement this
impl Bus {
    pub fn new(gamerom: &GameRom) -> Self {
        Bus {
            brom: None,
            mbc: Mbc::new(gamerom),
            ppu: Ppu::new(),
            hram: [0; 0x007F],

            // TODO: dummy spaces, replace
            dummy_io: [0; 0x0080],
        }
    }
    
    pub fn read8(&self, addr: u16) -> u8 {
        // Catch BROM accesses
        if let Some(brom) = self.brom && (0x0000..=0x00FF).contains(&addr) {
            return brom[addr as usize];
        }
        
        match addr {
            0x0000..=0x7FFF => {
                self.mbc.read(addr)
            },
            0x8000..=0x9FFF => {
                self.ppu.read(addr)
            },
            0xA000..=0xBFFF => {
                self.mbc.read(addr)
            },
            0xFF00..=0xFF7F => {
                self.dummy_io[(addr - 0xFF00) as usize]
            },
            0xFF80..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize]
            },
            _ => {
                todo!("Implement remaining bus read");
                0
            }
        }
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => {
                self.mbc.write(addr, val);
            },
            0x8000..=0x9FFF => {
                self.ppu.write(addr, val);
            },
            0xA000..=0xBFFF => {
                self.mbc.write(addr, val);
            },
            0xFF00..=0xFF7F => {
                self.dummy_io[(addr - 0xFF00) as usize] = val;
            },
            0xFF80..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize] = val;
            },
            _ => {
                todo!("Implement remaining bus writes");
            }
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let bl = self.read8(addr);
        let bh = self.read8(addr.wrapping_add(1));
        ((bh as u16) << 8) | (bl as u16)
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        let bl = val as u8;
        let bh = (val >> 8) as u8;
        self.write8(addr, bl);
        self.write8(addr.wrapping_add(1), bh);
    }

    pub fn load_brom(&mut self, brom_bytes: [u8; 256]) {
        self.brom = Some(brom_bytes);
    }
}
