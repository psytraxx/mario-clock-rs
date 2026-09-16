use crate::{FBType, display::draw_rgb_bitmap};
use heapless::Vec;

const MAX_SPRITE_SIZE: usize = 512;

/// Objects are used to represent interactive elements within the
/// game world, such as characters, items, or obstacles.
///
/// Pixels stay in their source RGB565 form: at 2 bytes each this keeps a
/// full-size sprite to 1 KB. Widening to a pre-converted `Rgb888` would double
/// that, and four of these plus a scratch buffer must fit the display task's
/// stack.
pub(crate) struct Object {
    sprite_data: Vec<u16, MAX_SPRITE_SIZE>,
    width: i32,
    height: i32,
}

impl Object {
    pub fn new(sprite_slice: &[u16], width: i32, height: i32) -> Self {
        let mut object = Self {
            sprite_data: Vec::new(),
            width,
            height,
        };
        object.set_sprite(sprite_slice);
        object
    }

    /// Creates an object with no pixels yet; fill it with `set_sprite`.
    ///
    /// Lets a caller build the object in its final location and generate the
    /// sprite in place, instead of passing a full-size buffer through the stack.
    pub fn empty(width: i32, height: i32) -> Self {
        Self {
            sprite_data: Vec::new(),
            width,
            height,
        }
    }

    /// Replaces the sprite data in place, reusing the existing buffer.
    pub fn set_sprite(&mut self, sprite_slice: &[u16]) {
        self.sprite_data.clear();
        self.sprite_data
            .extend_from_slice(sprite_slice)
            .expect("Sprite data too large for Object Vec capacity");
    }

    pub fn draw(&self, x: i32, y: i32, fb: &mut FBType) {
        draw_rgb_bitmap(fb, x, y, &self.sprite_data, self.width, self.height);
    }
}
