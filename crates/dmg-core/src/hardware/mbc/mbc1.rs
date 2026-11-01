use crate::hardware::error;

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
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Mbc1 {
            rom: vec![0; rom_banks * 0x4000],
            ram: vec![0; ram_banks * 0x4000],
            ram_en: false,
            rom_bank: 0,
            ram_bank: 0,
            bank_mode: BankingMode::Simple,
        }
    }

    pub fn read(addr: u16) -> u8 {
        0
    }

    pub fn write(addr: u16, val: u8) {
        ()
    }
}

enum BankingMode {
    Simple,
    Advanced,
}
