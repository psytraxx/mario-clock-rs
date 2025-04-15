use crate::{
    display::draw_rgb_bitmap,
    engine::{millis, Direction, Event, Sprite},
    FBType,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
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

    // Event Handling (Pub/Sub)
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
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
            jump_start_y: y, // Initialize jump_start_y
            rx: None,
            tx: None,
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
        if self.state == State::Idle && (millis() - self.last_animation_millis > JUMP_DEBOUNCE_MS) {
            self.state = State::Jumping;
            self.width = MARIO_JUMP_SIZE[0] as i32;
            self.height = MARIO_JUMP_SIZE[1] as i32;
            self.sprite = MARIO_JUMP;
            self.direction = Direction::Up; // Start jumping upwards
            self.jump_start_y = self.y; // Record starting Y for height check
            self.last_animation_millis = millis(); // Reset timer for debounce and animation
        }
    }

    /// Updates Mario's state, position, and draws him on the framebuffer.
    /// `trigger_jump` indicates if a jump should be initiated this frame.
    pub async fn update(&mut self, fb: &mut FBType, trigger_jump: bool) {
        let current_millis = millis();
        let next_x = self.x;
        let mut next_y = self.y;
        let mut position_changed = false; // Track if position changes this frame

        // --- 1. Handle Incoming Events ---
        if let Some(rx) = &mut self.rx {
            if let Some(event) = rx.try_next_message_pure() {
                match event {
                    Event::Collision(t)
                        if t.name != self.name()
                            && self.state == State::Jumping
                            && self.direction == Direction::Up =>
                    {
                        self.direction = Direction::Down;
                    }
                    _ => {}
                }
            }
        }

        // --- 2. Handle Jump Initiation ---
        if trigger_jump {
            self.start_jump();
            // If jump started, dimensions might change, force redraw logic later
        }

        // --- 3. Update State and Position ---
        match self.state {
            State::Idle => {
                // No position change in Idle state
            }
            State::Jumping => {
                if current_millis - self.last_animation_millis >= JUMP_ANIMATION_INTERVAL_MS {
                    let y_change = MARIO_PACE
                        * if self.direction == Direction::Up {
                            -1
                        } else {
                            1
                        };
                    next_y += y_change;
                    position_changed = y_change != 0; // Position changed if y_change is non-zero

                    if self.direction == Direction::Up
                        && (self.jump_start_y - next_y) >= MARIO_JUMP_HEIGHT
                    {
                        self.direction = Direction::Down;
                    }

                    if self.direction == Direction::Down && (next_y + self.height) >= GROUND_Y {
                        self.set_idle_state();
                        next_y = self.y; // Use the Y set by set_idle_state
                                         // Position effectively changed back to ground level
                        position_changed = true;
                    }

                    self.last_animation_millis = current_millis;
                } else {
                    // Not enough time passed for animation, keep current position
                    next_y = self.y;
                    // position_changed remains false if no movement calculation happened
                }
            }
        }

        // --- 4. Update Position ---
        // Update Mario's actual position only if it changed
        if position_changed {
            self.x = next_x; // Although x doesn't change in this logic, keep for consistency
            self.y = next_y;
        }

        // --- 5. Draw Current Frame ---
        // Always draw Mario at his current position
        draw_rgb_bitmap(fb, self.x, self.y, self.sprite, self.width, self.height);

        // --- 6. Publish Move Event if Position Changed ---
        if position_changed {
            let info = self.get_info();
            if let Some(tx) = &mut self.tx {
                tx.publish_immediate(Event::Move(info));
            }
        }
    }
}

// --- Sprite Trait Implementation ---
impl Sprite for Mario {
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
        "MARIO"
    }

    /// Subscribes Mario to the event channel.
    fn subscribe(
        &mut self,
        tx: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
        rx: Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
    ) {
        self.rx = Some(rx);
        self.tx = Some(tx);
    }

    // get_info uses the default trait implementation
}
