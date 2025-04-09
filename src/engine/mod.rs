use crate::mario::gfx::font::SUPER_MARIO_BROS_24PT;
use crate::{FBType, GRID_SIZE};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    Drawable, Pixel,
};

pub mod object;
pub mod tile;

// Type definitions and basic structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    //Left,
    //Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteInfo {
    pub name: &'static str,
    pub x: i8,
    pub y: i8,
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    Move(SpriteInfo),
    Collision(SpriteInfo),
}

// Utility functions
pub fn millis() -> u64 {
    /*    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64 */
    0
}

// Helper function to convert RGB565 to RGB888
#[inline]
pub fn rgb565_to_rgb888(color: u16) -> Rgb888 {
    // Extract 5-bit red, 6-bit green, and 5-bit blue.
    let r5 = ((color >> 11) & 0x1F) as u8;
    let g6 = ((color >> 5) & 0x3F) as u8;
    let b5 = (color & 0x1F) as u8;
    let rgb565: Rgb565 = Rgb565::new(r5, g6, b5);

    rgb565.into()
}

pub fn draw_rgb_bitmap(
    fb: &mut FBType,
    x: i32,
    y: i32,
    image: &'static [u16],
    width: i32,
    height: i32,
) {
    let image_width = width as usize;
    let image_height = height as usize;
    let start_point = Point::new(x, y);

    // Iterate over the pixels of the image
    for row in 0..image_height {
        for col in 0..image_width {
            let pixel_index = row * image_width + col;
            // Basic bounds check for the source image data
            if pixel_index >= image.len() {
                continue;
            }

            let rgb565_color = image[pixel_index];
            // Simple transparency: skip black pixels (adjust if needed, 0x0000 is black in RGB565)
            if rgb565_color == 0 {
                continue;
            }

            // Use the helper from this module
            let rgb888_color = rgb565_to_rgb888(rgb565_color);

            // Calculate the target point on the framebuffer
            let target_point = start_point + Point::new(col as i32, row as i32);

            // Draw the single pixel if it's within the framebuffer bounds
            if target_point.x >= 0
                && target_point.x < GRID_SIZE as i32
                && target_point.y >= 0
                && target_point.y < GRID_SIZE as i32
            {
                Pixel(target_point, rgb888_color).draw(fb).ok(); // Ignore errors
            }
        }
    }
}

pub fn fill_rect(fb: &mut FBType, x: i32, y: i32, width: u32, height: u32, color: Rgb888) {
    let start_point = Point::new(x, y);
    let size = Size::new(width, height);
    let style = PrimitiveStyleBuilder::new().fill_color(color).build();
    Rectangle::new(start_point, size)
        .into_styled(style)
        .draw(fb)
        .expect("Failed to draw rectangle");
}

pub fn print(fb: &mut FBType, text: &str, x: i32, y: i32, color: Rgb888) {
    let font = SUPER_MARIO_BROS_24PT;
    let mut cursor_x = x;
    let cursor_y = y;

    for c in text.chars() {
        if c < font.first as char || c > font.last as char {
            continue; // Skip characters not in the font
        }

        let glyph_index = c as usize - font.first as usize;
        let glyph = &font.glyph[glyph_index];
        let bitmap = &font.bitmap[glyph.bitmap_offset as usize..];

        for row in 0..glyph.height {
            let dest_y = cursor_y + glyph.y_offset as i32 + row as i32;
            if dest_y < 0 || dest_y >= GRID_SIZE as i32 {
                continue;
            }

            let bitmap_row_start = (row * glyph.width) as usize;

            for col in 0..glyph.width {
                let dest_x = cursor_x + glyph.x_offset as i32 + col as i32;
                if dest_x < 0 || dest_x >= GRID_SIZE as i32 {
                    continue;
                }

                // TODO: This needs access to the framebuffer (fb) directly, not self.pixel_buffer
                // Assuming fb is a type that allows direct pixel access or has a draw method
                let bit_index = bitmap_row_start + col as usize;
                if bitmap[bit_index / 8] & (0x80 >> (bit_index % 8)) != 0 {
                    let target_point = Point::new(dest_x, dest_y);
                    Pixel(target_point, color).draw(fb).ok(); // Draw pixel using fb
                }
            }
        }

        cursor_x += glyph.x_advance as i32;
    }
}

// Core sprite trait
pub trait Sprite: Send + Sync {
    // Required properties
    fn x(&self) -> i8;
    fn y(&self) -> i8;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
    fn name(&self) -> &'static str;

    // Event system methods
    fn subscribe(
        &mut self,
        tx: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
        rx: Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
    );

    fn collided_with(&self, sprite: &SpriteInfo) -> bool {
        self.x() < sprite.x + sprite.width as i8
            && self.x() + self.width() as i8 > sprite.x
            && self.y() < sprite.y + sprite.height as i8
            && self.y() + self.height() as i8 > sprite.y
    }

    fn get_info(&self) -> SpriteInfo {
        SpriteInfo {
            name: self.name(),
            x: self.x(),
            y: self.y(),
            width: self.width(),
            height: self.height(),
        }
    }
}

pub mod font {
    // Font related structs
    #[derive(Debug, Clone)]
    pub struct GFXfont<'a> {
        pub bitmap: &'a [u8],
        pub glyph: &'a [GFXglyph],
        pub first: u8,
        pub last: u8,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct GFXglyph {
        pub bitmap_offset: u16,
        pub width: u8,
        pub height: u8,
        pub x_advance: u8,
        pub x_offset: i8,
        pub y_offset: i8,
    }
}
