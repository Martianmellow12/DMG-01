use super::error;

use crate::software::gamerom::GameRom;

use super::mbc::Mbc;
use super::ppu::Ppu;

pub struct Bus {
    brom: Option<[u8; 256]>,
    mbc: Mbc,
    pub ppu: Ppu,
    hram: [u8; 0x007F],

    en_log: bool,
    
    // TODO: dummy spaces, replace
    dummy_io: [u8; 0x0080],
}
// TODO: implement this
impl Bus {
    pub fn new(gamerom: &GameRom, en_log: bool) -> Self {
        Bus {
            brom: None,
            mbc: Mbc::new(gamerom),
            ppu: Ppu::new(),
            hram: [0; 0x007F],

            en_log,

            // TODO: dummy spaces, replace
            dummy_io: [0; 0x0080],
        }
    }

    pub fn tick(&mut self, cycles: usize) {
        for i in 0..cycles {
            self.ppu.tick();
        }
    }
    
    pub fn read8(&self, addr: u16) -> u8 {
        // Catch BROM accesses
        if let Some(brom) = self.brom && (0x0000..=0x00FF).contains(&addr) {
            return brom[addr as usize];
        }
        
        let result = match addr {
            0x0000..=0x7FFF => {
                self.mbc.read(addr)
            },
            0x8000..=0x9FFF => {
                self.ppu.read(addr)
            },
            0xA000..=0xBFFF => {
                self.mbc.read(addr)
            },
            // LCD registers
            0xFF40..=0xFF4B => {
                self.ppu.read(addr)
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
        };

        self.log(
            format!("BUS READ [0x{:x}] -> 0x{:x}", addr, result)
        );
        result
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
            // LCD registers
            0xFF40..=0xFF4B => {
                self.ppu.write(addr, val);
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
        self.log(
            format!("BUS WRITE 0x{:x} -> [0x{:x}]", val, addr)
        );
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

    pub fn log(&self, msg: String) {
        if self.en_log {
            println!("{}", msg);
        }
    }
}
