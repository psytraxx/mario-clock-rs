use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};

pub mod object;
pub mod tile;

// Type definitions and basic structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    //Left,
    //Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SpriteInfo {
    pub name: &'static str,
    pub x: i8,
    pub y: i8,
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Event {
    Move(SpriteInfo),
    Collision(SpriteInfo),
}

// Utility functions
pub(crate) fn millis() -> u64 {
    /*    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64 */
    0
}

// Core sprite trait
pub(crate) trait Sprite: Send + Sync {
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
    pub(crate) struct GFXfont<'a> {
        pub bitmap: &'a [u8],
        pub glyph: &'a [GFXglyph],
        pub first: u8,
        pub last: u8,
    }

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct GFXglyph {
        pub bitmap_offset: u16,
        pub width: u8,
        pub height: u8,
        pub x_advance: u8,
        pub x_offset: i8,
        pub y_offset: i8,
    }
}
