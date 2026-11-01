pub mod mbc1;

pub enum Mbc {
    Mbc1(mbc1::Mbc1),
}

impl Mbc {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Mbc::Mbc1(mbc1::Mbc1::new(rom_banks, ram_banks))
    }
}
