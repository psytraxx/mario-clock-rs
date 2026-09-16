use crate::{
    FBType,
    display::draw_rgb_bitmap,
    engine::{Direction, SpriteInfo, millis},
};

use super::assets::{MARIO_IDLE, MARIO_IDLE_SIZE, MARIO_JUMP, MARIO_JUMP_SIZE};

// --- Constants ---
const MARIO_PACE: i32 = 3; // Pixels Mario moves per animation frame during jump
const MARIO_JUMP_HEIGHT: i32 = 14; // Maximum height Mario jumps in pixels
const JUMP_ANIMATION_INTERVAL_MS: u64 = 50; // Milliseconds between jump animation frames
const JUMP_DEBOUNCE_MS: u64 = 500; // Minimum time between jumps
const GROUND_Y: i32 = 56; // Y-coordinate considered as the ground level

// --- State ---
#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    Jumping,
}

// --- Mario Struct ---
pub(crate) struct Mario {
    // Position and Dimensions
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    // State and Animation
    state: State,
    direction: Direction,       // Used for jump direction (Up/Down)
    sprite: &'static [u16],     // Current sprite bitmap
    last_animation_millis: u64, // Timestamp of the last animation update
    jump_start_y: i32,          // Y position when the jump started
}

impl Mario {
    /// Creates a new Mario instance at the given coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Mario {
            x,
            y,
            width: MARIO_IDLE_SIZE[0] as i32,
            height: MARIO_IDLE_SIZE[1] as i32,
            state: State::Idle,
            direction: Direction::Up, // Default, relevant only during jump
            sprite: MARIO_IDLE,
            last_animation_millis: 0,
            jump_start_y: y,
        }
    }

    pub fn info(&self) -> SpriteInfo {
        SpriteInfo {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    /// Sends Mario back down, as when his head strikes a block.
    ///
    /// Deliberately does not require `direction == Up`: he reaches a block on
    /// the same step that hits the jump apex, by which point the direction has
    /// already flipped, so gating on it here would drop the bounce.
    pub fn bounce_off_block(&mut self) {
        if self.state == State::Jumping {
            self.direction = Direction::Down;
        }
    }

    /// Sets Mario's state to Idle and updates sprite properties.
    fn set_idle_state(&mut self) {
        if self.state != State::Idle {
            self.state = State::Idle;
            self.width = MARIO_IDLE_SIZE[0] as i32;
            self.height = MARIO_IDLE_SIZE[1] as i32;
            self.sprite = MARIO_IDLE;
            // Ensure Mario is exactly on the ground when idle
            self.y = GROUND_Y - self.height;
        }
    }

    /// Initiates the jump sequence.
    fn start_jump(&mut self) {
        if self.state == State::Idle
            && millis().saturating_sub(self.last_animation_millis) > JUMP_DEBOUNCE_MS
        {
            self.state = State::Jumping;
            self.width = MARIO_JUMP_SIZE[0] as i32;
            self.height = MARIO_JUMP_SIZE[1] as i32;
            self.sprite = MARIO_JUMP;
            self.direction = Direction::Up; // Start jumping upwards
            self.jump_start_y = self.y; // Record starting Y for height check
            self.last_animation_millis = millis(); // Reset timer for debounce and animation
        }
    }

    /// Advances Mario's state and position.
    ///
    /// Returns `true` if he moved *upwards* this step, i.e. this is a step on
    /// which his head could strike a block. Note that `direction` may already
    /// have flipped to `Down` by the time this returns, because the apex check
    /// runs in the same step as the move -- so callers must use this return
    /// value rather than inspecting the direction afterwards.
    pub fn advance(&mut self, trigger_jump: bool) -> bool {
        let current_millis = millis();
        let mut next_y = self.y;
        let mut position_changed = false;
        let mut moved_up = false;

        if trigger_jump {
            self.start_jump();
        }

        match self.state {
            State::Idle => {
                // No position change in Idle state
            }
            State::Jumping => {
                if current_millis.saturating_sub(self.last_animation_millis)
                    >= JUMP_ANIMATION_INTERVAL_MS
                {
                    let rising = self.direction == Direction::Up;
                    let y_change = MARIO_PACE * if rising { -1 } else { 1 };
                    next_y += y_change;
                    position_changed = y_change != 0;
                    moved_up = rising;

                    if self.direction == Direction::Up
                        && (self.jump_start_y - next_y) >= MARIO_JUMP_HEIGHT
                    {
                        self.direction = Direction::Down;
                    }

                    if self.direction == Direction::Down && (next_y + self.height) >= GROUND_Y {
                        self.set_idle_state();
                        next_y = self.y; // Use the Y set by set_idle_state
                        position_changed = true;
                    }

                    self.last_animation_millis = current_millis;
                } else {
                    // Not enough time passed for animation, keep current position
                    next_y = self.y;
                }
            }
        }

        if position_changed {
            self.y = next_y;
        }

        moved_up
    }

    /// Draws Mario at his current position.
    pub fn draw(&self, fb: &mut FBType) {
        draw_rgb_bitmap(fb, self.x, self.y, self.sprite, self.width, self.height);
    }
}
