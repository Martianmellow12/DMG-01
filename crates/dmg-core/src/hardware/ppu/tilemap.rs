use super::tile::Tile;
use super::pixel::Pixel;

#[derive(Debug, Copy, Clone)]
pub struct TileMap([[Tile; 32]; 32]);

impl TileMap {
    pub fn new(tiles: [[Tile; 32]; 32]) -> Self {
        Self(tiles)
    }

    pub fn get_line(&self, x_pos: usize, y_pos: usize) -> [Pixel; 160] {
        std::array::from_fn(|i| {
            self.get_pixel(x_pos + i, y_pos)
        })
    }

    fn get_pixel(&self, x_pos: usize, y_pos: usize) -> Pixel {
        let x_pos = x_pos % 256;
        let y_pos = y_pos % 256;
        let tile = self.0[y_pos/8][x_pos/8].get_tilemap();
        tile[y_pos%8][x_pos%8]
    }
}
