use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, Result};
use kira_editor::*;
use crate::keyboard::*;
use crate::editor_screen::*;
use std::path::Path;
use log::info;

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    erows : Vec<String>,
    filename: Option<String>
}

impl Editor{
    fn rebuild<T: Into<String>>(data: &[String], filename: T) -> Result<Self> {
        Ok(Self {
            filename: Some(filename.into()),
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            erows: if data.is_empty() {
                Vec::new()
            } else {
                 Vec::from(data)
            },
        })
    }
    pub fn new(data: &[String],filename: Option<String>) -> Result<Self> {
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            erows: Vec::new(),
            filename: filename
        })
    }

    pub fn new_with_file<T: AsRef<Path>+ ToString>(filename:&T) -> Result<Self> {


        let line: Vec<String> = std::fs::read_to_string(filename)
            .expect("Unable to open file")
            .split('\n')
            .map(|x| x.into())
            .collect(); 

            Editor::rebuild(&line, filename.to_string())
    }

    pub fn process_keypress(&mut self) -> Result<bool> {
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    return Ok(true)},
                KeyEvent {
                    code:KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL, 
                } => {
                    self.save()},
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }=> {
                    if let KeyEvent {
                        code: KeyCode::Delete,
                        ..
                    } = c
                    {
                        self.cursor.x += 1;
                    }
                    self.del_char();
                },
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
                    ..
                } => self.insert_char(key),
                
                _ => {}
            }
        } else {
            self.die("could not read from keyboard"
            .to_string());
        }
        Ok(false)
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err() {
                self.die("unable to refresh screen"
                .to_string());
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
            },
            'd' => self.cursor.x += 1,
            'w' => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            's' => self.cursor.y += 1,
            _ => self.die("invalid movement character"
                .to_string()),
        }
    }

    fn save(&self) {

        if self.filename.is_none() {
            return;
        } 

        let buff = self.erows_to_string();

        let _ = std::fs::write(&self.filename.as_ref()
            .unwrap(), buff);
    }

    fn insert_char(&mut self, c: char) {

        if !self.cursor.above(self.erows.len()) {
            self.erows.push(String::new());
        }
        self.erows[self.cursor.y as usize].insert(self.cursor.x as usize, c);
        self.cursor.x += 1;
    }

    fn find(&mut self, query: String) {
        
            for (i, row) in self.erows.iter().enumerate() {
                if let Some(m) = row.match_indices(query.as_str()).take(1).next() {
                    self.cursor.y = i as u16;
                    break;
                }
            }
    }

    fn del_char(&mut self) {

        if !self.cursor.above(self.erows.len()) {
            return;
        }
        if self.cursor.x > 0 {
        self.erows[self.cursor.y as usize].remove(self.cursor.x as usize-1);
        self.cursor.x -= 1;
        }
    }

    fn smart_undo(&mut self) {
       ()
    }

    fn erows_to_string(&self)-> String {

        let mut buff = String::new();

        for row in self.erows.iter() {
            buff.push_str(row.chars().as_str());
            buff.push('\n');
        }
        buff
    }
}