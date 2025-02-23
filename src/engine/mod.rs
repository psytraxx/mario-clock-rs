use std::{
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::broadcast::{self, Receiver, Sender};

// Module declarations
pub mod display;
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
    pub name: String,
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

// Event system types
pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

// Utility functions
pub fn millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn create_event_channel() -> (EventSender, EventReceiver) {
    broadcast::channel(100)
}

// Core sprite trait
pub trait Sprite: Send + Sync {
    // Required properties
    fn x(&self) -> i8;
    fn y(&self) -> i8;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
    fn name(&self) -> &str;

    // Event system methods
    fn subscribe(&mut self, rx: EventReceiver, tx: EventSender);
    fn get_sender(&self) -> Option<EventSender>;

    // Default implementations
    fn publish_event(&self, event: Event) {
        if let Some(sender) = self.get_sender() {
            match sender.send(event) {
                Ok(_) => {}
                Err(e) => eprintln!("Error sending event: {:?}", e),
            }
        }
    }

    fn collided_with(&self, sprite: &SpriteInfo) -> bool {
        self.x() < sprite.x + sprite.width as i8
            && self.x() + self.width() as i8 > sprite.x
            && self.y() < sprite.y + sprite.height as i8
            && self.y() + self.height() as i8 > sprite.y
    }

    fn get_info(&self) -> SpriteInfo {
        SpriteInfo {
            name: self.name().to_string(),
            x: self.x(),
            y: self.y(),
            width: self.width(),
            height: self.height(),
        }
    }
}

// Font related structs
#[derive(Debug, Clone)]
pub struct GFXfont<'a> {
    pub bitmap: &'a [u8],
    pub glyph: &'a [GFXglyph],
    pub first: u8,
    pub last: u8,
    pub y_advance: u8,
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
