use super::pixel::Pixel;

#[derive(Debug)]
pub enum Tile {
    Obj([[Pixel; 8]; 8]),
    BgWin([[Pixel; 8]; 8]),
}

impl Tile {
    pub fn new_obj(pixels: [[Pixel; 8]; 8]) -> Self {
        Self::Obj(pixels)
    }

    pub fn new_bgwin(pixels: [[Pixel; 8]; 8]) -> Self {
        Self::BgWin(pixels)
    }

    pub fn get_tilemap(&self) -> &[[Pixel; 8]; 8] {
        match self {
            Self::Obj(pixels) |
            Self::BgWin(pixels) => {
                pixels
            }
        }
    }
}
