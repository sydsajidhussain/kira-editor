#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn above(&self, row: usize) -> bool {
        self.y < row 
    }
}
