use crate::{
    engine::{draw_rgb_bitmap, fill_rect, millis, rgb565_to_rgb888, Direction, Event, Sprite},
    FBType,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{Publisher, Subscriber},
};

use super::assets::{MARIO_IDLE, MARIO_IDLE_SIZE, MARIO_JUMP, MARIO_JUMP_SIZE, SKY_COLOR};

const MARIO_PACE: u8 = 3;
const MARIO_JUMP_HEIGHT: u8 = 14;

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Idle,
    //Walking,
    Jumping,
}

pub struct Mario {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    direction: Direction,
    last_x: i32,
    last_y: i32,
    sprite: &'static [u16],
    last_millis: u64,
    state: State,
    last_state: State,
    rx: Option<Subscriber<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
    tx: Option<Publisher<'static, CriticalSectionRawMutex, Event, 3, 4, 4>>,
}

impl Mario {
    pub fn new(x: i32, y: i32) -> Self {
        Mario {
            x,
            y,
            width: MARIO_IDLE_SIZE[0] as i32,
            height: MARIO_IDLE_SIZE[1] as i32,
            direction: Direction::Up,
            last_x: x,
            last_y: y,
            sprite: MARIO_IDLE,
            last_millis: 0,
            state: State::Idle,
            last_state: State::Idle,
            rx: None,
            tx: None,
        }
    }

    pub fn init(&mut self, fb: &mut FBType) {
        draw_rgb_bitmap(
            fb,
            self.x,
            self.y,
            MARIO_IDLE,
            MARIO_IDLE_SIZE[0] as i32,
            MARIO_IDLE_SIZE[1] as i32,
        );
    }

    /* pub fn move_sprite(&mut self, dir: Direction) {
           match dir {
               Direction::Right => self.x += MARIO_PACE as i32,
               Direction::Left => self.x -= MARIO_PACE as i32,
               _ => {}
           }
       }
    */
    pub fn jump(&mut self, fb: &mut FBType) {
        if self.state != State::Jumping && (millis() - self.last_millis > 500) {
            self.last_state = self.state;
            self.state = State::Jumping;

            fill_rect(
                fb,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                rgb565_to_rgb888(SKY_COLOR),
            );

            self.width = MARIO_JUMP_SIZE[0] as i32;
            self.height = MARIO_JUMP_SIZE[1] as i32;
            self.sprite = MARIO_JUMP;

            self.direction = Direction::Up;
            self.last_y = self.y;
            self.last_x = self.x;
        }
    }

    fn idle(&mut self, fb: &mut FBType) {
        if self.state != State::Idle {
            self.last_state = self.state;
            self.state = State::Idle;

            fill_rect(
                fb,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                rgb565_to_rgb888(SKY_COLOR),
            );

            self.width = MARIO_IDLE_SIZE[0] as i32;
            self.height = MARIO_IDLE_SIZE[1] as i32;
            self.sprite = MARIO_IDLE;
        }
    }

    pub async fn update(&mut self, fb: &mut FBType) {
        if let Some(rx) = &mut self.rx {
            match rx.try_next_message_pure() {
                Some(Event::Collision(t)) if t.name != self.name() => {
                    self.direction = Direction::Down;
                }
                _ => {}
            }
        }

        if self.state == State::Idle && self.state != self.last_state {
            draw_rgb_bitmap(
                fb,
                self.x,
                self.y,
                MARIO_IDLE,
                MARIO_IDLE_SIZE[0] as i32,
                MARIO_IDLE_SIZE[1] as i32,
            );
        } else if self.state == State::Jumping && millis() - self.last_millis >= 50 {
            fill_rect(
                fb,
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                rgb565_to_rgb888(SKY_COLOR),
            );

            self.y += MARIO_PACE as i32
                * if self.direction == Direction::Up {
                    -1
                } else {
                    1
                };

            draw_rgb_bitmap(fb, self.x, self.y, self.sprite, self.width, self.height);

            let info = self.get_info();
            if let Some(tx) = &mut self.tx {
                tx.publish(Event::Move(info)).await;
            }

            if (self.last_y - self.y) >= MARIO_JUMP_HEIGHT as i32 {
                self.direction = Direction::Down;
            }

            if self.y + self.height >= 56 {
                self.idle(fb);
            }

            self.last_millis = millis();
        }
    }
}

impl Sprite for Mario {
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
        "MARIO"
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
