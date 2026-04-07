use crate::{FBType, display::draw_rgb_bitmap};
use heapless::Vec;

const MAX_SPRITE_SIZE: usize = 512;

/// Objects are used to represent interactive elements within the
/// game world, such as characters, items, or obstacles.
pub(crate) struct Object {
    sprite_data: Vec<u16, MAX_SPRITE_SIZE>,
    width: i32,
    height: i32,
}

impl Object {
    pub fn new(sprite_slice: &[u16], width: i32, height: i32) -> Self {
        let mut sprite_data = Vec::new();
        sprite_data
            .extend_from_slice(sprite_slice)
            .expect("Sprite data too large for Object Vec capacity");

        Self {
            sprite_data,
            width,
            height,
        }
    }

    pub fn draw(&self, x: i32, y: i32, fb: &mut FBType) {
        let data = self.sprite_data.as_slice();
        draw_rgb_bitmap(fb, x, y, data, self.width, self.height);
    }
}
