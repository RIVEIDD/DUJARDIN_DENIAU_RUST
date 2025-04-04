use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

/// Attend que l'utilisateur appuie sur une touche et retourne-la
pub fn wait_for_key() -> Option<char> {
    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => return Some(c),
                    KeyCode::Enter => return None,
                    _ => {}
                }
            }
        }
    }
}
