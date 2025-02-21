use super::display::Display;

pub struct Object {
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

    pub fn draw(&self, x: i32, y: i32) {
        Display::instance().lock().unwrap().draw_rgb_bitmap(
            x,
            y,
            self.image,
            self.width,
            self.height,
        );
    }
}
