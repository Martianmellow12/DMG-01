use std::collections::VecDeque;
use std::cell::RefCell;

use super::error::{HardwareError, Result};

pub struct Memory {
    raw_ram: [u8; 0x10000],
    log: Option<RefCell<VecDeque<String>>>,
}

impl Memory {
    pub fn new(en_log: bool) -> Self {
        let log = if en_log {
            Some(RefCell::new(VecDeque::new()))
        } else {
            None
        };
        Self {
            raw_ram: [0; 0x10000],
            log,
        }
    }

    // Raw accessors
    pub fn read8(&self, addr: u16) -> u8 {
        let result = self.raw_ram[addr as usize];
        self.add_log(format!("READ8 [{:04x}] -> {:02x}", addr, result));
        result
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.add_log(format!("WRITE8 [{:04x}] <- {:02x}", addr, val));
        self.raw_ram[addr as usize] = val;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let bh = self.raw_ram[addr.wrapping_add(1) as usize];
        let bl = self.raw_ram[addr as usize];
        let result = ((bh as u16) << 8) | (bl as u16);
        self.add_log(format!("READ16 [{:04x}] -> {:04x}", addr, result));
        result
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.add_log(format!("WRITE16 [{:04x}] <- {:04x}", addr, val));
        self.raw_ram[addr.wrapping_add(1) as usize] = (val >> 8) as u8;
        self.raw_ram[addr as usize] = val as u8;
    }

    // File loading
    pub fn load_file(&mut self, file: &str, start_addr: u16) -> Result<()> {
        let start_addr = start_addr as usize;
        let bytes = std::fs::read(file)?;
        if (start_addr + bytes.len() - 1) > 0xFFFF {
            Err(HardwareError::OutOfBoundsAccess("load_file").into())
        } else {
            for (idx, byte) in bytes.into_iter().enumerate() {
                self.raw_ram[start_addr + idx] = byte;
            }
            Ok(())
        }
    }

    // Log management
    pub fn drain_log(&self) {
        if let Some(log) = &self.log {
            let mut log = log.borrow_mut();
            while let Some(msg) = log.pop_front() {
                println!("{}", msg);
            }
        }
    }
    
    fn add_log(&self, msg: String) {
        if let Some(log) = &self.log {
            let mut log = log.borrow_mut();
            log.push_back(msg);
        }
    }
}
