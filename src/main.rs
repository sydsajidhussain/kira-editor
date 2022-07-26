use crossterm::Result;

mod keyboard;
mod editor_screen;

mod editor;
use editor::*;

fn main() -> Result<()> {
    let mut editor = Editor::new("testdata.txt")?;

    editor.init()?;

    Ok(())
}