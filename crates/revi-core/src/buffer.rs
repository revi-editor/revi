use ropey::{Rope, RopeSlice};
#[derive(Debug)]
pub struct Buffer {
    pub name: String,
    rope: Rope,
}

impl Buffer {
    pub fn new(name: impl Into<String>, rope: Rope) -> Self {
        Self {
            name: name.into(),
            rope,
        }
    }

    pub fn new_str(name: impl Into<String>, contents: &str) -> Self {
        Self {
            name: name.into(),
            rope: Rope::from_str(contents),
        }
    }

    pub fn from_path(name: impl Into<String>) -> Self {
        let name = name.into();
        let contents = std::fs::read_to_string(&name).expect("failed to read in file to buffer");
        let rope = Rope::from_str(contents.as_str());
        Self { name, rope }
    }

    #[must_use]
    pub fn on_screen(&self, top: usize, bottom: usize) -> Vec<RopeSlice> {
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
        self.rope.len_lines()
    }
    pub fn line_len(&self, line_idx: usize) -> usize {
        self.rope.line(line_idx).len_chars()
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
            rope: Rope::default(),
        }
    }
}
