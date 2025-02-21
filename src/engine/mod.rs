use std::time::{SystemTime, UNIX_EPOCH};

use game::Event;

pub mod display;
pub mod eventbus;
pub mod game;
pub mod object;
pub mod tile;

pub fn millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub trait Sprite: Send + Sync {
    // Required properties that implementing types must provide
    fn x(&self) -> i8;
    fn y(&self) -> i8;
    fn width(&self) -> u8;
    fn height(&self) -> u8;

    // Default implementations for common sprite behaviors
    fn collided_with(&self, sprite: &dyn Sprite) -> bool {
        self.x() < sprite.x() + sprite.width() as i8
            && self.x() + self.width() as i8 > sprite.x()
            && self.y() < sprite.y() + sprite.height() as i8
            && self.y() + self.height() as i8 > sprite.y()
    }

    fn log_position(&self) {
        println!(
            "x = {}, y = {}, w = {}, h = {}",
            self.x(),
            self.y(),
            self.width(),
            self.height()
        );
    }

    fn execute(&mut self, sender: &dyn Sprite, event: &Event);
    fn name(&self) -> &str;
}

/// Font metadata struct
#[derive(Debug, Clone)]
pub struct GFXfont<'a> {
    pub bitmap: &'a [u8],      // Bitmap data
    pub glyph: &'a [GFXglyph], // Glyph array
    pub first: u8,             // ASCII extents
    pub last: u8,
    pub y_advance: u8, // Newline distance
}

/// Glyph struct defining font character properties
#[derive(Debug, Clone, Copy)]
pub struct GFXglyph {
    pub bitmap_offset: u16, // Offset into bitmap array
    pub width: u8,          // Bitmap dimensions
    pub height: u8,
    pub x_advance: u8, // Distance to advance cursor
    pub x_offset: i8,  // X dist from cursor pos to UL corner
    pub y_offset: i8,  // Y dist from cursor pos to UL corner
}
