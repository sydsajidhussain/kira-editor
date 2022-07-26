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
    pub fn new(filename: Option<String>) -> Result<Self> {
        Ok(Self {
            screen: Screen::new()?,
            keyboard: Keyboard {},
            cursor: Position::default(),
            erows: Vec::new(),
            filename
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
                    code:KeyCode::Char('f'),
                    modifiers: KeyModifiers::CONTROL, 
                } => {
                    self.search()},
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }=> {
                    self.del_char();
                },
                KeyEvent {
                    code: KeyCode::Delete,
                    modifiers: _,
                } => {{
                    self.cursor.x += 1;
                }
                self.del_char();},
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
            self.screen.move_cursor(&self.cursor)?;
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
        eprintln!("{}", msg);
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

    fn save(&mut self) {
        
        if self.filename.is_none() {
            self.filename = self.prompt("save as");
        } 
 
        let buff = self.erows_to_string();

        let _ = std::fs::write(&self.filename.as_ref()
            .unwrap(), buff);
        
    }

    fn insert_char(&mut self, c: char) {

        if !self.cursor.above(self.erows.len()) {
            self.erows.push(String::new());
        }
        self.erows[self.cursor.y].insert(self.cursor.x, c);
        self.cursor.x += 1;
    }

    fn search(&mut self) {
        
        if let Some(query) = self.prompt("search") {
            for (i, row) in self.erows.iter().enumerate() {
                if let Some(ln) = row.match_indices(query.as_str()).take(1).next() {
                    self.cursor.y = i ;
                    self.cursor.x = ln.0; 
                    break;
                }
            }
        }
    }
    

    fn del_char(&mut self) {

        if !self.cursor.above(self.erows.len()) {
            return;
        }
        if self.cursor.x > 0 {
        self.erows[self.cursor.y].remove(self.cursor.x-1);
        self.cursor.x -= 1;
        }
    }

    fn smart_undo(&mut self) {
    //    todo
    }

    fn erows_to_string(&self)-> String {

        let mut buff = String::new();

        for row in self.erows.iter() {
            buff.push_str(row.chars().as_str());
            buff.push('\n');
        }
        buff
    }

    

    fn prompt(&mut self, prompt: &str) -> Option<String> {
        let mut buf = String::from("");

        loop {
            let _ = self.refresh_screen();
            let msg = format!("{}: {}", prompt, buf);
            print!("{}",msg);
            let _ = self.screen.flush();
            if let Ok(keypress) = self.keyboard.read() {
                match keypress {
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        return Some(buf);
                    }
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: modif,
                    } => {
                        if matches!(modif, KeyModifiers::NONE | KeyModifiers::SHIFT) {
                            buf.push(c);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

}