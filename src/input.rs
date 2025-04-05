use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

/// Attend que l'utilisateur appuie sur une touche et retourne-la.
/// Retourne une erreur si la lecture échoue.
pub fn wait_for_key() -> Result<Option<char>, std::io::Error> {
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => return Ok(Some(c)),
                    KeyCode::Enter => return Ok(None),
                    _ => {}
                }
            }
        }
    }
}
