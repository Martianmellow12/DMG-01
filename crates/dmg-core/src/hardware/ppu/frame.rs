use super::pixel::Pixel;
use super::tile::Tile;

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

    pub fn set_ready(&mut self, ready: bool) {
        self.frame_ready = ready;
    }

    pub fn is_ready(&self) -> bool {
        self.frame_ready
    }

    // Tile functions
    pub fn draw_obj_tile(&mut self, tile: Tile, x_pos: usize, y_pos: usize) {
        let tilemap = tile.get_tilemap();
        for y in 0..8 {
            for x in 0..8 {
                let y_pos = y_pos + y;
                let x_pos = x_pos + x;
                if y_pos >= 144 { return }
                if x_pos >= 160 { break }

                // Color 0 is transparent for objects
                match tilemap[y_pos][x_pos] {
                    Pixel::L0 => (), // Transparency for objects
                    other => self.frame_vec[y_pos][x_pos] = other,
                }
            }
        }
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
