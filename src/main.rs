use rdev::{listen, Event, EventType};
use std::time::Instant;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LAST_CLICK_TIME: Mutex<Option<Instant>> = Mutex::new(None);
}

fn callback(event: Event) {
    if let EventType::ButtonPress(button) = event.event_type {
        if button == rdev::Button::Left {
            let now = Instant::now();
            let mut last_click_time = LAST_CLICK_TIME.lock().unwrap();
            if let Some(last_time) = *last_click_time {
                let duration = now.duration_since(last_time);
                println!("Time since last left click: {} ms", duration.as_millis());
            }
            *last_click_time = Some(now);
        }
    }
}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
