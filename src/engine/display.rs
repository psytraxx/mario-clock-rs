use super::GFXfont;

pub struct Display {}

impl Display {
    pub fn new() -> Self {
        Display {}
    }

    pub fn draw_rgb_bitmap(&self, x: i32, y: i32, _image: &'static [u16], width: i32, height: i32) {
        println!(
            "Drawing RGB bitmap at ({}, {}) with width {} and height {}",
            x, y, width, height
        );
    }
    pub fn fill_rect(&self, x: i32, y: i32, width: i32, height: i32, color: u16) {
        println!(
            "Filling rectangle at ({}, {}) with width {} and height {} and color {}",
            x, y, width, height, color
        );
    }
    pub fn set_text_color(&self, color: u16) {
        println!("Setting text color to {}", color);
    }
    pub fn set_cursor(&self, x: i32, y: i32) {
        println!("Setting cursor to ({}, {})", x, y);
    }
    pub fn print(&self, text: &str) {
        println!("Printing text: {}", text);
    }
    pub fn set_font(&self, font: &GFXfont<'static>) {
        println!("Setting font to {:?}", font);
    }
}
