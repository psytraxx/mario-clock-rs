use super::assets::{BLACK, BLOCK};
use crate::{
    FBType,
    display::{draw_rgb_bitmap, print_text},
    engine::{Direction, SpriteInfo, millis},
};
use core::fmt::Write;
use heapless::String;

// --- Constants ---
const MOVE_PACE: i32 = 2; // Pixels the block moves per animation frame when hit
const MAX_MOVE_HEIGHT: i32 = 4; // Maximum height the block moves upwards when hit
const ANIMATION_INTERVAL_MS: u64 = 60; // Milliseconds between animation frames
const BLOCK_WIDTH: i32 = 19;
const BLOCK_HEIGHT: i32 = 19;

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

    // State and Animation
    state: State,
    direction: Direction, // Direction of movement during Hit animation (Up/Down)
    start_y: i32,         // Original Y position, used to return after animation
    last_animation_millis: u64, // Timestamp of the last animation update

    // Displayed Text (stack-allocated with heapless to avoid heap allocation)
    text: String<4>, // Max 4 characters, stored on stack (zero heap usage)
}

impl Block {
    /// Creates a new Block instance at the given coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Block {
            x,
            y,
            state: State::Idle,
            direction: Direction::Up, // Initial direction for hit animation
            start_y: y,               // Store the initial Y position
            last_animation_millis: 0,
            text: String::new(), // Initialize empty (stack-allocated, no heap)
        }
    }

    pub fn info(&self) -> SpriteInfo {
        SpriteInfo {
            x: self.x,
            y: self.y,
            width: BLOCK_WIDTH,
            height: BLOCK_HEIGHT,
        }
    }

    /// Formats a u32 value to 2-digit text without heap allocation
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
    pub fn trigger_hit_animation(&mut self) {
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

        print_text(fb, self.text.as_str(), text_x, text_y, BLACK);
    }

    /// Updates the block's state, position, and draws it.
    /// `current_value` is the number to display (e.g., current hour or minute).
    pub fn update(&mut self, fb: &mut FBType, current_value: u32) {
        let current_millis = millis();
        let mut next_y = self.y;

        // --- 1. Update Displayed Text ---
        self.set_text_from_u32(current_value);

        // --- 2. Update State and Position (Animation Logic) ---
        match self.state {
            State::Idle => {
                // In Idle state, position remains unchanged (self.start_y)
                next_y = self.start_y;
            }
            State::Hit => {
                // Animate only if enough time has passed
                if current_millis.saturating_sub(self.last_animation_millis)
                    >= ANIMATION_INTERVAL_MS
                {
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

        // --- 3. Update Position ---
        // NOTE: Clearing the previous frame is omitted; the main loop clears
        // the entire screen each frame.
        self.y = next_y;

        // --- 4. Draw Current Frame ---
        draw_rgb_bitmap(fb, self.x, self.y, BLOCK, BLOCK_WIDTH, BLOCK_HEIGHT);
        self.draw_text_on_block(fb);
    }
}
