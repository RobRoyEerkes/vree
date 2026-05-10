use ratatui::layout::{Offset, Position};

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub input_mode: InputMode,
    pos: Position,
    prev_pos: Position,
}

#[allow(dead_code)]
impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.pos = self.pos.offset(Offset { x: -1, y: 0 }),
            Direction::Down => self.pos = self.pos.offset(Offset { x: 0, y: 1 }),
            Direction::Up => self.pos = self.pos.offset(Offset { x: 0, y: -1 }),
            Direction::Right => self.pos = self.pos.offset(Offset { x: 1, y: 0 }),
        };
    }
    pub fn reset(&mut self) {
        self.pos = Position { x: 0, y: 0 };
        self.input_mode = InputMode::Normal;
    }

    pub fn get_pos(&self) -> Position {
        self.pos
    }

    pub fn insert_mode(&mut self) {
        self.input_mode = InputMode::Insert;
    }

    pub fn visual_mode(&mut self) {
        self.input_mode = InputMode::Visual;
    }

    pub fn normal_mode(&mut self) {
        if self.input_mode == InputMode::Command {
            self.pos = self.prev_pos;
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn command_mode(&mut self) {
        self.prev_pos = self.pos;
        self.pos = Position {
            x: 0,
            y: Position::MAX.y - 1,
        };
        self.input_mode = InputMode::Command;
    }
}
