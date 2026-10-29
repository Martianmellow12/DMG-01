use super::error::{HardwareError, Result};

pub struct Memory {
    raw_ram: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            raw_ram: [0; 0xFFFF],
        }
    }

    // Raw accessors
    pub fn read8(&self, addr: u16) -> u8 {
        self.raw_ram[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.raw_ram[addr as usize] = val;
    }

    pub fn read16(&self, addr: u16) -> Result<u16> {
        if addr < 0xFFFF {
            Ok(((self.read8(addr + 1) as u16) << 8) | (self.read8(addr) as u16))
        } else {
            Err(HardwareError::OutOfBoundsAccess("read16"))
        }
    }

    pub fn write16(&mut self, addr: u16, val: u16) -> Result<()> {
        if addr < 0xFFFF {
            let bh = (val >> 8) as u8;
            let bl = val as u8;
            self.write8(addr, bl);
            self.write8(addr + 1, bh);
            Ok(())
        } else {
            Err(HardwareError::OutOfBoundsAccess("write16"))
        }
    }

    // File loading
    pub fn load_file(&mut self, file: &str, start_addr: u16) -> Result<()> {
        let start_addr = start_addr as usize;
        let bytes = std::fs::read(file)?;
        if (start_addr + bytes.len() - 1) > 0xFFFF {
            Err(HardwareError::OutOfBoundsAccess("load_file"))
        } else {
            for (idx, byte) in bytes.into_iter().enumerate() {
                self.raw_ram[start_addr + idx] = byte;
            }
            Ok(())
        }
    }
}
