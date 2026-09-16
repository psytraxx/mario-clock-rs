use chrono::Timelike;
use static_cell::StaticCell;

use crate::{
    COLS, DISPLAY_TIMEZONE, FBType, I2CType, ROWS,
    clock::Clock,
    display::fill_rect,
    engine::{millis, object::Object, tile::Tile},
};

use super::gfx::{
    assets::{BUSH, GROUND, HILL, SKY_COLOR},
    block::Block,
    generate_cloud,
    mario::Mario,
};

// --- Constants ---
/// Wall-clock interval between cloud movements. Time-based rather than
/// frame-based so cloud speed does not vary with render load or with changes
/// to PLANES / pixel clock.
const CLOUD_MOVE_INTERVAL_MS: u64 = 120;
const CLOUD_PIXELS_PER_MOVE: i32 = 1; // Pixels to move the cloud when it moves
const CLOUD1_WIDTH: usize = 24;
const CLOUD1_HEIGHT: usize = 13;
const CLOUD2_WIDTH: usize = 16;
const CLOUD2_HEIGHT: usize = 13;
const INITIAL_CLOUD1_SEED: u64 = 1;
const INITIAL_CLOUD2_SEED: u64 = 2;

/// Sentinel minute, outside 0..=59, so the first minute change always jumps.
const NO_MINUTE: u32 = u32::MAX;

pub(crate) struct Clockface {
    ground: Tile,
    bush: Object,
    cloud1: Object,
    cloud2: Object,
    hill: Object,
    mario: Mario,
    hour_block: Block,
    minute_block: Block,
    // Cloud positions
    cloud1_x: i32,
    cloud2_x: i32,
    // Timestamp of the last cloud movement
    last_cloud_move_millis: u64,
    // Seeds for cloud generation
    cloud1_seed: u64,
    cloud2_seed: u64,
    /// Scratch buffer for cloud generation, reused to keep 1 KB arrays off the
    /// task stack.
    cloud_scratch: [u16; 512],
    /// Last minute on which Mario jumped, so he jumps once per minute.
    last_jump_minute: u32,
}

impl Clockface {
    /// Builds the clock face directly into `cell`.
    ///
    /// The struct carries several kilobytes of sprite buffers, which is more
    /// than the display task's stack can hold, so it is constructed in place
    /// in static storage rather than returned by value.
    pub fn init(cell: &'static StaticCell<Clockface>) -> &'static mut Clockface {
        let this = cell.init(Self {
            ground: Tile::new(GROUND, 8, 8),
            bush: Object::new(BUSH, 21, 9),
            // Clouds start empty and are generated in place below, so no
            // full-size sprite buffer is ever built on the stack.
            cloud1: Object::empty(CLOUD1_WIDTH as i32, CLOUD1_HEIGHT as i32),
            cloud2: Object::empty(CLOUD2_WIDTH as i32, CLOUD2_HEIGHT as i32),
            hill: Object::new(HILL, 20, 22),
            mario: Mario::new(23, 40),
            hour_block: Block::new(13, 8),
            minute_block: Block::new(32, 8),
            // Initial cloud positions
            cloud1_x: 0,  // Start cloud1 near the left
            cloud2_x: 51, // Start cloud2 further right
            last_cloud_move_millis: 0,
            cloud1_seed: INITIAL_CLOUD1_SEED,
            cloud2_seed: INITIAL_CLOUD2_SEED,
            cloud_scratch: [0u16; 512],
            last_jump_minute: NO_MINUTE,
        });

        let seed = this.cloud1_seed;
        Self::regenerate_cloud(
            &mut this.cloud_scratch,
            &mut this.cloud1,
            CLOUD1_WIDTH,
            CLOUD1_HEIGHT,
            5,
            seed,
        );

        let seed = this.cloud2_seed;
        Self::regenerate_cloud(
            &mut this.cloud_scratch,
            &mut this.cloud2,
            CLOUD2_WIDTH,
            CLOUD2_HEIGHT,
            5,
            seed,
        );

        this
    }

    pub fn now() -> chrono::DateTime<chrono_tz::Tz> {
        Clock::<I2CType>::get_time_in_zone(DISPLAY_TIMEZONE)
    }

    /// Moves a cloud left by one step, returning true if it wrapped off-screen.
    fn step_cloud(x: &mut i32, width: i32) -> bool {
        *x -= CLOUD_PIXELS_PER_MOVE;
        if *x + width < 0 {
            *x = COLS as i32;
            return true;
        }
        false
    }

    /// Regenerates a cloud's pixels in place, reusing the scratch buffer and
    /// the object's existing allocation.
    fn regenerate_cloud(
        scratch: &mut [u16; 512],
        cloud: &mut Object,
        width: usize,
        height: usize,
        circles: u8,
        seed: u64,
    ) {
        if generate_cloud(scratch, width, height, circles, seed) {
            cloud.set_sprite(&scratch[..width * height]);
        }
    }

    pub fn update(&mut self, fb: &mut FBType) {
        let now_millis = millis();

        // --- 1. Clear Background ---
        fill_rect(fb, 0, 0, ROWS as u32, COLS as u32, SKY_COLOR);

        // --- 2. Update Cloud Positions & Regenerate clouds if needed ---
        if now_millis.saturating_sub(self.last_cloud_move_millis) >= CLOUD_MOVE_INTERVAL_MS {
            self.last_cloud_move_millis = now_millis;

            if Self::step_cloud(&mut self.cloud1_x, CLOUD1_WIDTH as i32) {
                // Derive the next seed from the previous one so successive
                // clouds differ without depending on frame timing.
                self.cloud1_seed = self.cloud1_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let circles = 4 + (self.cloud1_seed >> 33) as u8 % 5;
                Self::regenerate_cloud(
                    &mut self.cloud_scratch,
                    &mut self.cloud1,
                    CLOUD1_WIDTH,
                    CLOUD1_HEIGHT,
                    circles,
                    self.cloud1_seed,
                );
            }

            if Self::step_cloud(&mut self.cloud2_x, CLOUD2_WIDTH as i32) {
                self.cloud2_seed = self.cloud2_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let circles = 3 + (self.cloud2_seed >> 33) as u8 % 5;
                Self::regenerate_cloud(
                    &mut self.cloud_scratch,
                    &mut self.cloud2,
                    CLOUD2_WIDTH,
                    CLOUD2_HEIGHT,
                    circles,
                    self.cloud2_seed,
                );
            }
        }

        // --- 3. Draw Static Background Elements ---
        self.ground.fill_row(COLS as i32 - self.ground.height(), fb);
        self.bush.draw(43, 47, fb);
        self.hill.draw(0, 34, fb);

        // --- 4. Draw Moving Clouds ---
        self.cloud1.draw(self.cloud1_x, 21, fb);
        self.cloud2.draw(self.cloud2_x, 7, fb);

        // --- 5. Update Time and Interactive Elements ---
        let now = Self::now();

        // Jump once per minute, on the transition into second 0.
        let current_minute = now.minute();
        let jump = if current_minute != self.last_jump_minute && now.second() == 0 {
            self.last_jump_minute = current_minute;
            true
        } else {
            false
        };

        // Advance Mario, then resolve collisions against the blocks directly.
        // This replaces the previous pub/sub channel, which delivered at most
        // one event per frame and so could drop a collision.
        if self.mario.advance(jump) && self.mario.is_rising() {
            let mario_info = self.mario.info();
            let mut hit = false;

            if mario_info.collides_with(&self.hour_block.info()) {
                self.hour_block.trigger_hit_animation();
                hit = true;
            }
            if mario_info.collides_with(&self.minute_block.info()) {
                self.minute_block.trigger_hit_animation();
                hit = true;
            }

            if hit {
                self.mario.bounce_off_block();
            }
        }

        self.mario.draw(fb);
        self.hour_block.update(fb, now.hour());
        self.minute_block.update(fb, now.minute());
    }
}
