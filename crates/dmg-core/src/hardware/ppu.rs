use crate::hardware::error::{HardwareError, Result};

use rand;

pub struct Ppu {
    frame: Frame,
    vram: [u8; 0x2000],
    reg_lcdc: u8,
    reg_stat: u8,
    reg_scy: u8,
    reg_scx: u8,
    reg_ly: LcdYCoord,
    reg_bgp: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            frame: Frame::new(),
            vram: [0; 0x2000],
            reg_lcdc: 0,
            reg_stat: 0,
            reg_scx: 0,
            reg_scy: 0,
            reg_ly: LcdYCoord::new(),
            reg_bgp: 0,
        }
    }

    pub fn tick(&mut self) {
        // Update the LCD line we're on
        if self.reg_ly.read() < 144 && self.reg_ly.incremented() {
            //self.frame.write_line(self.reg_ly.read() as usize, &[Pixel::L3; 160]);
            self.frame.randomize_line(self.reg_ly.read() as usize);
        }
        self.reg_ly.step();
        if self.reg_ly.read() == 144 && self.reg_ly.incremented() {
            self.frame.frame_ready = true;
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFF40 => self.reg_lcdc,
            0xFF41 => self.reg_stat,
            0xFF42 => self.reg_scy,
            0xFF43 => self.reg_scx,
            0xFF44 => self.reg_ly.read(),
            0xFF47 => self.reg_bgp,
            other => panic!("Attempted VRAM read at 0x{:x}", other),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFF40 => self.reg_lcdc = val,
            0xFF41 => self.reg_stat = val,
            0xFF42 => self.reg_scy = val,
            0xFF43 => self.reg_scx = val,
            0xFF44 => (),
            0xFF47 => self.reg_bgp = val,
            other => panic!("Attempted VRAM write at 0x{:x}", other),
        }
    }

    // Rendering functions
    pub fn get_frame_buf(&mut self) -> Option<Vec<u32>> {
        if self.frame.frame_ready {
            self.frame.frame_ready = false;
            Some(self.frame.to_buf())
        } else {
            None
        }
    }
}

struct LcdYCoord(usize);

impl LcdYCoord {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn step(&mut self) {
        self.0 += 1;
        if (self.0 / 114) > 153 {
            self.0 = 0;
        }
    }

    pub fn incremented(&self) -> bool {
        self.0 % 114 == 0
    }

    pub fn read(&self) -> u8 {
        self.read_ly() as u8
    }

    // Helper functions
    fn read_ly(&self) -> usize {
        (self.0 / 114)
    }
}

pub struct Frame {
    frame_vec: Vec<Vec<Pixel>>,
    frame_ready: bool,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            frame_vec: vec![vec![Pixel::L2; 160]; 144],
            frame_ready: false,
        }
    }

    pub fn write_line(&mut self, line_num: usize, line_data: &[Pixel; 160]) {
        self.frame_vec[line_num] = line_data.to_vec();
    }
    
    pub fn to_buf(&self) -> Vec<u32> {
        let mut buf = Vec::new();
        for row in self.frame_vec.iter() {
            for pixel in row {
                let val = match pixel {
                    Pixel::L0 => 0x00000000,
                    Pixel::L1 => 0x00555555,
                    Pixel::L2 => 0x00AAAAAA,
                    Pixel::L3 => 0x00FFFFFF,
                };
                buf.push(val);
            }
        }
        buf
    }

    // Debug functions
    pub fn randomize_line(&mut self, line_num: usize) {
        let rand_data: Vec<u8> = (0..160)
            .map(|_| rand::random_range(0..=3))
            .collect();
        for i in 0..self.frame_vec[line_num].len() {
            self.frame_vec[line_num][i] = match rand_data[i] {
                0 => Pixel::L0,
                _ => Pixel::L3,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pixel {
    L0,
    L1,
    L2,
    L3,
}
