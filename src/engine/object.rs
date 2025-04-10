use crate::{display::draw_rgb_bitmap, FBType};

/// Objects are used to represent interactive elements within the
/// game world, such as characters, items, or obstacles.
pub(crate) struct Object {
    image: &'static [u16],
    width: i32,
    height: i32,
}

impl Object {
    pub fn new(image: &'static [u16], width: i32, height: i32) -> Self {
        Object {
            image,
            width,
            height,
        }
    }

    pub fn draw(&self, x: i32, y: i32, fb: &mut FBType) {
        draw_rgb_bitmap(fb, x, y, self.image, self.width, self.height);
    }
}
