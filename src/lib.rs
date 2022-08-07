#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn above(&self, row: usize) -> bool {
        self.y < row as u16
    }
}
