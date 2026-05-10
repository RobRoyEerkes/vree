use super::cursor::{Direction, InputMode};
use super::{App, Mode};

use color_eyre::Result;
use color_eyre::eyre::eyre;
use crossterm::event;
use std::{path::PathBuf, time::Duration};
impl App {
    pub fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Some(key) = crossterm::event::read()?.as_key_press_event() {
            match self.cursor.input_mode {
                InputMode::Normal => self.normal_mode_inputs(key),
                InputMode::Command => self.command_mode_inputs(key),
                _ => Err(eyre!("its all fucked")),
            }?
        }

        Ok(())
    }

    fn normal_mode_inputs(&mut self, key: event::KeyEvent) -> Result<()> {
        match key.code {
            event::KeyCode::Char('h') | event::KeyCode::Left => self.cursor.shift(Direction::Left),
            event::KeyCode::Char('j') | event::KeyCode::Down => self.cursor.shift(Direction::Down),
            event::KeyCode::Char('k') | event::KeyCode::Up => self.cursor.shift(Direction::Up),
            event::KeyCode::Char('l') | event::KeyCode::Right => {
                self.cursor.shift(Direction::Right)
            }
            event::KeyCode::Char(':') => {
                // TODO: clear Command buffer and write to command buffer the char which was pressed                // (also need to make / used for searching)
                self.clear_command();
                self.cursor.command_mode();
                self.add_char_to_cursor(':')?;
            }
            event::KeyCode::Backspace => {
                if let Some(parent) = self.dir.parent() {
                    self.dir = PathBuf::from(parent);
                }
                self.get_files_in_current_dir()?;
                self.cursor.reset();
            }
            event::KeyCode::Enter => self.enter_on_cursor()?,
            _ => {}
        };

        Ok(())
    }

    fn command_mode_inputs(&mut self, key: event::KeyEvent) -> Result<()> {
        if key.kind != event::KeyEventKind::Press {
            return Ok(());
        }
        match key.code {
            event::KeyCode::Esc => self.cursor.normal_mode(),
            event::KeyCode::Backspace => self.delete_char(),
            event::KeyCode::Enter => self.run_command_line(),
            event::KeyCode::Char(c) => self.add_char_to_cursor(c)?,

            _ => {}
        };
        Ok(())
    }
    fn clear_command(&mut self) {
        self.command = String::from("");
    }

    fn run_command_line(&mut self) {
        if let Some(c) = self.command.chars().next() {
            if c == '/' {
                todo!("seach and replace");
            } else if c == ':' {
                self.run_command();
            }
        }
        self.cursor.normal_mode();
    }

    fn run_command(&mut self) {
        match &self.command[1..] {
            "q" => self.mode = Mode::Quit,
            &_ => {}
        };
    }

    fn delete_char(&mut self) {
        match self.cursor.input_mode {
            InputMode::Command => _ = self.command.pop(),
            _ => {}
        };
        self.cursor.shift(Direction::Left);
    }

    fn add_char_to_cursor(&mut self, c: char) -> Result<()> {
        match self.cursor.input_mode {
            InputMode::Command => {
                self.command.push(c);
            }
            _ => {}
        };
        self.cursor.shift(Direction::Right);
        Ok(())
    }
}
