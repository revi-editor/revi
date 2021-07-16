use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LineNumbers {
    AbsoluteNumber,
    RelativeNumber,
    Both,
    None,
}

impl LineNumbers {
    pub fn lines(
        &self,
        width: usize,
        height: usize,
        offset: usize,
        cursor: usize,
        len_lines: usize,
    ) -> String {
        match self {
            Self::AbsoluteNumber => absolute_number(width, height, offset, cursor, len_lines),
            Self::RelativeNumber => both_number(self, width, height, offset, cursor, len_lines),
            Self::Both => both_number(self, width, height, offset, cursor, len_lines),
            Self::None => String::new(),
        }
    }
}

fn absolute_number(
    width: usize,
    height: usize,
    offset: usize,
    _cursor: usize,
    len_lines: usize,
) -> String {
    let bottom = (offset + height).min(len_lines);
    let numbers = abs_lines(offset..=bottom.saturating_sub(1));
    let blanks = height.saturating_sub(numbers.len());
    let total = (bottom + blanks).saturating_sub(1);
    numbers
        .iter()
        .chain(&blank_lines(bottom..=total))
        .enumerate()
        .map(|(i, n)| {
            let padding = make_padding(width - 2, n.len());
            let new_line = if i == total { "" } else { "\r\n" };
            format!("{}{} {}", padding, n, new_line)
        })
        .collect::<String>()
}

fn both_number(
    number_type: &LineNumbers,
    width: usize,
    height: usize,
    offset: usize,
    cursor: usize,
    len_lines: usize,
) -> String {
    let top: usize = cursor;
    let bottom: usize = height
        .saturating_sub(1)
        .min(len_lines)
        .saturating_sub(cursor);
    let blanks = height.saturating_sub(bottom + top + 1);
    let top_lines = rabs_lines(1..=top);
    let bottom_lines = abs_lines(1..=bottom);
    let blank_lines = blank_lines(1..=blanks);
    let cursor_in_file = offset + cursor;
    format_rel_lines(
        number_type,
        width,
        cursor_in_file,
        &top_lines,
        &bottom_lines,
        &blank_lines,
    )
}

fn format_rel_lines(
    number_type: &LineNumbers,
    width: usize,
    current_line: usize,
    top: &[String],
    bottom: &[String],
    blanks: &[String],
) -> String {
    let mut all = top.iter().chain(bottom).chain(blanks).peekable();
    let mut lines = String::new();
    let mut prev: Option<&str> = None;
    while let Some(string_number) = all.next() {
        let number_len = string_number.len();
        let padding = (0..(std::cmp::max(3, width as usize - 2) - number_len))
            .map(|_| " ")
            .collect::<String>();

        match (string_number.as_str(), all.peek().map(|i| i.as_str())) {
            ("1", Some("1")) => {
                lines.push_str(&format!("{}{} \r\n", padding, string_number));
                cursor_line(number_type, &mut lines, width, current_line);
            }
            ("1", Some("~")) if prev != Some("1") => {
                lines.push_str(&format!("{}{} \r\n", padding, string_number));
                cursor_line(number_type, &mut lines, width, current_line);
            }
            ("1", None) => {
                lines.push_str(&format!("{}{} \r\n", padding, string_number));
                last_cursor(number_type, &mut lines, width, current_line);
            }
            (_, None) => last_number(&mut lines, &padding, &string_number),
            ("1", Some("2")) if prev.is_none() => {
                cursor_line(number_type, &mut lines, width, current_line);
                lines.push_str(&format!("{}{} \r\n", padding, string_number));
            }
            _ => lines.push_str(&format!("{}{} \r\n", padding, string_number)),
        }
        prev = Some(string_number.as_str());
    }
    lines
}

fn rabs_lines<'a>(range: RangeInclusive<usize>) -> Vec<String> {
    range.rev().map(|i| i.to_string()).collect::<Vec<String>>()
}

fn abs_lines<'a>(range: RangeInclusive<usize>) -> Vec<String> {
    range.map(|i| i.to_string()).collect::<Vec<String>>()
}

fn blank_lines(range: RangeInclusive<usize>) -> Vec<String> {
    range.map(|_| "~".to_string()).collect::<Vec<String>>()
}

fn last_number(lines: &mut String, padding: &str, string_number: &str) {
    lines.push_str(&format!("{}{} ", padding, string_number));
}
fn last_cursor(number_type: &LineNumbers, lines: &mut String, width: usize, current_line: usize) {
    if number_type == &LineNumbers::Both {
        let padding = make_padding(width, current_line.to_string().len());
        lines.push_str(&format!("{}{}", current_line, padding));
    } else {
        let padding = make_padding(width - 2, 1);
        lines.push_str(&format!("{}0 ", padding));
    }
}
fn cursor_line(number_type: &LineNumbers, lines: &mut String, width: usize, current_line: usize) {
    if number_type == &LineNumbers::Both {
        let padding = make_padding(width, current_line.to_string().len());
        lines.push_str(&format!("{}{}\r\n", current_line, padding));
    } else {
        let padding = make_padding(width - 2, 1);
        lines.push_str(&format!("{}0 \r\n", padding));
    }
}

fn make_padding(width: usize, number_len: usize) -> String {
    (0..(width - number_len)).map(|_| " ").collect::<String>()
}
#[test]
fn test_relative_number_bottom() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 9;
    let len_lines = 20;
    let line_type = LineNumbers::RelativeNumber;
    let line_numbers = both_number(&line_type, width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  9 \r\n  8 \r\n  7 \r\n  6 \r\n  5 \r\n  4 \r\n  3 \r\n  2 \r\n  1 \r\n  0 ";
    assert_eq!(line_numbers, right.to_string());
}

#[test]
fn test_relative_number_bottom_with_emtpy_lines() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 7;
    let len_lines = 7;
    let line_type = LineNumbers::RelativeNumber;
    let line_numbers = both_number(&line_type, width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  7 \r\n  6 \r\n  5 \r\n  4 \r\n  3 \r\n  2 \r\n  1 \r\n  0 \r\n  ~ \r\n  ~ ";
    assert_eq!(line_numbers, right.to_string());
}

#[test]
fn test_relative_number_with_empty_lines() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 3;
    let len_lines = 7;
    let line_type = LineNumbers::RelativeNumber;
    let line_numbers = both_number(&line_type, width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  3 \r\n  2 \r\n  1 \r\n  0 \r\n  1 \r\n  2 \r\n  3 \r\n  4 \r\n  ~ \r\n  ~ ";
    assert_eq!(line_numbers, right.to_string());
}

#[test]
fn test_relative_number_without_empty_lines() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 3;
    let len_lines = 20;
    let line_type = LineNumbers::RelativeNumber;
    let line_numbers = both_number(&line_type, width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  3 \r\n  2 \r\n  1 \r\n  0 \r\n  1 \r\n  2 \r\n  3 \r\n  4 \r\n  5 \r\n  6 ";
    assert_eq!(line_numbers, right.to_string());
}

#[test]
fn test_absolute_number_without_empty_lines() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 3;
    let len_lines = 20;
    let line_numbers = absolute_number(width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  0 \r\n  1 \r\n  2 \r\n  3 \r\n  4 \r\n  5 \r\n  6 \r\n  7 \r\n  8 \r\n  9 ";
    assert_eq!(line_numbers, right.to_string());
}

#[test]
fn test_absolute_number_with_empty_lines() {
    let width = 5;
    let height = 10;
    let offset = 0;
    let cursor = 3;
    let len_lines = 6;
    let line_numbers = absolute_number(width, height, offset, cursor, len_lines);
    eprintln!("{}", line_numbers);
    let right = "  0 \r\n  1 \r\n  2 \r\n  3 \r\n  4 \r\n  5 \r\n  ~ \r\n  ~ \r\n  ~ \r\n  ~ ";
    assert_eq!(line_numbers, right.to_string());
}
