use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, Result};
use kira_editor::*;
use crate::keyboard::*;
use crate::editor_screen::*;
use std::path::Path;


pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    erows : Vec<String>
}

impl Editor {
    pub fn new<T: AsRef<Path>>(filename: T) -> Result<Self> {
        let line1 = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .next()
            .unwrap()
            .to_string(); 

        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            erows: vec![line1]
        })
    }

    pub fn process_keypress(&mut self) -> Result<bool> {
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => return Ok(true),
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: _,
                } => self.move_cursor('w'),
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: _,
                } => self.move_cursor('a'),
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: _,
                } => self.move_cursor('s'),
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: _,
                } => self.move_cursor('d'),
                KeyEvent {
                    code: KeyCode::Char(key),
                    modifiers: _,
                } => match key {
                    'w' | 'a' | 's' | 'd' => self.move_cursor(key),
                    _ => {}
                },
                _ => {}
            }
        } else {
            self.die("could not read from keyboard".to_string());
        }
        Ok(false)
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen".to_string());
            }
            self.screen.cursor_move_to(&self.cursor)?;
            self.screen.flush()?;
            if self.process_keypress()? {
                break;
            }
        }
        terminal::disable_raw_mode()
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.screen.clear()?;
        self.screen.draw_rows(&self.erows)
    }

    pub fn die(&mut self, msg: String) {
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}", msg.to_string());
        std::process::exit(1);
    }

    fn move_cursor(&mut self, key: char) {
        match key {
            'a' => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
            }
            'd' => self.cursor.x += 1,
            'w' => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            's' => self.cursor.y += 1,
            _ => self.die("invalid movement character".to_string()),
        }
    }
}
