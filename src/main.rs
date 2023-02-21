use std::error::Error;

use rdev::{Event, EventType, Key, listen, SimulateError};
use rdev::Key::ScrollLock;

static mut isPressed: bool = false;

fn main() {
    if let Err(error) = listen(callbackWrapper) {
        println!("Error: {:?}", error)
    }
}

fn callbackWrapper(event: Event) {
    match callback(event) {
        Ok(_) => {}
        Err(e) => println!("{}", e)
    }
}

fn callback(event: Event) -> Result<(), SimulateError> {
    if event.event_type == EventType::KeyPress(ScrollLock) {
        rdev::simulate(&EventType::KeyPress(Key::BackSlash))?;
    }
    else if event.event_type == EventType::KeyRelease(ScrollLock) {
        rdev::simulate(&EventType::KeyRelease(Key::BackSlash))?;
    }
    Ok(())
}