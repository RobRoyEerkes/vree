use std::path::PathBuf;

use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Flags {
    Colored,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct App {
    dir: PathBuf,
    flags: Vec<Flags>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
    pub fn parse_cmd_args(args: &Vec<String>) -> Option<Self> {
        let mut flags: Vec<Flags> = vec![];
        let mut dir = std::env::current_dir().unwrap();
        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-c" => flags.push(Flags::Colored),
                "-d" => {
                    let dir_str = iter.next()?;
                    dir = PathBuf::from(dir_str);
                }
                _ => return None,
            }
        }
        Some(App { dir, flags })
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(self.dir.to_str().unwrap(), frame.area());
    }
}
