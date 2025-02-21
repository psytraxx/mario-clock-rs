use crate::GRID_SIZE;

use super::display::Display;

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

    pub fn draw(&self, x: i32, y: i32) {
        Display::instance().lock().unwrap().draw_rgb_bitmap(
            x,
            y,
            self.image,
            self.width,
            self.height,
        );
    }

    pub fn fill_row(&self, y: i32) {
        for x in (0..GRID_SIZE).step_by(self.width as usize) {
            self.draw(x as i32, y);
        }
    }

    pub(crate) fn height(&self) -> i32 {
        self.height
    }
}
