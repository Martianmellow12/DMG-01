use crate::hardware::error;

use crate::software::gamerom::GameRom;

pub struct Mbc1 {
    // Memory regions
    rom: Vec<u8>,
    ram: Vec<u8>,

    // Registers
    ram_en: bool,
    rom_bank: u8,
    ram_bank: u8,
    bank_mode: BankingMode,
}

// TODO: implement this
impl Mbc1 {
    pub fn new(gamerom: &GameRom) -> Self {
        let rom_banks = gamerom.rom_size.num_banks();
        let ram_banks = gamerom.rom_size.num_banks();
        Mbc1 {
            rom: vec![0; rom_banks * 0x4000],
            ram: vec![0; ram_banks * 0x2000],
            ram_en: false,
            rom_bank: 0,
            ram_bank: 0,
            bank_mode: BankingMode::Simple,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // TODO: support reads from bank X0
            0x0000..0x3FFF => self.rom[addr as usize],
            0x4000..0x7FFF => self.rom[addr as usize],
            0xA000..0xBFFF => self.ram[(addr - 0xA000) as usize],
            _ => {
                todo!("Implement error handling for bad MBC accesses");
                0
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xA000..0xBFFF => self.ram[(addr - 0xA000) as usize] = val,
            _ => {
                todo!("Implement all write stuff")
            }
        }
    }
}

enum BankingMode {
    Simple,
    Advanced,
}
