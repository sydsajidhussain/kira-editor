use std::ops::Index;

const TAB_STOP: usize = 8;

#[derive(Debug)]
pub struct Row {
    chars: String,
    pub render: String,
}

impl Row {
    pub fn new(chars: String) -> Self {
        let render = Row::render_row(&chars);
        Self { chars, 
            render 
        }
    }

    pub fn render_len(&self) -> usize {
        self.render.len()
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn cx_to_rx(&self, cx: u16) -> u16 {
        let mut rx = 0;
        for c in self.chars.chars().take(cx as usize) {
            if c == '\t' {
                rx += (TAB_STOP - 1) - (rx % TAB_STOP);
            }
            rx += 1;
        }
        rx as u16
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        if at >= self.chars.len() {
            self.chars.push(c);
        } else {
            self.chars.insert(at, c);
        }
        self.render = Row::render_row(&self.chars);
    }

    fn render_row(chars: &str) -> String {
        let mut render = String::new();
        let mut index = 0;
        for c in chars.chars() {
            match c {
                '\t' => {
                    render.push(' ');
                    index += 1;
                    while index % TAB_STOP != 0 {
                        render.push(' ');
                        index += 1;
                    }
                }
                _ => {
                    render.push(c);
                    index += 1;
                }
            }
        }
        render
    }
}