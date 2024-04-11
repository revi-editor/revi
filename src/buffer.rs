use ratatui::layout::Position;
use std::{fs::OpenOptions, io::BufWriter};

use ropey::Rope;

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub pos: Position,
    pub scroll: Position,
    max: Position,
}

impl Cursor {
    pub fn up(&mut self, line_len: usize) {
        let max = self.max.x as usize;
        let col = self.pos.x as usize;
        let col = col.max(max).min(line_len);
        self.set_col(col);
        self.sub_row(1);
    }

    pub fn down(&mut self, line_len: usize) {
        let max = self.max.x as usize;
        let col = self.pos.x as usize;
        let col = col.max(max).min(line_len);
        self.set_col(col);
        self.add_row(1);
    }

    pub fn left(&mut self) {
        self.sub_col_effect_max(1);
    }

    pub fn right(&mut self) {
        self.add_col_effect_max(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll.y = self.scroll.y.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, max: usize) {
        let max = max as u16;
        self.scroll.y = self.scroll.y.saturating_add(1).min(max);
    }

    pub fn scroll_left(&mut self) {
        self.scroll.x = self.scroll.x.saturating_sub(1);
    }

    pub fn scroll_right(&mut self, max: usize) {
        let max = max as u16;
        self.scroll.x = self.scroll.x.saturating_add(1).min(max);
    }

    pub fn pos(&self) -> Position {
        let x = self.pos.x + self.scroll.x;
        let y = self.pos.y + self.scroll.y;
        Position { x, y }
    }

    pub fn new_line(&mut self) {
        self.set_col(0);
        self.add_row(1);
    }

    pub fn row_scroll(&self) -> usize {
        (self.pos.y + self.scroll.y) as usize
    }

    // pub fn row(&self) -> usize {
    //     self.pos.y as usize
    // }

    // pub fn add_row_effect_max(&mut self, row: usize) {
    //     let row = row as u16;
    //     self.pos.y = self.pos.y.saturating_add(row);
    //     self.max.y = self.pos.y.max(self.max.y);
    // }

    pub fn set_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = row;
    }

    pub fn add_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_add(row);
    }

    pub fn sub_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_sub(row);
    }

    // pub fn sub_row_effect_max(&mut self, row: usize) {
    //     let row = row as u16;
    //     self.pos.y = self.pos.y.saturating_sub(row);
    //     self.max.y = self.pos.y.min(self.max.y);
    // }

    // pub fn col(&self) -> usize {
    //     self.pos.x as usize
    // }

    pub fn add_col_effect_max(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_add(col);
        self.max.x = self.pos.x.max(self.max.x);
    }

    pub fn add_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_add(col);
    }

    pub fn sub_col_effect_max(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_sub(col);
        self.max.x = self.pos.x;
    }

    pub fn sub_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_sub(col);
        self.max.x = self.pos.x;
    }

    pub fn set_col_effect(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = col;
        self.max.x = self.pos.x.min(self.max.x);
    }

    pub fn set_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = col;
    }

    pub fn set_max_col(&mut self, col: usize) {
        let col = col as u16;
        self.max.x = col;
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub name: String,
    rope: Rope,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn from_path(path: impl Into<String> + Clone) -> Self {
        let src = std::fs::read_to_string(&path.clone().into()).unwrap_or_default();
        Self {
            name: path.into(),
            rope: Rope::from_str(&src),
            cursor: Cursor::default(),
        }
    }

    pub fn save(&self, filename: Option<String>) -> std::io::Result<()> {
        let name = match filename {
            Some(ref n) => n,
            None => &self.name,
        };
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(name)?;
        self.rope.write_to(BufWriter::new(file))?;
        Ok(())
    }

    pub fn align_cursor(&mut self) {
        let max = self.cursor.max.x as usize;
        let line_max = self.current_line_len();
        self.cursor.set_col(line_max.min(max));
    }

    pub fn _get_line_len(&self, row: usize) -> Option<usize> {
        self.rope
            .get_line(row)
            .map(|line| line.len_chars().saturating_sub(1))
    }

    pub fn current_line_len(&self) -> usize {
        let row = (self.cursor.pos.y + self.cursor.scroll.y) as usize;
        self.rope.line(row).len_chars().saturating_sub(2)
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.rope.line(row).len_chars().saturating_sub(2)
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines().saturating_sub(2)
    }

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn get_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn on_screen(&self, width: u16, height: u16) -> Vec<String> {
        let width = width as usize;
        let height = height as usize;
        let top = self.cursor.scroll.y as usize;
        let bottom = top + height;
        let start = self.cursor.scroll.x as usize;
        let end = start + width;
        self.rope
            .lines()
            .skip(top)
            .take(bottom)
            .map(|line| {
                line.get_slice(start..end).map(|l| l.to_string()).unwrap_or(
                    line.get_slice(start..)
                        .map(|l| {
                            if l.len_chars() == 0 {
                                return " ".to_string();
                            }
                            l.to_string()
                        })
                        .unwrap_or(" ".to_string()),
                )
            })
            .collect()
    }

    pub fn get_all_text(&self) -> String {
        self.rope.to_string()
    }

    pub fn clear(&mut self) {
        self.rope = Rope::from_str("");
        self.cursor = Cursor::default();
    }

    pub fn insert(&mut self, text: impl Into<String>) {
        let text = text.into();
        let row = self.cursor.pos.y as usize;
        let col = self.cursor.pos.x as usize;
        let char_idx = self.rope.line_to_char(row);
        self.rope.insert(char_idx + col, &text);
        let col = text.len();
        self.cursor.add_col(col);
        if text.contains('\n') {
            self.cursor.new_line();
        }
    }

    pub fn backspace(&mut self) {
        let col = self.cursor.pos.x as usize;
        let row = self.cursor.pos.y as usize;
        let char_idx = self.rope.line_to_char(row);
        let start = (char_idx + col).saturating_sub(1);
        let end = char_idx + col;
        self.rope.remove(start..end);
        if col == 0 {
            self.cursor_up();
            self.cursor_end();
            return;
        }
        self.cursor.left();
    }

    pub fn delete_char(&mut self) {
        let col = self.cursor.pos.x as usize;
        let row = self.cursor.pos.y as usize;
        let char_idx = self.rope.line_to_char(row);
        let start = char_idx + col;
        let end = (char_idx + col).saturating_add(1);
        self.rope.remove(start..end);
    }

    pub fn cursor_up(&mut self) -> bool {
        let row = self.cursor.pos.y as usize;
        if row == 0 {
            return false;
        }
        let offset = self.cursor.scroll.y as usize;
        let row = row.saturating_sub(1);
        let len = self.line_len(row + offset);
        self.cursor.up(len);
        true
    }

    pub fn cursor_down(&mut self, max: usize) -> bool {
        let row = self.cursor.pos.y as usize;
        if row >= max {
            return false;
        }
        let offset = self.cursor.scroll.y as usize;
        let row = row.saturating_add(1);
        let len = self.line_len(row + offset);
        self.cursor.down(len);
        true
    }

    pub fn cursor_left(&mut self) -> bool {
        let col = self.cursor.pos.x;
        if col == 0 {
            return false;
        }
        self.cursor.left();
        true
    }

    pub fn cursor_right(&mut self, width: usize) -> bool {
        let len_col = self.current_line_len();
        let col = self.cursor.pos.x as usize;
        if col < len_col && col < width {
            self.cursor.right();
            return true;
        }
        false
    }

    pub fn cursor_end(&mut self) {
        let row = self.cursor.pos.y as usize;
        let len = self.line_len(row);
        self.cursor.set_col(len);
        self.cursor.set_max_col(len);
    }

    pub fn cursor_home(&mut self) {
        self.cursor.set_col(0);
        self.cursor.set_max_col(0);
    }

    pub fn scroll_up(&mut self) {
        self.cursor.scroll_up();
        self.align_cursor()
    }

    pub fn scroll_down(&mut self, height: usize) {
        let max = self.rope.lines().count();
        self.cursor.scroll_down(max.saturating_sub(height));
        self.align_cursor();
    }

    pub fn scroll_left(&mut self) {
        self.cursor.scroll_left();
    }

    pub fn scroll_right(&mut self, width: usize) {
        let max = self.current_line_len();
        self.cursor.scroll_right(max.saturating_sub(width));
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            name: "N/A".into(),
            rope: Rope::default(),
            cursor: Cursor::default(),
        }
    }
}
