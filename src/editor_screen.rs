use crossterm::{cursor, style::Print, terminal, QueueableCommand, Result};
use std::io::{stdout, Stdout, Write};

use kira_editor::*;

pub struct Screen {
    stdout: Stdout,
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
            stdout: stdout(),
        })
    }

    pub fn draw_rows(&mut self, erows: &[String]) -> Result<()> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        
        for row in 0..self.height {
            if row >= erows.len() as u16{ 
                if row == self.height / 3 {
                    let mut description = "KIRA editor ---".to_owned() + VERSION;
                    description.truncate(self.width as usize);
                    if description.len() < self.width as usize {
                        let leftmost = ((self.width as usize - description.len()) / 2) as u16;
                        self.stdout
                            .queue(cursor::MoveTo(0, row))?
                            .queue(Print("~".to_string()))?
                            .queue(cursor::MoveTo(leftmost, row))?
                            .queue(Print(description))?;
                    } else {
                        self.stdout
                            .queue(cursor::MoveTo(0, row))?
                            .queue(Print(description))?;
                    }
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(0, row))?
                        .queue(Print("~".to_string()))?;
                }
            } else {
                let row_len = if erows[0].len() > self.width as usize {
                    self.width as usize
                } else {
                    erows[0].len()
                };
                self.stdout
                    .queue(cursor::MoveTo(0, row))?
                    .queue(Print(erows[0][0..row_len].to_string() ))?;
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    pub fn cursor_position(&self) -> Result<(u16, u16)> {
        cursor::position()
    }

    pub fn cursor_move_to(&mut self, pos: &Position) -> Result<()> {
        self.stdout.queue(cursor::MoveTo(pos.x, pos.y))?;
        Ok(())
    }
}
