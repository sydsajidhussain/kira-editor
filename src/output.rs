use std::io::{stdout, Stdout, Write};
use log::info;

use crossterm::{cursor, terminal, terminal::size, QueueableCommand, Result, style::Print};

pub fn clear_screen(stdout: &mut Stdout) -> Result<()> {
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()
}

pub fn editor_refresh_screen() -> Result<()> {
    let mut stdout = stdout();
    clear_screen(&mut stdout)?;
    editor_draw_rows(&mut stdout)?;
    stdout.queue(cursor::MoveTo(0, 0))?.flush()
}

pub fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    let _ = clear_screen(&mut stdout);
    let _ = terminal::disable_raw_mode();
    info!("{:?}",&message.into());
    std::process::exit(1);
}

#[derive(Default)]
pub Struct Position {
    pub row : u16,
    pub col: u16
}

pub fn editor_draw_rows(stdout: &mut Stdout) -> Result<()> {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let (columns, rows) = size()?;
    let height = rows as usize;

    for row in 0..rows {
        if usize::from(row) == height/ 3 {
            let mut description = "Kira editor -- ".to_string()+VERSION;
            description.truncate(columns as usize);

            if description.len() < usize::from(columns){
                let leftmost = ((columns as usize - description.len()) / 2) as u16;
                stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?
                .queue(cursor::MoveTo(leftmost, row))?
                .queue(Print(description))?;
            } else {
                stdout
                .queue(Print("~".to_string()))?
                .queue(cursor::MoveTo(0, row))?
                .queue(Print(description))?;
            }
        } else {
            stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
    }
    Ok(())
}