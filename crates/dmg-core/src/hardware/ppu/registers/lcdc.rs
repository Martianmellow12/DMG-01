pub struct RegLcdc(u8);

impl RegLcdc {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn write(&mut self, value: u8) {
        self.0 = value
    }

    // LCDC.3 - BG tile map
    pub fn bg_tile_map(&self) -> BgTileMap {
        if self.read_bit(3) == 0 {
            BgTileMap::Map0
        } else {
            BgTileMap::Map1
        }
    }

    // LCDC.4 - BG/window tile area
    pub fn bg_tile_area(&self) -> BgTileArea {
        if self.read_bit(4) == 0 {
            BgTileArea::Area0
        } else {
            BgTileArea::Area1
        }
    }

    // LCDC.7 - LCD/PPU enable
    pub fn lcd_en(&self) -> bool {
        self.read_bit(7) == 1
    }
    
    fn read_bit(&self, pos: usize) -> usize {
        ((self.0 >> pos) & 1) as usize
    }
}

pub enum BgTileMap {
    Map0,
    Map1,
}

impl BgTileMap {
    pub fn base_addr(&self) -> u16 {
        match self {
            Self::Map0 => 0x1800,
            Self::Map1 => 0x1C00,
        }
    }
}

pub enum BgTileArea {
    Area0,
    Area1,
}

impl BgTileArea {
    pub fn idx_addr(&self, idx: u8) -> u16 {
        let idx = idx as u16;
        match self {
            Self::Area0 => {
                match idx {
                    0..=127 => 0x1000 + (idx * 16),
                    128..=255 => 0x800 + ((idx - 128) * 16),
                    _ => panic!("BgTileMapArea - Unreachable"),
                }
            },
            Self::Area1 => idx * 16,
        }
    }
}
