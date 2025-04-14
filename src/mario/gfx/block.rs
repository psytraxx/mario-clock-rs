use super::assets::{BLACK, BLOCK, SKY_COLOR};
use crate::{
    display::{draw_rgb_bitmap, fill_rect, print_text},
    engine::{millis, Direction, Event, Sprite},
    FBType,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};
use heapless::String;

const MOVE_PACE: u8 = 2;
const MAX_MOVE_HEIGHT: u8 = 4;

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    Hit,
}

pub(crate) struct Block {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    direction: Direction,
    text: String<2>,
    last_millis: u64,
    state: State,
    last_state: State,
    first_y: i32,
    last_y: i32,
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
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
            rx: None,
            tx: None,
        }
    }

    pub fn init(&mut self, fb: &mut FBType) {
        draw_rgb_bitmap(
            fb,
            self.x,
            self.y,
            BLOCK, // Use the BLOCK asset
            self.width,
            self.height,
        );
        self.set_text_block(fb);
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text).unwrap();
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

    fn set_text_block(&self, fb: &mut FBType) {
        if self.text.len() == 1 {
            print_text(fb, &self.text, self.x + 6, self.y + 12, BLACK);
        } else {
            print_text(fb, &self.text, self.x + 2, self.y + 12, BLACK);
        }
    }

    pub async fn update(&mut self, fb: &mut FBType) {
        if let Some(rx) = &mut self.rx {
            if let Some(Event::Move(sprite)) = rx.try_next_message_pure() {
                if sprite.name != self.name() && self.collided_with(&sprite) {
                    self.hit();
                    let info = self.get_info();
                    if let Some(tx) = &mut self.tx {
                        tx.publish(Event::Collision(info)).await;
                    }
                }
            }
        }

        if self.state == State::Idle && self.last_state != self.state {
            draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);
            self.set_text_block(fb);
            self.last_state = self.state;
        } else if self.state == State::Hit && millis() - self.last_millis >= 60 {
            fill_rect(
                fb,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                SKY_COLOR,
            );

            self.y += MOVE_PACE as i32
                * if self.direction == Direction::Up {
                    -1
                } else {
                    1
                };

            draw_rgb_bitmap(fb, self.x, self.y, BLOCK, self.width, self.height);

            self.set_text_block(fb);

            if (self.first_y - self.y) >= MAX_MOVE_HEIGHT as i32 {
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

    fn name(&self) -> &'static str {
        "Block"
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
