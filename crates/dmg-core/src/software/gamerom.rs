use crate::software::error::{GameRomError, Result};

const HDR_OFF: usize = 0x100;
const HDR_SIZE: usize = 0x4F;
const CART_TYPE_OFF: usize = HDR_OFF + 0x47;
const ROM_SIZE_OFF: usize = HDR_OFF + 0x48;
const RAM_SIZE_OFF: usize = HDR_OFF + 0x49;

pub struct GameRom {
    pub cart_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
}

impl GameRom {
    pub fn new(rom_path: &str) -> Result<Self> {
        let bytes = std::fs::read(rom_path)?;
        if bytes.len() < (HDR_OFF + HDR_SIZE) {
            return Err(GameRomError::BadHeaderSize(bytes.len()).into());
        }

        let cart_type = CartridgeType::try_from(bytes[CART_TYPE_OFF])?;
        let rom_size = RomSize::try_from(bytes[ROM_SIZE_OFF])?;
        let ram_size = RamSize::try_from(bytes[RAM_SIZE_OFF])?;
        
        Ok(GameRom {
            cart_type,
            rom_size,
            ram_size,
        })
    }
}

pub enum CartridgeType {
    Mbc1,
}

impl TryFrom<u8> for CartridgeType {
    type Error = GameRomError;
    
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x01 => Self::Mbc1,
            other => return Err(GameRomError::BadCartridgeTypeByte(other)),
        })
    }
}

pub enum RomSize {
    Banks2,
    Banks4,
    Banks8,
    Banks16,
    Banks32,
    Banks64,
    Banks128,
    Banks256,
    Banks512,
}

impl RomSize {
    pub fn num_banks(&self) -> usize {
        match self {
            Self::Banks2 => 2,
            Self::Banks4 => 4,
            Self::Banks8 => 8,
            Self::Banks16 => 16,
            Self::Banks32 => 32,
            Self::Banks64 => 64,
            Self::Banks128 => 128,
            Self::Banks256 => 256,
            Self::Banks512 => 512,
        }
    }
}

impl TryFrom<u8> for RomSize {
    type Error = GameRomError;
    
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::Banks2,
            0x01 => Self::Banks4,
            0x02 => Self::Banks8,
            0x03 => Self::Banks16,
            0x04 => Self::Banks32,
            0x05 => Self::Banks64,
            0x06 => Self::Banks128,
            0x07 => Self::Banks256,
            0x08 => Self::Banks512,
            other => return Err(GameRomError::BadRomSizeByte(other))
        })
    }
}

pub enum RamSize {
    None,
    Banks1,
    Banks4,
    Banks16,
    Banks8,
}

impl RamSize {
    pub fn num_banks(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Banks1 => 1,
            Self::Banks4 => 4,
            Self::Banks16 => 16,
            Self::Banks8 => 8,
        }
    }
}

impl TryFrom<u8> for RamSize {
    type Error = GameRomError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::None,
            0x02 => Self::Banks1,
            0x03 => Self::Banks4,
            0x04 => Self::Banks16,
            0x05 => Self::Banks8,
            other => return Err(GameRomError::BadRamSizeByte(other)),
        })
    }
}
