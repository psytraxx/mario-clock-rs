use crate::engine::{
    display::Display, millis, Direction, Event, EventReceiver, EventSender, Sprite,
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

#[derive(Debug)]
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
    event_tx: Option<EventSender>,
    event_rx: Option<EventReceiver>,
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
            event_tx: None,
            event_rx: None,
        }
    }

    pub fn init(&mut self, display: &mut Display) {
        display.draw_rgb_bitmap(
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
    pub fn jump(&mut self, display: &mut Display) {
        if self.state != State::Jumping && (millis() - self.last_millis > 500) {
            self.last_state = self.state;
            self.state = State::Jumping;

            display.fill_rect(self.x, self.y, self.width, self.height, SKY_COLOR);

            self.width = MARIO_JUMP_SIZE[0] as i32;
            self.height = MARIO_JUMP_SIZE[1] as i32;
            self.sprite = MARIO_JUMP;

            self.direction = Direction::Up;
            self.last_y = self.y;
            self.last_x = self.x;
        }
    }

    fn idle(&mut self, display: &mut Display) {
        if self.state != State::Idle {
            self.last_state = self.state;
            self.state = State::Idle;

            display.fill_rect(self.x, self.y, self.width, self.height, SKY_COLOR);

            self.width = MARIO_IDLE_SIZE[0] as i32;
            self.height = MARIO_IDLE_SIZE[1] as i32;
            self.sprite = MARIO_IDLE;
        }
    }

    pub fn update(&mut self, display: &mut Display) {
        if let Some(rx) = &mut self.event_rx {
            if let Ok(Event::Collision(info)) = rx.try_recv() {
                if info.name != self.name() {
                    self.direction = Direction::Down;
                }
            }
        }

        if self.state == State::Idle && self.state != self.last_state {
            display.draw_rgb_bitmap(
                self.x,
                self.y,
                MARIO_IDLE,
                MARIO_IDLE_SIZE[0] as i32,
                MARIO_IDLE_SIZE[1] as i32,
            );
        } else if self.state == State::Jumping && millis() - self.last_millis >= 50 {
            display.fill_rect(self.x, self.y, self.width, self.height, SKY_COLOR);

            self.y += MARIO_PACE as i32
                * if self.direction == Direction::Up {
                    -1
                } else {
                    1
                };

            display.draw_rgb_bitmap(self.x, self.y, self.sprite, self.width, self.height);

            self.publish_event(Event::Move(self.get_info()));

            if ((self.last_y - self.y) as f32).floor() as i32 >= MARIO_JUMP_HEIGHT as i32 {
                self.direction = Direction::Down;
            }

            if self.y + self.height >= 56 {
                self.idle(display);
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

    fn name(&self) -> &str {
        "MARIO"
    }

    fn subscribe(&mut self, rx: EventReceiver, tx: EventSender) {
        self.event_rx = Some(rx);
        self.event_tx = Some(tx);
    }

    fn get_sender(&self) -> Option<EventSender> {
        self.event_tx.clone()
    }
}
