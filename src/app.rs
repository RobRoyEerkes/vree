use crate::cursor::{Cursor, Direction};
use color_eyre::Result;
use crossterm::event;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Widget};
use ratatui::{DefaultTerminal, Frame};
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Flags {
    Colored,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Default)]
pub struct App {
    dir: PathBuf,
    flags: Vec<Flags>,
    mode: Mode,
    cursor: Cursor,
    files: Vec<PathBuf>,
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.get_files_in_current_dir()?;
        while self.is_running() {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    pub fn parse_cmd_args(args: &Vec<String>) -> Option<Self> {
        let mut app = App {
            dir: std::env::current_dir().expect("No current directory how did you manage that!"),
            ..App::default()
        };

        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                // TODO: Change this to Nc or mc so colored is standard
                "-c" => app.flags.push(Flags::Colored),
                "-d" => {
                    let dir_str = iter.next()?;
                    app.dir = PathBuf::from(dir_str);
                }
                _ => return None,
            }
        }

        Some(app)
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(self.cursor.get_pos());
    }

    fn get_files_in_current_dir(&mut self) -> Result<()> {
        let mut files: Vec<PathBuf> = self
            .dir
            .read_dir()?
            .into_iter()
            .flatten()
            .map(|entry| entry.path())
            .collect();
        files.sort_by(|a, b| {
            if a.is_dir() != b.is_dir() {
                b.is_dir().cmp(&a.is_dir())
            } else {
                a.cmp(b)
            }
        });
        self.files = files;

        Ok(())
    }

    fn enter_on_cursor(&mut self) -> Result<()> {
        let y: usize = self.cursor.get_pos().y.into();

        if self.files.len() < y {
            return Ok(());
        }

        match self.files[y - 1].metadata() {
            Ok(t) if t.is_dir() => {
                self.dir = self.files[y - 1].clone();
                self.get_files_in_current_dir()?
            }
            Ok(_) => todo!("symlink, file, error"),
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [title_bar, tab, bottom_bar] = area.layout(&layout);

        Block::new().render(area, buf);
        self.render_title_bar(title_bar, buf);
        self.render_files(tab, buf);
        App::render_bottom_bar(bottom_bar, buf);
    }
}
impl App {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        Text::from(self.dir.to_string_lossy() + "/").render(area, buf);
    }

    fn render_files(&self, area: Rect, buf: &mut Buffer) {
        let lines: Vec<Line> = self
            .files
            .iter()
            .filter_map(|entry| {
                let mut name = entry.file_name()?.to_string_lossy();

                if entry.is_dir() {
                    name.to_mut().push('/');
                }

                Some(Line::from(name))
            })
            .collect();
        Text::from(lines).left_aligned().render(area, buf);
    }

    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        Text::from("Status line").render(area, buf);
    }
}
