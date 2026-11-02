pub mod mbc1;

use crate::software::gamerom::{GameRom, CartridgeType};

pub enum Mbc {
    Mbc1(mbc1::Mbc1),
}

impl Mbc {
    pub fn new(gamerom: &GameRom) -> Self {
        match gamerom.cart_type {
            CartridgeType::Mbc1 => Mbc::Mbc1(mbc1::Mbc1::new(gamerom)),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Self::Mbc1(mbc) => mbc.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match self {
            Self::Mbc1(mbc) => mbc.write(addr, val),
        }
    }
}
