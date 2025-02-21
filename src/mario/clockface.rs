use chrono::{DateTime, Timelike, Utc};

use crate::{
    engine::{locator::Locator, millis, object::Object, tile::Tile},
    ClockfaceTrait,
};

use super::gfx::{
    assets::{BUSH, CLOUD2, GROUND, HILL, SKY_COLOR},
    block::Block,
    font::SUPER_MARIO_BROS_24PT,
    mario::Mario,
};

pub struct Clockface {
    date_time: Option<DateTime<Utc>>,
    last_millis: u64,
    // Game objects
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
        Self {
            date_time: None,
            last_millis: 0,
            ground: Tile::new(GROUND, 8, 8),
            bush: Object::new(BUSH, 21, 9),
            cloud1: Object::new(super::gfx::assets::CLOUD1, 13, 12),
            cloud2: Object::new(CLOUD2, 13, 12),
            hill: Object::new(HILL, 20, 22),
            mario: Mario::new(23, 40),
            hour_block: Block::new(13, 8),
            minute_block: Block::new(32, 8),
        }
    }

    fn update_time(&mut self) {
        if let Some(date_time) = &self.date_time {
            self.hour_block.set_text(date_time.hour().to_string());
            self.minute_block
                .set_text(format!("{:02}", date_time.minute()));
        }
    }

    pub fn external_event(&mut self, event_type: i32) {
        if event_type == 0 {
            self.mario.jump();
            self.update_time();
        }
    }
}

impl ClockfaceTrait for Clockface {
    fn setup(&mut self, date_time: DateTime<Utc>) {
        self.date_time = Some(date_time);

        let display = Locator::get_display();
        display.set_font(&SUPER_MARIO_BROS_24PT);
        display.fill_rect(0, 0, 64, 64, SKY_COLOR);

        // Initialize scene
        self.ground.fill_row(64 - self.ground.height());
        self.bush.draw(43, 47);
        self.hill.draw(0, 34);
        self.cloud1.draw(0, 21);
        self.cloud2.draw(51, 7);

        self.update_time();

        self.hour_block.init();
        self.minute_block.init();
        self.mario.init();
    }

    fn update(&mut self) {
        self.hour_block.update();
        self.minute_block.update();
        self.mario.update();

        if let Some(date_time) = &self.date_time {
            if date_time.second() == 0 && millis() - self.last_millis > 1000 {
                self.mario.jump();
                self.update_time();
                self.last_millis = millis();
            }
        }
    }
}
