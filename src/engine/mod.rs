use embassy_time::Instant;

pub mod object;
pub mod tile;

// Type definitions and basic structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
}

/// A sprite's position and extent, used for collision tests.
///
/// Coordinates are `i32` throughout: the display is small enough that `i8`
/// would fit today, but `x + width` in `collides_with` would silently wrap
/// under `overflow-checks = false` if artwork ever moved past 127.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpriteInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl SpriteInfo {
    /// Axis-aligned bounding-box overlap test.
    pub fn collides_with(&self, other: &SpriteInfo) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

// Utility functions
pub(crate) fn millis() -> u64 {
    Instant::now().as_millis()
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
