use ropey::Rope;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::ops::Range;
use unicode_width::UnicodeWidthStr;

use crate::position::Position;

pub const JUMP_MATCHES: [char; 100] = make_matches();

const fn make_matches() -> [char; 100] {
    [
        '^', '@', '|', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
        '[', ']', '{', '`', '}', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<',
        '=', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
        'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ',
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Letter,
    Punctuation,
    Number,
    NewLine,
    WhiteSpace,
    None,
}
impl From<char> for CharType {
    fn from(c: char) -> Self {
        match c {
            _ if c.is_ascii_alphabetic() => Self::Letter,
            _ if c.is_ascii_punctuation() => Self::Punctuation,
            _ if c.is_ascii_digit() => Self::Number,
            _ if c.is_ascii_whitespace() => Self::WhiteSpace,
            '\n' => Self::NewLine,
            _ => Self::None,
        }
    }
}

impl From<&str> for CharType {
    fn from(s: &str) -> Self {
        match s.parse::<char>().ok() {
            Some(c) => Self::from(c),
            None => Self::None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Buffer {
    inner: Rope,
    name: Option<String>,
    dirty: bool,
}

impl Buffer {
    /// Creates a new empty buffer with no name.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Rope::new(),
            name: None,
            dirty: false,
        }
    }

    /// Creates a new buffer from a string path of file.
    #[must_use]
    pub fn from_path(filename: &str) -> Self {
        let rope = from_path(filename);
        Self {
            inner: rope,
            name: Some(filename.to_string()),
            dirty: false,
        }
    }

    /// Creates a new buffer from string without a name.
    #[must_use]
    pub fn new_str(text: &str) -> Self {
        Self {
            inner: Rope::from(text),
            name: None,
            dirty: false,
        }
    }
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    #[must_use]
    pub fn idx_of_position(&self, pos: &Position) -> usize {
        self.inner.line_to_char(pos.as_usize_y()) + pos.as_usize_x()
    }

    #[must_use]
    pub fn char_at_pos(&self, pos: &Position) -> char {
        self.inner.char(self.idx_of_position(pos))
    }

    #[must_use]
    pub fn line(&self, y: usize) -> String {
        self.inner.line(y).chars().collect::<String>()
    }

    #[must_use]
    pub fn line_len(&self, y: usize) -> usize {
        // let line = self.inner.line(y).chars().collect::<String>();
        // UnicodeWidthStr::width(line.as_str())
        self.inner.line(y).as_str().map(
        UnicodeWidthStr::width).unwrap_or(0)
    }

    #[must_use]
    pub fn len_chars(&self) -> usize {
        self.inner.len_chars()
    }

    #[must_use]
    pub fn len_lines(&self) -> usize {
        self.inner.len_lines()
    }

    #[must_use]
    pub fn line_to_char(&self, line: usize) -> usize {
        self.inner.line_to_char(line)
    }

    #[must_use]
    pub fn char_to_line(&self, line: usize) -> usize {
        self.inner.char_to_line(line)
    }

    pub fn insert_char(&mut self, idx: usize, c: char) {
        self.inner.insert_char(idx, c);
        self.dirty = true;
    }

    pub fn insert_line(&mut self, at_line: usize, text: &str) {
        let idx = self.inner.line_to_char(at_line);
        self.inner.insert(idx, text);
        self.dirty = true;
    }

    pub fn insert(&mut self, idx: usize, text: &str) {
        self.inner.insert(idx, text);
        self.dirty = true;
    }

    pub fn get_char(&self, idx: usize) -> Option<char> {
        self.inner.get_char(idx)
    }

    pub fn remove(&mut self, range: Range<usize>) {
        self.inner.remove(range);
        self.dirty = true;
    }

    #[must_use]
    pub fn contents(&self) -> String {
        self.inner.chars().collect::<String>()
    }

    pub fn clear(&mut self) {
        self.inner = Rope::new();
    }

    #[must_use]
    pub fn on_screen(&self, top: usize, bottom: usize) -> String {
        let top_line = top;
        let bottom_line = bottom;
        self.inner
            .lines_at(top_line)
            .enumerate()
            .filter(|(i, _)| *i < bottom_line)
            .map(|(_, s)| s.to_string())
            .collect::<String>()
    }

    pub fn write_to<T: std::io::Write>(&mut self, writer: T) -> std::io::Result<()> {
        match self.inner.write_to(writer) {
            ok @ Ok(_) => {
                self.dirty = false;
                ok
            }
            err @ Err(_) => err,
        }
    }

    #[must_use]
    pub fn next_jump_idx(&self, pos: &Position) -> Option<usize> {
        // TODO: Fix this awful garbage!!!!!!!!!!!
        let (x, y) = pos.as_usize();
        let result: Vec<(usize, CharType)> = self.line(y).as_str()[x..]
            .match_indices(&JUMP_MATCHES[..])
            .map(|(i, c)| (i, c.into()))
            .collect();
        let possible_jumps = word_indices(&result);
        possible_jumps
            .get(1)
            .and_then(|w| w.first())
            .map(|(i, _)| *i + x)
    }

    #[must_use]
    pub fn prev_jump_idx(&self, pos: &Position) -> Option<usize> {
        // TODO: Fix this awful garbage!!!!!!!!!!!
        let (x, y) = pos.as_usize();
        let result: Vec<(usize, CharType)> = self.line(y).as_str()[..x]
            .rmatch_indices(&JUMP_MATCHES[..])
            .map(|(i, c)| (i, c.into()))
            .collect();
        let possible_jumps = word_indices(&result);
        let idx = possible_jumps
            .get(0)
            .and_then(|w| w.last())
            .map_or(false, |(_, i)| i == &CharType::WhiteSpace) as usize;
        possible_jumps
            .get(idx)
            .and_then(|w| w.last())
            .map(|(i, _)| *i)
    }
}

#[must_use]
pub fn from_path(path: &str) -> Rope {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Problem opening the file");

    Rope::from_reader(BufReader::new(file)).expect("Failed to open file.")
}

fn word_indices(items: &[(usize, CharType)]) -> Vec<Vec<(usize, CharType)>> {
    // TODO: Fix this awful garbage!!!!!!!!!!!
    let mut stream = items.iter().peekable();
    let mut word_loc = Vec::new();
    if let Some(f) = stream.next() {
        word_loc.push(vec![*f]);
    } else {
        return word_loc;
    }

    while let Some(current) = stream.next() {
        if current.1 == CharType::WhiteSpace {
            if let Some(f) = word_loc.last() {
                if !f.is_empty() {
                    word_loc.push(Vec::new());
                }
            }
            continue;
        }
        if let Some(last_word) = word_loc.last_mut() {
            if let Some(last_char) = last_word.last() {
                if current.1 != last_char.1 {
                    word_loc.push(vec![*current]);
                } else if let Some(next) = stream.peek() {
                    if next.1 != current.1 {
                        last_word.push(*current);
                    }
                } else {
                    last_word.push(*current);
                }
            } else {
                last_word.push(*current);
            }
        }
    }
    word_loc
}

#[test]
fn test_buffer_len() {
    use ropey::Rope;
    let rope = Rope::from("0\n1\n2\n3\n4\n5\n"); // 7 lines
    assert_eq!(rope.len_lines(), 7);
}
