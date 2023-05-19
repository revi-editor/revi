use crate::panes::Cursor;
use ropey::{Rope, RopeSlice};
#[derive(Debug)]
pub struct Buffer {
    pub name: String,
    pub cursor: Cursor,
    rope: Rope,
}

impl Buffer {
    pub fn new(name: impl Into<String>, rope: Rope) -> Self {
        Self {
            name: name.into(),
            rope,
            cursor: Cursor::default(),
        }
    }

    pub fn new_str(name: impl Into<String>, contents: &str) -> Self {
        Self {
            name: name.into(),
            rope: Rope::from_str(contents),
            cursor: Cursor::default(),
        }
    }

    pub fn from_path(name: impl Into<String>) -> Self {
        let name = name.into();
        let contents = std::fs::read_to_string(&name).expect("failed to read in file to buffer");
        let rope = Rope::from_str(contents.as_str());
        Self {
            name,
            rope,
            cursor: Cursor::default(),
        }
    }

    #[must_use]
    pub fn on_screen(&self, height: u16) -> Vec<RopeSlice> {
        let top = self.cursor.scroll.y as usize;
        let bottom = (self.cursor.scroll.y + height) as usize;
        let mut result = vec![];
        for idx in top..=bottom {
            let Some(line) = self.rope.get_line(idx) else {
                break;
            };
            result.push(line);
        }
        result
        // let start = self.rope.line_to_char(top);
        // let end = self.rope.line_to_char(bottom);
        // self.rope.slice(start..end)
    }

    pub fn len_lines(&self) -> usize {
        let len_lines = self.rope.len_lines();
        let row = self.cursor.pos.y as usize;
        let line_len = self.line_len(row);
        // eprintln!("{}", line_len);
        // if len_lines <= 1 {
        //     return (line_len > 1) as usize;
        // }
        len_lines.saturating_sub(1)
    }

    pub fn line_len(&self, line_idx: usize) -> usize {
        self.rope.line(line_idx).len_chars()
    }

    pub fn push(&mut self, c: impl Into<String>) {
        let screen_row = self.cursor.pos.y as usize;
        let scroll_row = self.cursor.scroll.y as usize;
        let row = screen_row + scroll_row;
        let idx = self.rope.line_to_char(row);
        self.rope.insert(idx, c.into().as_str());
    }

    pub fn clear(&mut self) {
        self.rope = Rope::new();
    }

    pub fn get_rope(&self) -> &Rope {
        &self.rope
    }

    pub fn get_rope_mut(&mut self) -> &mut Rope {
        &mut self.rope
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            name: "N/A".into(),
            rope: Rope::default(), //Rope::from_str("\n"),
            cursor: Cursor::default(),
        }
    }
}
