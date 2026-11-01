use super::error;

use super::mbc::Mbc;

pub struct Bus {
    mbc: Mbc,
}
// TODO: implement this
impl Bus {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Bus {
            mbc: Mbc::new(rom_banks, ram_banks),
        }
    }
    
    pub fn read8(&self, addr: u16) -> u8 {
        0
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        ()
    }

    pub fn read16(&self, addr: u16) -> u16 {
        0
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        ()
    }
}
