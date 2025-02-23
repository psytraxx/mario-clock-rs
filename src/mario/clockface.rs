use crate::{
    engine::{create_event_channel, display::Display, object::Object, tile::Tile, Sprite},
    ClockfaceTrait, GRID_SIZE,
};
use chrono::{Timelike, Utc};

use super::gfx::{
    assets::{BUSH, CLOUD2, GROUND, HILL, SKY_COLOR},
    block::Block,
    font::SUPER_MARIO_BROS_24PT,
    mario::Mario,
};

pub struct Clockface {
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
        let (tx, rx) = create_event_channel();

        let mut mario = Mario::new(23, 40);
        mario.subscribe(rx.resubscribe(), tx.clone());
        let mut hour_block = Block::new(13, 8);
        hour_block.subscribe(rx.resubscribe(), tx.clone());
        let mut minute_block = Block::new(32, 8);
        minute_block.subscribe(rx.resubscribe(), tx.clone());

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
        let date_time =
            Utc::now().with_timezone(&chrono::FixedOffset::east_opt(3600).expect("Invalid offset"));
        self.hour_block.set_text(date_time.hour().to_string());
        self.minute_block
            .set_text(format!("{:02}", date_time.minute()));
    }
}

impl ClockfaceTrait for Clockface {
    fn setup(&mut self, display: &mut Display) {
        // set global font
        display.set_font(SUPER_MARIO_BROS_24PT);
        display.fill_rect(0, 0, GRID_SIZE as i32, GRID_SIZE as i32, SKY_COLOR);

        // Initialize scene
        self.ground
            .fill_row(GRID_SIZE as i32 - self.ground.height(), display);
        self.bush.draw(43, 47, display);
        self.hill.draw(0, 34, display);
        self.cloud1.draw(0, 21, display);
        self.cloud2.draw(51, 7, display);

        self.update_time();

        self.hour_block.init(display);
        self.minute_block.init(display);
        self.mario.init(display);
    }

    fn update(&mut self, display: &mut Display) {
        self.hour_block.update(display);
        self.minute_block.update(display);
        self.mario.update(display);

        if Utc::now().second() == 0 {
            self.mario.jump(display);
            self.update_time();
        }
    }
}
