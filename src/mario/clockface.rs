use alloc::boxed::Box;
use chrono::Timelike;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{PubSubChannel, Publisher},
};
use static_cell::StaticCell;

use crate::{
    clock::Clock,
    display::fill_rect,
    engine::{object::Object, tile::Tile, Event, Sprite, Updatable}, // Added Updatable
    ClockfaceTrait,
    FBType,
    I2CType,
    COLS,
    ROWS,
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
    updatables: [Box<dyn for<'any> Updatable<'any>>; 3],
    publisher: Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>,
}

impl Clockface {
    pub fn new() -> Self {
        let channel: &'static mut _ = CHANNEL.init(PubSubChannel::new());

        let mut mario = Mario::new(23, 40);
        mario.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let mut hour_block = Block::new(13, 8, "hour");
        hour_block.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let mut minute_block = Block::new(32, 8, "minute");
        minute_block.subscribe(channel.publisher().unwrap(), channel.subscriber().unwrap());

        let updatables: [Box<dyn for<'any> Updatable<'any>>; 3] = [
            Box::new(mario),
            Box::new(hour_block),
            Box::new(minute_block),
        ];

        Self {
            ground: Tile::new(GROUND, 8, 8),
            bush: Object::new(BUSH, 21, 9),
            cloud1: Object::new(super::gfx::assets::CLOUD1, 13, 12),
            cloud2: Object::new(CLOUD2, 13, 12),
            hill: Object::new(HILL, 20, 22),
            updatables,
            publisher: channel.publisher().unwrap(),
        }
    }

    pub fn now() -> chrono::DateTime<chrono_tz::Tz> {
        Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich)
    }

    fn publish_time_event(&self) {
        let now = Self::now();
        let event = Event::TimeUpdate {
            hour: now.hour() as u8,
            minute: now.minute() as u8,
        };
        self.publisher.publish_immediate(event);
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

        self.publish_time_event();

        // Check if it's time to trigger a jump
        let now = Self::now();
        if now.second() % 10 == 0 {
            self.publisher.publish_immediate(Event::JumpTrigger);
        }

        for element in self.updatables.iter_mut() {
            element.update(fb).await;
        }
    }
}
