use crossterm::Result;

mod keyboard;
mod editor_screen;

mod editor;
use editor::*;


fn main() -> Result<()> {

    let mut args = std::env::args();
    
    let mut editor = if args.len() >= 2 {        
        Editor::new_with_file(&args.nth(1).unwrap())?
    } else {
        Editor::new(&[],None)?
    };

    editor.init()?;

    Ok(())
}