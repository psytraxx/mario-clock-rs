use chrono::Timelike;
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use static_cell::StaticCell;

use crate::{
    clock::Clock,
    display::fill_rect,
    engine::{object::Object, tile::Tile, Event, Sprite},
    ClockfaceTrait, FBType, I2CType, COLS, ROWS,
};

use super::gfx::{
    assets::{BUSH, CLOUD2, GROUND, HILL, SKY_COLOR},
    block::Block,
    generate_cloud,
    mario::Mario,
};

static CHANNEL: StaticCell<PubSubChannel<CriticalSectionRawMutex, Event, 3, 4, 4>> =
    StaticCell::new();

// --- Constants ---
const CLOUD_MOVE_INTERVAL: u32 = 5; // Move cloud every X update cycles
const CLOUD_PIXELS_PER_MOVE: i32 = 1; // Pixels to move the cloud when it moves
const CLOUD1_WIDTH: usize = 24;
const CLOUD1_HEIGHT: usize = 13;
const INITIAL_CLOUD1_SEED: u64 = 1; // Define initial seed

// Track the last minute when Mario jumped to prevent multiple jumps per minute
// Initialized to 255 (invalid minute) to ensure first jump always triggers
static LAST_JUMP_MINUTE: AtomicU32 = AtomicU32::new(255);

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
    // Frame counter for slow movement
    frame_count: u32,
    // Seed for cloud1 generation
    cloud1_seed: u64,
}

impl Clockface {
    pub fn new() -> Self {
        let channel: &'static mut _ = CHANNEL.init(PubSubChannel::new());

        let mut mario = Mario::new(23, 40);
        mario.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let mut hour_block = Block::new(13, 8);
        hour_block.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let mut minute_block = Block::new(32, 8);
        minute_block.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let initial_seed = INITIAL_CLOUD1_SEED;
        let cloud1_array = generate_cloud(CLOUD1_WIDTH, CLOUD1_HEIGHT, 3, initial_seed)
            .expect("Failed to generate initial cloud");
        let cloud1_size = CLOUD1_WIDTH * CLOUD1_HEIGHT;

        Self {
            ground: Tile::new(GROUND, 8, 8),
            bush: Object::new(BUSH, 21, 9),
            cloud1: Object::new(
                &cloud1_array[0..cloud1_size],
                CLOUD1_WIDTH as i32,
                CLOUD1_HEIGHT as i32,
            ),
            cloud2: Object::new(CLOUD2, 13, 12),
            hill: Object::new(HILL, 20, 22),
            mario,
            hour_block,
            minute_block,
            // Initial cloud positions
            cloud1_x: 0,               // Start cloud1 near the left
            cloud2_x: 51,              // Start cloud2 further right
            frame_count: 0,            // Initialize frame counter
            cloud1_seed: initial_seed, // Store initial seed
        }
    }

    pub fn now() -> chrono::DateTime<chrono_tz::Tz> {
        Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich)
    }

    /// Updates the position of a cloud, returns true if it wrapped.
    fn update_cloud_position(x: &mut i32, width: i32, frame_count: u32) -> bool {
        let mut wrapped = false;
        // Only move the cloud every CLOUD_MOVE_INTERVAL frames
        if frame_count % CLOUD_MOVE_INTERVAL == 0 {
            *x -= CLOUD_PIXELS_PER_MOVE;
            // If the cloud is completely off the left edge
            if *x + width < 0 {
                // Reset its position to the right edge
                *x = COLS as i32;
                wrapped = true; // Signal that wrapping occurred
            }
        }
        wrapped
    }
}

impl ClockfaceTrait for Clockface {
    async fn update(&mut self, fb: &mut FBType) {
        // Increment frame counter (wraps around automatically on overflow)
        self.frame_count = self.frame_count.wrapping_add(1);

        // --- 1. Clear Background ---
        fill_rect(fb, 0, 0, ROWS as u32, COLS as u32, SKY_COLOR);

        // --- 2. Update Cloud Positions & Regenerate cloud1 if needed ---
        let cloud1_wrapped =
            Self::update_cloud_position(&mut self.cloud1_x, CLOUD1_WIDTH as i32, self.frame_count);
        Self::update_cloud_position(&mut self.cloud2_x, 24, self.frame_count);

        if cloud1_wrapped {
            // Update seed - using frame_count ensures variety
            self.cloud1_seed = self.frame_count as u64;

            let mut rng = SmallRng::seed_from_u64(self.cloud1_seed);
            let circles = rng.random_range(3..8);
            // Regenerate cloud data
            let cloud1_array =
                generate_cloud(CLOUD1_WIDTH, CLOUD1_HEIGHT, circles, self.cloud1_seed)
                    .expect("Failed to regenerate cloud");
            let cloud1_size = CLOUD1_WIDTH * CLOUD1_HEIGHT;
            // Replace the cloud1 object with a new one containing the new data
            self.cloud1 = Object::new(
                &cloud1_array[0..cloud1_size],
                CLOUD1_WIDTH as i32,
                CLOUD1_HEIGHT as i32,
            );
        }

        // --- 3. Draw Static Background Elements ---
        self.ground.fill_row(COLS as i32 - self.ground.height(), fb);
        self.bush.draw(43, 47, fb); // Bush position seems fixed
        self.hill.draw(0, 34, fb); // Hill position seems fixed

        // --- 4. Draw Moving Clouds ---
        self.cloud1.draw(self.cloud1_x, 21, fb); // Use updated x, fixed y
        self.cloud2.draw(self.cloud2_x, 7, fb); // Use updated x, fixed y

        // --- 5. Update Time and Interactive Elements ---
        let now = Self::now();

        // Check if it's time to trigger a jump - we jump every minute
        // Use atomic compare-and-swap to ensure we only jump once per minute
        // even if update() is called multiple times per second
        let current_minute = now.minute();
        let last_minute = LAST_JUMP_MINUTE.load(Ordering::Relaxed);
        let jump = if current_minute != last_minute && now.second() == 0 {
            // New minute detected and we're at second 0
            LAST_JUMP_MINUTE.store(current_minute, Ordering::Relaxed);
            true
        } else {
            false
        };

        // Update Mario (handles jump trigger) and time blocks
        self.mario.update(fb, jump).await;
        self.hour_block.update(fb, now.hour()).await;
        self.minute_block.update(fb, now.minute()).await;
    }
}
