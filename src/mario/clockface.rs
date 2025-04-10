use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use static_cell::StaticCell;

use crate::{
    display::fill_rect,
    engine::{object::Object, tile::Tile, Event, Sprite},
    ClockfaceTrait, FBType, GRID_SIZE,
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
        }
    }

    fn update_time(&mut self) {
        /*
        let date_time =
            Utc::now().with_timezone(&chrono::FixedOffset::east_opt(3600).expect("Invalid offset"));
        self.hour_block.set_text(date_time.hour().to_string());
        self.minute_block
            .set_text(format!("{:02}", date_time.minute()));
        */

        self.hour_block.set_text("00");
        self.minute_block.set_text("00");
    }
}

impl ClockfaceTrait for Clockface {
    fn setup(&mut self, fb: &mut FBType) {
        fill_rect(fb, 0, 0, GRID_SIZE as u32, GRID_SIZE as u32, SKY_COLOR);

        // Initialize scene
        self.ground
            .fill_row(GRID_SIZE as i32 - self.ground.height(), fb);
        self.bush.draw(43, 47, fb);
        self.hill.draw(0, 34, fb);
        self.cloud1.draw(0, 21, fb);
        self.cloud2.draw(51, 7, fb);

        self.update_time();

        self.hour_block.init(fb);
        self.minute_block.init(fb);
        self.mario.init(fb);
    }

    async fn update(&mut self, fb: &mut FBType) {
        self.hour_block.update(fb).await;
        self.minute_block.update(fb).await;
        self.mario.update(fb).await;

        //if Utc::now().second() == 0 {
        self.mario.jump(fb);
        self.update_time();
        //}
    }
}
