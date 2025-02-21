use std::sync::{Arc, LazyLock, Mutex};

use super::{game::Event, Sprite};

const MAX_SUBS: usize = 5;

pub struct EventBus {
    subscriptions: Vec<Arc<Mutex<dyn Sprite + Send + Sync>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscriptions: vec![],
        }
    }

    pub fn broadcast(&self, event: &Event, sender: &dyn Sprite) {
        for sprite in &self.subscriptions {
            // Lock the mutex to access the sprite
            let mut sprite = sprite.lock().unwrap();
            if sprite.name() != sender.name() {
                sprite.execute(sender, event);
            }
        }
    }

    pub fn subscribe(&mut self, sprite: Arc<Mutex<dyn Sprite + Send + Sync>>) {
        if self.subscriptions.len() < MAX_SUBS {
            self.subscriptions.push(sprite);
        } else {
            println!("Max subscriptions reached");
        }
    }

    pub fn instance() -> &'static Mutex<EventBus> {
        static INSTANCE: LazyLock<Mutex<EventBus>> = LazyLock::new(|| Mutex::new(EventBus::new()));
        &INSTANCE
    }
}
