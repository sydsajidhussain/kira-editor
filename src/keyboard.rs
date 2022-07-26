use crossterm::event::{read, Event::*, KeyEvent};

use crate::*;
use log::info;

#[derive(Debug, Default)]
pub struct Keyboard;

impl Keyboard {
    pub fn read(&self) -> Result<KeyEvent> {
        loop {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                info!("failed to read key {:?}", &self);
                eprintln!("failed to read key {:?}", &self);
            }
        }
    }
}