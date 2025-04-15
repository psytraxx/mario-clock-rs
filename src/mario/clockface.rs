use chrono::Timelike;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{PubSubChannel, Publisher},
};
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
    mario::Mario,
};

static CHANNEL: StaticCell<PubSubChannel<CriticalSectionRawMutex, Event, 3, 4, 4>> =
    StaticCell::new();

pub(crate) struct Clockface {
    ground: Tile,
    bush: Object,
    cloud1: Object,
    cloud2: Object,
    hill: Object,
    mario: Mario,
    hour_block: Block,
    minute_block: Block,
    publisher: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
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

        Self {
            ground: Tile::new(GROUND, 8, 8),
            bush: Object::new(BUSH, 21, 9),
            cloud1: Object::new(super::gfx::assets::CLOUD1, 13, 12),
            cloud2: Object::new(CLOUD2, 13, 12),
            hill: Object::new(HILL, 20, 22),
            mario,
            hour_block,
            minute_block,
            publisher: channel.publisher().unwrap(),
        }
    }

    pub fn now() -> chrono::DateTime<chrono_tz::Tz> {
        Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich)
    }
}

impl ClockfaceTrait for Clockface {
    async fn update(&mut self, fb: &mut FBType) {
        fill_rect(fb, 0, 0, ROWS as u32, COLS as u32, SKY_COLOR);

        self.ground.fill_row(COLS as i32 - self.ground.height(), fb);
        self.bush.draw(43, 47, fb);
        self.hill.draw(0, 34, fb);
        self.cloud1.draw(0, 21, fb);
        self.cloud2.draw(51, 7, fb);

        let now = Self::now();
        // Check if it's time to trigger a jump

        if now.second() % 10 == 0 {
            self.publisher.publish_immediate(Event::JumpTrigger);
        }
        // Update the hour and minute blocks
        self.mario.update(fb).await;
        self.hour_block.update(fb, now.hour()).await;
        self.minute_block.update(fb, now.minute()).await;
    }
}
