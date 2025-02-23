use crate::engine::{
    display::Display, millis, Direction, Event, EventReceiver, EventSender, Sprite,
};

use super::assets::{BLOCK, SKY_COLOR};

const MOVE_PACE: u8 = 2;
const MAX_MOVE_HEIGHT: u8 = 4;

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    Hit,
}

#[derive(Debug)]
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
    event_tx: Option<EventSender>,
    event_rx: Option<EventReceiver>,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Block {
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
            event_tx: None,
            event_rx: None,
        }
    }

    pub fn init(&mut self, display: &mut Display) {
        display.draw_rgb_bitmap(self.x, self.y, BLOCK, self.width, self.height);
        self.set_text_block(display);
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

    fn set_text_block(&self, display: &mut Display) {
        if self.text.len() == 1 {
            display.print(&self.text, self.x + 6, self.y + 12, 0x0000);
        } else {
            display.print(&self.text, self.x + 2, self.y + 12, 0x0000);
        }
    }

    pub fn update(&mut self, display: &mut Display) {
        if let Some(rx) = &mut self.event_rx {
            if let Ok(Event::Move(sprite)) = rx.try_recv() {
                if sprite.name != self.name() && self.collided_with(&sprite) {
                    self.hit();
                    self.publish_event(Event::Collision(self.get_info()));
                }
            }
        }

        if self.state == State::Idle && self.last_state != self.state {
            display.draw_rgb_bitmap(self.x, self.y, BLOCK, self.width, self.height);
            self.set_text_block(display);
            self.last_state = self.state;
        } else if self.state == State::Hit && millis() - self.last_millis >= 60 {
            display.fill_rect(self.x, self.y, self.width, self.height, SKY_COLOR);

            self.y += MOVE_PACE as i32
                * if self.direction == Direction::Up {
                    -1
                } else {
                    1
                };

            display.draw_rgb_bitmap(self.x, self.y, BLOCK, self.width, self.height);
            self.set_text_block(display);

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

    fn subscribe(&mut self, rx: EventReceiver, tx: EventSender) {
        self.event_rx = Some(rx);
        self.event_tx = Some(tx);
    }

    fn get_sender(&self) -> Option<EventSender> {
        self.event_tx.clone()
    }
}
