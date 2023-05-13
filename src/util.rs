use crossterm::event::KeyEvent;

use crate::Error;

pub fn next_key() -> Result<KeyEvent, Error> {
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            return Ok(key);
        }
    }
}
