use crate::{
    clock::Clock, // Added Clock
    display::{draw_rgb_bitmap, fill_rect},
    engine::{millis, Direction, Event, Sprite, Updatable}, // Added Updatable
    FBType,
    I2CType, // Added I2CType for Clock
};
use alloc::boxed::Box; // Added Box
use chrono::Timelike;
use core::future::Future; // Added Future
use core::pin::Pin; // Added Pin
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};

use super::assets::{MARIO_IDLE, MARIO_IDLE_SIZE, MARIO_JUMP, MARIO_JUMP_SIZE, SKY_COLOR};

const MARIO_PACE: u8 = 3;
const MARIO_JUMP_HEIGHT: u8 = 14;

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    //Walking,
    Jumping,
}

pub(crate) struct Mario {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    direction: Direction,
    last_x: i32,
    last_y: i32,
    sprite: &'static [u16],
    last_millis: u64,
    state: State,
    last_state: State,
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
}

impl Mario {
    pub fn new(x: i32, y: i32) -> Self {
        Mario {
            x,
            y,
            width: MARIO_IDLE_SIZE[0] as i32,
            height: MARIO_IDLE_SIZE[1] as i32,
            direction: Direction::Up,
            last_x: x,
            last_y: y,
            sprite: MARIO_IDLE,
            last_millis: 0,
            state: State::Idle,
            last_state: State::Idle,
            rx: None,
            tx: None,
        }
    }

    // Removed init function

    /* pub fn move_sprite(&mut self, dir: Direction) {
           match dir {
               Direction::Right => self.x += MARIO_PACE as i32,
               Direction::Left => self.x -= MARIO_PACE as i32,
               _ => {}
           }
       }
    */
    // Removed jump function (logic moved to update)

    fn idle(&mut self, fb: &mut FBType) {
        if self.state != State::Idle {
            self.last_state = self.state;
            self.state = State::Idle;

            fill_rect(
                fb,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                SKY_COLOR,
            );

            self.width = MARIO_IDLE_SIZE[0] as i32;
            self.height = MARIO_IDLE_SIZE[1] as i32;
            self.sprite = MARIO_IDLE;
        }
    }

    // Removed old async update function
}

// Implement Updatable trait for Mario
impl<'fb> Updatable<'fb> for Mario {
    // Change return type to Pin<Box<dyn Future>>
    fn update(
        &'fb mut self,
        fb: &'fb mut FBType,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fb>> {
        // Wrap the async block with Box::pin()
        Box::pin(async move {
            // --- Jump Trigger Logic (moved from clockface.rs) ---
            // Get current time - requires access to Clock or passing time via event
            // Let's assume Clock is accessible statically here for simplicity
            // NOTE: This might need adjustment depending on how Clock is truly accessed globally
            let now = Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich);
            if now.second() % 10 == 0 {
                // Trigger jump state if not already jumping
                if self.state != State::Jumping && (millis() - self.last_millis > 500) {
                    // Reuse existing debounce logic
                    self.last_state = self.state;
                    self.state = State::Jumping;

                    // Clear previous sprite area (important for state change)
                    // Use last known width/height before changing state
                    fill_rect(
                        fb,
                        self.x,
                        self.y,
                        MARIO_IDLE_SIZE[0] as u32, // Use IDLE size for clearing
                        MARIO_IDLE_SIZE[1] as u32,
                        SKY_COLOR,
                    );

                    // Update sprite properties for jumping
                    self.width = MARIO_JUMP_SIZE[0] as i32;
                    self.height = MARIO_JUMP_SIZE[1] as i32;
                    self.sprite = MARIO_JUMP;
                    self.direction = Direction::Up;
                    self.last_y = self.y;
                    self.last_x = self.x;
                }
            }

            // --- Event Handling ---
            if let Some(rx) = &mut self.rx {
                // Use try_next_message_pure for non-blocking check on Subscriber
                if let Some(event) = rx.try_next_message_pure() {
                    // Corrected method call
                    match event {
                        Event::Collision(t) if t.name != self.name() => {
                            // If jumping and collided, change direction to Down
                            if self.state == State::Jumping {
                                self.direction = Direction::Down;
                            }
                        }
                        _ => {} // Ignore other events
                    }
                }
            }

            // --- State Machine & Drawing Logic ---
            if self.state == State::Idle {
                // Always draw in Idle state for full redraw loop
                draw_rgb_bitmap(
                    fb,
                    self.x,
                    self.y,
                    MARIO_IDLE,
                    MARIO_IDLE_SIZE[0] as i32,
                    MARIO_IDLE_SIZE[1] as i32,
                );
                // self.last_state = self.state; // Redundant if always drawing
            } else if self.state == State::Jumping {
                if millis() - self.last_millis >= 50 {
                    // Animation timing
                    // Clear previous frame's position
                    fill_rect(
                        fb,
                        self.x,
                        self.y,
                        self.width as u32,
                        self.height as u32,
                        SKY_COLOR,
                    );

                    // Update position based on direction
                    self.y += MARIO_PACE as i32
                        * if self.direction == Direction::Up {
                            -1
                        } else {
                            1
                        };

                    // Draw Mario at the new position
                    draw_rgb_bitmap(fb, self.x, self.y, self.sprite, self.width, self.height);

                    // Publish Move event
                    let info = self.get_info();
                    if let Some(tx) = &mut self.tx {
                        // Use non-blocking publish
                        tx.publish_immediate(Event::Move(info));
                    }

                    // Check jump height limit
                    if (self.last_y - self.y) >= MARIO_JUMP_HEIGHT as i32
                        && self.direction == Direction::Up
                    {
                        self.direction = Direction::Down;
                    }

                    // Check if landed (adjust landing condition if ground level changes)
                    // Assuming ground is around y=56 - height
                    if self.y + self.height >= 56 && self.direction == Direction::Down {
                        // Landed, transition back to Idle
                        self.y = 56 - MARIO_IDLE_SIZE[1] as i32; // Set precise idle Y
                        self.idle(fb); // Call idle to reset state and sprite properties

                        // Draw immediately in Idle state after landing
                        fill_rect(
                            fb,
                            self.x,
                            self.y + MARIO_PACE as i32,
                            self.width as u32,
                            self.height as u32,
                            SKY_COLOR,
                        ); // Clear last jump pos
                        draw_rgb_bitmap(
                            fb,
                            self.x,
                            self.y,
                            MARIO_IDLE,
                            MARIO_IDLE_SIZE[0] as i32,
                            MARIO_IDLE_SIZE[1] as i32,
                        );
                    }

                    self.last_millis = millis();
                } else {
                    // If not enough time passed for animation step, draw at current position
                    draw_rgb_bitmap(fb, self.x, self.y, self.sprite, self.width, self.height);
                }
            }
        }) // Close Box::pin()
    }
}

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

    fn subscribe(
        &mut self,
        tx: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
        rx: Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
    ) {
        self.rx = Some(rx);
        self.tx = Some(tx);
    }
}
