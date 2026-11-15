mod registers;
mod pixel;
mod tile;
mod frame;

use rand;

use crate::hardware::error::{HardwareError, Result};

use registers::RegisterFile;
use pixel::Pixel;
use tile::Tile;
use frame::Frame;

pub struct Ppu {
    registers: RegisterFile,
    frame: Frame,
    vram: [u8; 0x2000],
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: RegisterFile::new(),
            frame: Frame::new(),
            vram: [0; 0x2000],
        }
    }

    pub fn tick(&mut self) {
        // DEBUG: Update the LCD line we're on
        if self.registers.ly.read() < 144 && self.registers.ly.at_line_start() {
            //self.frame.write_line(self.reg_ly.read() as usize, &[Pixel::L3; 160]);
            self.frame.randomize_line(self.registers.ly.read() as usize);
            let test_tile = self.get_obj_tile(25).unwrap();
            self.frame.draw_obj_tile(test_tile, 0, 0);
        }
        
        self.registers.tick();
        
        if self.registers.ly.at_vblank_start() {
            self.frame.set_ready(true);
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFF40 => self.registers.lcdc.read(),
            0xFF41 => self.registers.stat.read(),
            0xFF42 => self.registers.scy.read(),
            0xFF43 => self.registers.scx.read(),
            0xFF44 => self.registers.ly.read(),
            0xFF47 => self.registers.bgp.read(),
            other => panic!("Attempted VRAM read at 0x{:x}", other),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFF40 => self.registers.lcdc.write(val),
            0xFF41 => self.registers.stat.write(val),
            0xFF42 => self.registers.scy.write(val),
            0xFF43 => self.registers.scx.write(val),
            0xFF44 => (),
            0xFF47 => self.registers.bgp.write(val),
            other => panic!("Attempted VRAM write at 0x{:x}", other),
        }
    }

    // Tile functions
    pub fn get_obj_tile(&self, idx: u8) -> Result<Tile> {
        let mut pixels = [[Pixel::L0; 8]; 8];
        for i in 0..8usize {
            let addr = ((idx as u16) * 16) + ((i as u16) * 2);
            pixels[i] = self.parse_tile_line(addr)?;
        }
        Ok(Tile::new_obj(pixels))
    }

    fn parse_tile_line(&self, addr: u16) -> Result<[Pixel; 8]> {
        let lsb = self.vram[addr as usize];
        let msb = self.vram[addr.wrapping_add(1) as usize];
        let mut result = [Pixel::L0; 8];
        for i in 0..8 {
            let l = (lsb >> (7 - i)) & 1;
            let h = (msb >> (7 - i)) & 1;
            let pixel_bits = (h << 1) | l;
            result[i] = Pixel::try_from(pixel_bits)?;
        }
        Ok(result)
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
}
