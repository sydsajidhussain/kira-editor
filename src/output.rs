use std::io::{stdout, Stdout, Write};
use log::info;

use crossterm::{cursor, terminal, QueueableCommand, Result, style::Print};

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

pub fn editor_draw_rows(stdout: &mut Stdout) -> Result<()> {
    for row in 0..24 {
        stdout
            .queue(cursor::MoveTo(0, row))?
            .queue(Print("~".to_string()))?;
    }
    Ok(())
}