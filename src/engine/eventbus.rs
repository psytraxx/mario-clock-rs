use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, Once},
};

use super::{game::Event, Sprite};

const MAX_SUBS: usize = 5;

pub struct EventBus {
    subscriptions: Vec<Rc<RefCell<dyn Sprite>>>,
}

static mut EVENT_BUS: Option<Arc<Mutex<EventBus>>> = None;
static INIT: Once = Once::new(); // Ensures initialization happens only once

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscriptions: vec![],
        }
    }

    pub fn broadcast(&self, event: &Event, sender: &dyn Sprite) {
        for sprite in &self.subscriptions {
            if sprite.borrow().name() != sender.name() {
                sprite.borrow_mut().execute(sender, event);
            }
        }
    }

    pub fn subscribe(&mut self, sprite: Rc<RefCell<dyn Sprite>>) {
        if self.subscriptions.len() < MAX_SUBS {
            self.subscriptions.push(sprite);
        } else {
            println!("Max subscriptions reached");
        }
    }

    pub fn get_instance() -> Arc<Mutex<EventBus>> {
        unsafe {
            INIT.call_once(|| {
                EVENT_BUS = Some(Arc::new(Mutex::new(EventBus::new())));
            });

            EVENT_BUS.clone().unwrap() // Safe because it's initialized only once
        }
    }
}
