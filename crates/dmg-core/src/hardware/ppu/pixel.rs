#[derive(Debug, Clone, Copy)]
pub enum Pixel {
    L0,
    L1,
    L2,
    L3,
}

impl From<u8> for Pixel {
    fn from(value: u8) -> Pixel {
        match value & 0x3 {
            0 => Pixel::L0,
            1 => Pixel::L1,
            2 => Pixel::L2,
            3 => Pixel::L3,
            _ => panic!("Pixel: unreachable arm"),
        }
    }
}
