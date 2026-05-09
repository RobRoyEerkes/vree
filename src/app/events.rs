use super::cursor::Direction;
use super::{App, Mode};

use color_eyre::Result;
use crossterm::event;
use std::{path::PathBuf, time::Duration};
impl App {
    pub fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !crossterm::event::poll(timeout)? {
            return Ok(());
        }

        if let Some(key) = crossterm::event::read()?.as_key_press_event() {
            match key.code {
                event::KeyCode::Char('q') | event::KeyCode::Esc => self.mode = Mode::Quit,
                event::KeyCode::Char('h') | event::KeyCode::Left => {
                    self.cursor.shift(Direction::Left)
                }
                event::KeyCode::Char('j') | event::KeyCode::Down => {
                    self.cursor.shift(Direction::Down)
                }
                event::KeyCode::Char('k') | event::KeyCode::Up => self.cursor.shift(Direction::Up),
                event::KeyCode::Char('l') | event::KeyCode::Right => {
                    self.cursor.shift(Direction::Right)
                }
                event::KeyCode::Char(':') => self.cursor.command_mode(),
                event::KeyCode::Backspace => {
                    if let Some(parent) = self.dir.parent() {
                        self.dir = PathBuf::from(parent);
                    }
                    self.get_files_in_current_dir()?;
                }
                event::KeyCode::Enter => self.enter_on_cursor()?,
                _ => {}
            };
        }

        Ok(())
    }
}
