mod registers;
mod pixel;
mod tile;
mod tilemap;
mod frame;
mod video_mode;

use rand;

use crate::hardware::error::{HardwareError, Result};

use registers::RegisterFile;
use pixel::Pixel;
use tile::Tile;
use tilemap::TileMap;
use frame::Frame;
use video_mode::VideoMode;

pub struct Ppu {
    registers: RegisterFile,
    frame: Frame,
    vram: [u8; 0x2000],
    bgmap_cache: Option<TileMap>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: RegisterFile::new(),
            frame: Frame::new(),
            vram: [0; 0x2000],
            bgmap_cache: None,
        }
    }

    pub fn tick(&mut self) {
        self.registers.tick();

        if self.registers.lcdc.lcd_en() {
            // Draw the line at the start of mode 3
            let mode = self.registers.ly.get_mode();
            if let VideoMode::Mode3 = mode && self.registers.ly.at_mode_start() {
                let scx = self.registers.scx.read();
                let scy = self.registers.scy.read();
                let ly = self.registers.ly.read();
                let bg_x = scx as usize;
                let bg_y = scy.wrapping_add(ly) as usize;
                let bg_line = self.get_bg_tilemap().get_line(bg_x, bg_y);
                self.frame.write_line(ly as usize, &bg_line);
            }

            // Mark the frame ready at the start of mode 1
            if self.registers.ly.at_vblank_start() {
                self.frame.set_ready(true);
            }
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        let src = AddrMap::from_addr(addr);

        if !self.vram_avail() && let AddrMap::Vram = src {
            return 0xFF;
        }
        
        match src {
            AddrMap::Vram => self.vram[(addr - 0x8000) as usize],
            AddrMap::RegLcdc => self.registers.lcdc.read(),
            AddrMap::RegStat => self.registers.stat.read(),
            AddrMap::RegScy => self.registers.scy.read(),
            AddrMap::RegScx => self.registers.scx.read(),
            AddrMap::RegLy => self.registers.ly.read(),
            AddrMap::RegBgp => self.registers.bgp.read(),
            AddrMap::Unmapped => panic!("Attempted VRAM read at 0x{:x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        let dst = AddrMap::from_addr(addr);

        if !self.vram_avail() && let AddrMap::Vram = dst {
            return;
        }
        
        match dst {
            AddrMap::Vram => self.vram[(addr - 0x8000) as usize] = val,
            AddrMap::RegLcdc => self.registers.lcdc.write(val),
            AddrMap::RegStat => self.registers.stat.write(val),
            AddrMap::RegScy => self.registers.scy.write(val),
            AddrMap::RegScx => self.registers.scx.write(val),
            AddrMap::RegLy => (),
            AddrMap::RegBgp => self.registers.bgp.write(val),
            AddrMap::Unmapped => panic!("Attempted VRAM write at 0x{:x}", addr),
        };

        // Clear the bgmap cache if we modified VRAM/LCDC
        match dst {
            AddrMap::Vram | AddrMap::RegLcdc => self.bgmap_cache = None,
            _ => (),
        };
    }

    // Tile functions
    pub fn get_obj_tile(&self, idx: u8) -> Tile {
        let mut pixels = [[Pixel::L0; 8]; 8];
        for i in 0..8usize {
            let addr = ((idx as u16) * 16) + ((i as u16) * 2);
            pixels[i] = self.parse_tile_line(addr);
        }
        Tile::new_obj(pixels)
    }

    pub fn get_bg_tile(&self, idx: u8) -> Tile {
        let base_addr = self.registers.lcdc.bg_tile_area().idx_addr(idx);
        let pixels = std::array::from_fn(|i| {
            self.parse_tile_line(base_addr + ((i as u16) * 2))
        });
        Tile::new_bgwin(pixels)
    }

    fn parse_tile_line(&self, addr: u16) -> [Pixel; 8] {
        let lsb = self.vram[addr as usize];
        let msb = self.vram[addr.wrapping_add(1) as usize];
        std::array::from_fn(|i| {
            let l = (lsb >> (7 - i)) & 1;
            let h = (msb >> (7 - i)) & 1;
            Pixel::from((h << 1) | l)
        })
    }

    // Tile map functions
    fn get_bg_tilemap(&mut self) -> TileMap {
        if let Some(map) = self.bgmap_cache {
            map.clone()
        } else {
            let base_addr = self.registers.lcdc.bg_tile_map().base_addr();
            let tiles = std::array::from_fn(|y| std::array::from_fn(|x| {
                let idx = self.vram[(base_addr as usize) + (y*32) + x];
                self.get_bg_tile(idx)
            }));
            self.bgmap_cache = Some(TileMap::new(tiles));
            TileMap::new(tiles)
        }
    }

    // Rendering functions
    pub fn get_frame_buf(&mut self) -> Option<Vec<u32>> {
        if self.frame.is_ready() {
            self.frame.set_ready(false);
            Some(self.frame.to_buf())
        } else {
            None
        }
    }

    // Memory availability functions
    fn vram_avail(&self) -> bool {
        if !self.registers.lcdc.lcd_en() {
            true
        } else {
            match self.registers.ly.get_mode() {
                VideoMode::Mode3 => false,
                _ => true,
            }
        }
    }
}

enum AddrMap {
    Vram,
    RegLcdc,
    RegStat,
    RegScy,
    RegScx,
    RegLy,
    RegBgp,
    Unmapped
}

impl AddrMap {
    fn from_addr(addr: u16) -> Self {
        match addr {
            0x8000..=0x9FFF => Self::Vram,
            0xFF40 => Self::RegLcdc,
            0xFF41 => Self::RegStat,
            0xFF42 => Self::RegScy,
            0xFF43 => Self::RegScx,
            0xFF44 => Self::RegLy,
            0xFF47 => Self::RegBgp,
            _ => Self::Unmapped,
        }
    }
}
