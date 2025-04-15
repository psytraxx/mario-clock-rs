use super::assets::{BLACK, BLOCK, SKY_COLOR};
use crate::{
    display::{draw_rgb_bitmap, fill_rect, print_text},
    engine::{millis, Direction, Event, Sprite, Updatable}, // Added Updatable
    FBType,
};
use alloc::{boxed::Box, format}; // Added Box, Use alloc::format for heapless String compatibility
use core::future::Future; // Added Future
use core::pin::Pin; // Added Pin
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};
use heapless::String;

const MOVE_PACE: u8 = 2;
const MAX_MOVE_HEIGHT: u8 = 4;

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    Hit,
}

pub(crate) struct Block {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    direction: Direction,
    text: String<2>,
    last_millis: u64,
    state: State,
    last_state: State,
    first_y: i32,
    last_y: i32,
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    id: &'static str, // Added ID to distinguish hour/minute blocks
}

impl Block {
    // Modified new to accept id
    pub fn new(x: i32, y: i32, id: &'static str) -> Self {
        Block {
            x,
            y,
            width: 19,
            height: 19,
            direction: Direction::Up,
            text: String::new(),
            last_millis: 0,
            state: State::Idle,
            last_state: State::Idle,
            first_y: y,
            last_y: y,
            rx: None,
            tx: None,
            id, // Store id
        }
    }

    // Removed init function

    // set_text remains the same
    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text).unwrap();
    }

    fn idle(&mut self) {
        if self.state != State::Idle {
            self.last_state = self.state;
            self.state = State::Idle;
            self.y = self.first_y;
        }
    }

    fn hit(&mut self) {
        if self.state != State::Hit {
            self.last_state = self.state;
            self.state = State::Hit;
            self.last_y = self.y;
            self.direction = Direction::Up;
        }
    }

    fn set_text_block(&self, fb: &mut FBType) {
        if self.text.len() == 1 {
            print_text(fb, &self.text, self.x + 6, self.y + 12, BLACK);
        } else {
            print_text(fb, &self.text, self.x + 2, self.y + 12, BLACK);
        }
    }

    // Removed old async update function
}

// Implement Updatable trait
impl<'fb> Updatable<'fb> for Block {
    // Change return type to Pin<Box<dyn Future>>
    fn update(
        &'fb mut self,
        fb: &'fb mut FBType,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fb>> {
        // Wrap the async block with Box::pin()
        Box::pin(async move {
            // Check for events first
            if let Some(rx) = &mut self.rx {
                // Use try_next_message_pure for non-blocking check on Subscriber
                if let Some(event) = rx.try_next_message_pure() {
                    // Corrected method call
                    match event {
                        Event::TimeUpdate { hour, minute } => {
                            // Update text based on block ID
                            let formatted_text = if self.id == "hour" {
                                format!("{:02}", hour)
                            } else {
                                // Assuming "minute"
                                format!("{:02}", minute)
                            };
                            self.set_text(&formatted_text);
                        }
                        Event::Move(sprite) => {
                            // Handle collision check from Move event
                            if sprite.name != self.name() && self.collided_with(&sprite) {
                                self.hit(); // Trigger hit state
                                let info = self.get_info();
                                if let Some(tx) = &mut self.tx {
                                    // Use non-blocking publish if possible, or handle potential block
                                    tx.publish_immediate(Event::Collision(info));
                                }
                            }
                        }
                        _ => {} // Ignore other events like Collision received by the block itself
                    }
                }
            }

            // Drawing and state logic (previously in update)
            if self.state == State::Idle {
                // Only redraw if state changed to Idle to avoid flicker,
                // or always draw if background isn't persistent
                // For full redraw loop, always draw:
                draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
                self.set_text_block(fb);
                // If we always redraw, last_state check might be redundant here
                // self.last_state = self.state;
            } else if self.state == State::Hit {
                // Handle Hit animation
                // Clear previous position only if needed (depends on full redraw)
                // With full redraw, clearing isn't strictly necessary here,
                // but might be needed if movement calculation relies on it.
                // Let's keep the fill_rect for now to ensure clean animation.
                if millis() - self.last_millis >= 60 {
                    // Animation timing
                    fill_rect(
                        fb,
                        self.x, // Use the *current* y before moving
                        self.y,
                        self.width as u32,
                        self.height as u32,
                        SKY_COLOR,
                    );

                    self.y += MOVE_PACE as i32
                        * if self.direction == Direction::Up {
                            -1
                        } else {
                            1
                        };

                    // Draw at new position
                    draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
                    self.set_text_block(fb); // Draw text at new position

                    // Check movement boundaries
                    if (self.first_y - self.y) >= MAX_MOVE_HEIGHT as i32 {
                        self.direction = Direction::Down;
                    }

                    // Check if animation finished
                    if self.y >= self.first_y && self.direction == Direction::Down {
                        self.y = self.first_y; // Ensure exact final position
                        self.idle(); // Return to idle state
                                     // Re-draw in idle state immediately after finishing hit animation
                        fill_rect(
                            fb,
                            self.x,
                            self.y + MOVE_PACE as i32,
                            self.width as u32,
                            self.height as u32,
                            SKY_COLOR,
                        ); // Clear last downward step pos
                        draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
                        self.set_text_block(fb);
                    }

                    self.last_millis = millis();
                } else {
                    // If not enough time passed for animation step, draw at current position
                    draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
                    self.set_text_block(fb);
                }
            }
        }) // Close Box::pin()
    }
}

impl Sprite for Block {
    fn x(&self) -> i8 {
        self.x as i8
    }

    fn y(&self) -> i8 {
        self.y as i8
    }

    fn width(&self) -> u8 {
        self.width as u8
    }

    fn height(&self) -> u8 {
        self.height as u8
    }

    fn name(&self) -> &'static str {
        "Block"
    }

    fn subscribe(
        &mut self,
        tx: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
        rx: Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
    ) {
        self.rx = Some(rx);
        self.tx = Some(tx);
    }
}
