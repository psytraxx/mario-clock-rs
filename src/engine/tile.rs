use crate::{engine::draw_rgb_bitmap, FBType, GRID_SIZE};

/// Tiles are used to represent static elements of the game world,
/// such as the ground, walls, or other background elements.
pub struct Tile {
    image: &'static [u16],
    width: i32,
    height: i32,
}

impl Tile {
    pub fn new(image: &'static [u16], width: i32, height: i32) -> Self {
        Tile {
            image,
            width,
            height,
        }
    }

    pub fn fill_row(&self, y: i32, fb: &mut FBType) {
        for x in (0..GRID_SIZE).step_by(self.width as usize) {
            draw_rgb_bitmap(fb, x as i32, y, self.image, self.width, self.height);
        }
    }

    pub(crate) fn height(&self) -> i32 {
        self.height
    }
}
