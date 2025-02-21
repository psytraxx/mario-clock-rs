use std::sync::{Arc, Mutex};

use crate::engine::{
    display::Display,
    eventbus::EventBus,
    game::{Direction, Event},
    millis, Sprite,
};

use super::assets::{BLOCK, SKY_COLOR};

const MOVE_PACE: u8 = 2;
const MAX_MOVE_HEIGHT: u8 = 4;

#[derive(PartialEq, Clone, Copy)]
enum State {
    Idle,
    Hit,
}

#[derive(PartialEq, Clone)]
pub struct Block {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    direction: Direction,
    text: String,
    last_millis: u64,
    state: State,
    last_state: State,
    first_y: i32,
    last_y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        let block = Block {
            x,
            y,
            width: 19,
            height: 19,
            direction: Direction::Up,
            text: String::new(),
            last_millis: 0,
            state: State::Idle,
            last_state: State::Idle,
            first_y: y,
            last_y: y,
        };
        let block_arc = Arc::new(Mutex::new(block.clone()));
        EventBus::instance().lock().unwrap().subscribe(block_arc);

        block
    }

    pub fn init(&mut self) {
        Display::instance().lock().unwrap().draw_rgb_bitmap(
            self.x,
            self.y,
            BLOCK,
            self.width,
            self.height,
        );
        self.set_text_block();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn idle(&mut self) {
        if self.state != State::Idle {
            self.last_state = self.state;
            self.state = State::Idle;
            self.y = self.first_y;
        }
    }

    fn hit(&mut self) {
        if self.state != State::Hit {
            self.last_state = self.state;
            self.state = State::Hit;
            self.last_y = self.y;
            self.direction = Direction::Up;
        }
    }

    fn set_text_block(&self) {
        let display = Display::instance().lock().unwrap();
        display.set_text_color(0x0000);

        if self.text.len() == 1 {
            display.set_cursor(self.x + 6, self.y + 12);
        } else {
            display.set_cursor(self.x + 2, self.y + 12);
        }

        display.print(&self.text);
    }

    pub fn update(&mut self) {
        if self.state == State::Idle && self.last_state != self.state {
            Display::instance().lock().unwrap().draw_rgb_bitmap(
                self.x,
                self.y,
                BLOCK,
                self.width,
                self.height,
            );
            self.set_text_block();
            self.last_state = self.state;
        } else if self.state == State::Hit && millis() - self.last_millis >= 60 {
            Display::instance().lock().unwrap().fill_rect(
                self.x,
                self.y,
                self.width,
                self.height,
                SKY_COLOR,
            );

            self.y += MOVE_PACE as i32
                * if self.direction == Direction::Up {
                    -1
                } else {
                    1
                };

            Display::instance().lock().unwrap().draw_rgb_bitmap(
                self.x,
                self.y,
                BLOCK,
                self.width,
                self.height,
            );
            self.set_text_block();

            if ((self.first_y - self.y) as f32).floor() as i32 >= MAX_MOVE_HEIGHT as i32 {
                self.direction = Direction::Down;
            }

            if self.y >= self.first_y && self.direction == Direction::Down {
                self.idle();
            }

            self.last_millis = millis();
        }
    }
}

impl Sprite for Block {
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

    fn name(&self) -> &str {
        "Block"
    }

    fn execute(&mut self, sender: &dyn Sprite, event: &Event) {
        if *event == Event::Move && self.collided_with(sender) {
            self.hit();
            EventBus::instance()
                .lock()
                .unwrap()
                .broadcast(&Event::Collision, self);
        }
    }
}
