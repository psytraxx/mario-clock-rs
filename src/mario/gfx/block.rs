use super::assets::{BLACK, BLOCK};
use crate::{
    display::{draw_rgb_bitmap, print_text},
    engine::{millis, Direction, Event, Sprite},
    FBType,
};
use core::fmt::Write;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};
use heapless::String;

// --- Constants ---
const MOVE_PACE: i32 = 2; // Pixels the block moves per animation frame when hit
const MAX_MOVE_HEIGHT: i32 = 4; // Maximum height the block moves upwards when hit
const ANIMATION_INTERVAL_MS: u64 = 60; // Milliseconds between animation frames

// --- State ---
#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle, // Block is stationary
    Hit,  // Block is animating upwards and downwards after being hit
}

// --- Block Struct ---
pub(crate) struct Block {
    // Position and Dimensions
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    // State and Animation
    state: State,
    direction: Direction, // Direction of movement during Hit animation (Up/Down)
    start_y: i32,         // Original Y position, used to return after animation
    last_animation_millis: u64, // Timestamp of the last animation update

    // Displayed Text (stack-allocated with heapless to avoid heap allocation)
    text: String<4>, // Max 4 characters, stored on stack (zero heap usage)

    // Event Handling (Pub/Sub)
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
}

impl Block {
    /// Creates a new Block instance at the given coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Block {
            x,
            y,
            width: 19,  // Assuming BLOCK asset width
            height: 19, // Assuming BLOCK asset height
            state: State::Idle,
            direction: Direction::Up, // Initial direction for hit animation
            start_y: y,               // Store the initial Y position
            last_animation_millis: 0,
            text: String::new(), // Initialize empty (stack-allocated, no heap)
            rx: None,
            tx: None,
        }
    }

    /// Formats a u32 value to 2-digit text without heap allocation
    /// Uses heapless::String for ergonomic API with zero heap usage
    /// Values 0-99 are displayed as "00"-"99"
    /// Values >= 100 are displayed as "??"
    fn set_text_from_u32(&mut self, value: u32) {
        self.text.clear();

        if value <= 99 {
            // Format with leading zero, using core::fmt::Write
            // This writes directly to the stack-allocated String buffer
            write!(&mut self.text, "{:02}", value).expect("Format failed - buffer too small");
        } else {
            // Value out of range
            self.text
                .push_str("??")
                .expect("Push failed - buffer too small");
        }
    }

    /// Sets the block's state to Idle and resets its position.
    fn set_idle_state(&mut self) {
        if self.state != State::Idle {
            self.state = State::Idle;
            self.y = self.start_y; // Reset to original position
        }
    }

    /// Initiates the Hit animation sequence.
    fn trigger_hit_animation(&mut self) {
        if self.state != State::Hit {
            self.state = State::Hit;
            self.direction = Direction::Up; // Start moving up
            self.last_animation_millis = millis(); // Reset animation timer
        }
    }

    /// Helper function to draw the text centered on the block.
    fn draw_text_on_block(&self, fb: &mut FBType) {
        // Basic centering logic (adjust offsets as needed for the font)
        let text_x = if self.text.len() == 1 {
            self.x + 6 // Approx center for 1 char
        } else {
            self.x + 2 // Approx center for 2 chars
        };
        let text_y = self.y + 12; // Approx vertical center

        // heapless::String implements AsRef<str>, so we can pass it directly
        print_text(fb, self.text.as_str(), text_x, text_y, BLACK);
    }

    /// Updates the block's state, position, and draws it.
    /// Handles collision detection and animation.
    /// `current_value` is the number to display (e.g., current hour or minute).
    pub async fn update(&mut self, fb: &mut FBType, current_value: u32) {
        let current_millis = millis();
        let next_x = self.x; // X position doesn't change in this logic
        let mut next_y = self.y;

        // --- 1. Handle Incoming Events (Collision Detection) ---
        if let Some(rx) = &mut self.rx {
            if let Some(Event::Move(sprite_info)) = rx.try_next_message_pure() {
                // Check for collision with other sprites (e.g., Mario)
                if sprite_info.name != self.name() && self.collided_with(&sprite_info) {
                    // If collided, trigger the hit animation
                    self.trigger_hit_animation();
                    // Publish a collision event *from* the block
                    let info = self.get_info();
                    if let Some(tx) = &mut self.tx {
                        // Use non-blocking publish_immediate
                        tx.publish_immediate(Event::Collision(info));
                    }
                }
            }
        }

        // --- 2. Update Displayed Text ---
        // Convert value to 2-digit text without heap allocation
        self.set_text_from_u32(current_value);

        // --- 3. Update State and Position (Animation Logic) ---
        match self.state {
            State::Idle => {
                // In Idle state, position remains unchanged (self.start_y)
                next_y = self.start_y;
            }
            State::Hit => {
                // Animate only if enough time has passed
                if current_millis - self.last_animation_millis >= ANIMATION_INTERVAL_MS {
                    // Calculate next Y based on direction
                    next_y += MOVE_PACE
                        * if self.direction == Direction::Up {
                            -1
                        } else {
                            1
                        };

                    // Check if upward movement limit reached
                    if self.direction == Direction::Up && (self.start_y - next_y) >= MAX_MOVE_HEIGHT
                    {
                        self.direction = Direction::Down; // Start moving down
                    }

                    // Check if returned to start position while moving down
                    if self.direction == Direction::Down && next_y >= self.start_y {
                        next_y = self.start_y; // Snap to start position
                        self.set_idle_state(); // Transition back to Idle
                    }

                    self.last_animation_millis = current_millis; // Update animation timer
                } else {
                    // Not enough time passed, keep current position for this frame
                    next_y = self.y;
                }
            }
        }

        // --- 4. Update Position ---
        // Update the block's actual position
        // NOTE: Clearing the previous frame is omitted, assuming the main loop
        // clears the entire screen or handles background redraws.
        self.x = next_x;
        self.y = next_y;

        // --- 5. Draw Current Frame ---
        // Draw the block sprite at the current position
        draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
        // Draw the text on top of the block
        self.draw_text_on_block(fb);

        // --- 6. Publish Move Event (Optional) ---
        // If the block itself needed to notify others of its movement (unlikely here)
        // if position_changed {
        //     if let Some(tx) = &mut self.tx {
        //         tx.publish_immediate(Event::Move(self.get_info()));
        //     }
        // }
    }
}

// --- Sprite Trait Implementation ---
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
        // Consider unique names if multiple blocks exist, e.g., "HourBlock", "MinuteBlock"
        // Or pass an ID during creation. For now, using a generic name.
        "Block"
    }

    /// Subscribes the block to the event channel.
    fn subscribe(
        &mut self,
        tx: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
        rx: Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
    ) {
        self.rx = Some(rx);
        self.tx = Some(tx);
    }

    // get_info uses the default trait implementation based on x, y, width, height, name.
}
