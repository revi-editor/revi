// This hole file SUCKS.

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LineNumberKind {
    AbsoluteNumber,
    RelativeNumber,
    Both,
    None,
}

impl LineNumberKind {
    #[must_use]
    pub fn lines(&self, builder: LineNumberBuilder) -> LineNumber {
        match self {
            Self::AbsoluteNumber => absolute_line_numbers(builder),
            Self::RelativeNumber => relative_line_numbers(builder),
            Self::Both => Vec::new(),
            Self::None => Vec::new(),
        }
    }
}
type LineNumber = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumberBuilder {
    pub width: usize,
    pub height: usize,
    pub line_len: usize,
    pub cursor_pos: usize,
    pub window_offset: usize,
    pub blank_line: String,
}

impl LineNumberBuilder {
    pub fn top(&self) -> usize {
        self.window_offset
    }
    pub fn bottom(&self) -> usize {
        (self.height + self.window_offset).min(self.line_len)
    }
}

pub fn pad(w: usize, num_width: usize) -> String {
    (0..w.saturating_sub(num_width)).map(|_| " ").collect()
}

pub fn format_line_number(builder: LineNumberBuilder, offset: usize) -> impl Fn(usize) -> String {
    move |num| {
        format!(
            "{}{} ",
            pad(builder.width, format!("{}", num + offset).len()),
            num + offset
        )
    }
}

pub fn format_blanks(builder: LineNumberBuilder) -> impl Fn(usize) -> String {
    move |_| {
        format!(
            "{}{} ",
            pad(builder.width, builder.blank_line.len()),
            builder.blank_line
        )
    }
}

pub fn absolute_line_numbers(builder: LineNumberBuilder) -> LineNumber {
    let mut numbers = (builder.top()..builder.bottom())
        .map(format_line_number(builder.clone(), 0))
        .collect::<Vec<_>>();
    let mut blanks = (0..builder.height.saturating_sub(numbers.len()))
        .map(format_blanks(builder.clone()))
        .collect::<Vec<_>>();
    numbers.append(&mut blanks);

    numbers
}

pub fn relative_line_numbers(builder: LineNumberBuilder) -> LineNumber {
    let mut above_cursor = (0..builder.cursor_pos.saturating_sub(builder.top()))
        .map(format_line_number(builder.clone(), 1))
        .rev()
        .collect::<Vec<_>>();
    let cursor = format_line_number(builder.clone(), 0)(0);
    // CHANGED: made this 1..=builder
    let mut below_cursor = (0..builder
        .bottom()
        .saturating_sub(1)
        .saturating_sub(builder.cursor_pos))
        .map(format_line_number(builder.clone(), 1))
        .collect::<Vec<_>>();
    let mut blanks = (0..builder
        .height
        .saturating_sub(above_cursor.len() + 1 + below_cursor.len()))
        .map(format_blanks(builder.clone()))
        .collect::<Vec<_>>();
    above_cursor.push(cursor);
    above_cursor.append(&mut below_cursor);
    above_cursor.append(&mut blanks);

    above_cursor
}

mod test {
    const _WIDTH: usize = 5;
    use super::*;
    fn _builder_for_curor_line_0() -> LineNumberBuilder {
        LineNumberBuilder {
            width: _WIDTH - 1,
            height: 10,
            line_len: 2,
            cursor_pos: 0,
            window_offset: 0,
            blank_line: "~".into(),
        }
    }
    #[test]
    fn test_absolute_line_numbers() {
        let builder = _builder_for_curor_line_0();
        let left = absolute_line_numbers(builder.clone());
        let right = vec![
            "   0 ", "   1 ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ",
            "   ~ ",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        assert_eq!(left, right);
    }
    #[test]
    fn test_relative_line_numbers_0() {
        let builder = _builder_for_curor_line_0();
        let left = absolute_line_numbers(builder.clone());
        let right = vec![
            "   0 ", "   1 ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ", "   ~ ",
            "   ~ ",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        assert_eq!(left, right);
    }
    #[test]
    fn test_relative_line_numbers_1() {
        let builder = LineNumberBuilder {
            width: _WIDTH - 1,
            height: 10,
            line_len: 20,
            cursor_pos: 5,
            window_offset: 0,
            blank_line: "~".into(),
        };
        let left = relative_line_numbers(builder.clone());
        let right = vec![
            "   5 ", "   4 ", "   3 ", "   2 ", "   1 ", "   0 ", "   1 ", "   2 ", "   3 ",
            "   4 ",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        assert_eq!(left, right);
    }
}
