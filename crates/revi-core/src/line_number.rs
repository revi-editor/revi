#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LineNumbers {
    RelativeNumber(usize, usize),
    AbsoluteNumber(usize, usize),
    None,
}

impl LineNumbers {
    pub fn width(&self) -> usize {
        match self {
            Self::AbsoluteNumber(w, _) | Self::RelativeNumber(w, _) => *w as usize,
            Self::None => 0,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Self::AbsoluteNumber(_, h) | Self::RelativeNumber(_, h) => *h as usize,
            Self::None => 0,
        }
    }
    pub fn lines(&self, offset: usize, line_len: usize, cursor_y: usize) -> String {
        match self {
            Self::RelativeNumber(w, h) => relative_number(*w, *h, cursor_y, 0),
            Self::AbsoluteNumber(w, h) => absolute_number(*w, *h, offset, line_len),
            Self::None => String::new(),
        }
    }
}

fn absolute_number(width: usize, height: usize, offset: usize, file_len: usize) -> String {
    use std::cmp::max;
    let max_number_len = file_len.to_string().len();
    let top = offset;
    let bottom = top + height;
    let blank = (0..width + 1)
        .enumerate()
        .map(|(i, _)| {
            if i == 0 {
                "~"
            } else if i == width {
                "\r\n"
            } else {
                " "
            }
        })
        .collect::<String>();
    (top..bottom)
        .map(|n| {
            if n > file_len.saturating_sub(1) {
                return blank.clone();
            }
            let end = if n == (height + offset) {
                //.saturating_sub(1) {
                ""
            } else {
                "\r\n"
            };
            let padding = (0..(max(max_number_len, width as usize - 2) - n.to_string().len()))
                .map(|_| " ")
                .collect::<String>();
            let line_number = format!(" {}{}", padding, n);
            let width = (0..width.saturating_sub(line_number.len()))
                .map(|_| " ")
                .collect::<String>();
            format!("{}{}{}", line_number, width, end)
        })
        .collect::<String>()
}

#[test]
fn test_line_number_format() {
    //                    width: usize, height: usize, offset: usize, file_len: usize
    let line_numbers = absolute_number(6, 20, 30, 40);
    eprintln!("{}", line_numbers);
    assert_eq!(
        line_numbers,
        "   30 \r\n   31 \r\n   32 \r\n   33 \r\n   34 \r
   35 \r\n   36 \r\n   37 \r\n   38 \r\n   39 \r\n~     \r
~     \r\n~     \r\n~     \r\n~     \r\n~     \r\n~     \r
~     \r\n~     \r\n~     \r\n"
            .to_string()
    );
}

fn relative_number(width: usize, height: usize, cursor_y: usize, line: usize) -> String {
    let height = height.saturating_sub(1);
    let bottom_len = height.saturating_sub(cursor_y);
    let top_len = height.saturating_sub(bottom_len);
    let top_lines = rline_number_formater(width, 1, top_len);
    let bottom_lines = line_number_formater(width, 1, bottom_len);
    let cursor_line = format!("{}{} ", spaceing(width - 2, line), line);
    let cursor_line = format_cursor_line(height, cursor_y, &cursor_line);
    format!("{}{}{}", top_lines, cursor_line, bottom_lines)
}

fn format_cursor_line(height: usize, cursor_y: usize, line: &str) -> String {
    let pre = if cursor_y != 0 { "\r\n" } else { "" };
    let suf = if cursor_y >= height { "" } else { "\r\n" };
    format!("{}{}{}", pre, line, suf)
}

fn spaceing(w: usize, n: usize) -> String {
    (0..=w.saturating_sub(n.to_string().len()))
        .map(|_| ' ')
        .collect()
}
fn new_line(n: usize, stop: usize) -> &'static str {
    if n == stop {
        ""
    } else {
        "\r\n"
    }
}
fn line_number_formater(width: usize, start: usize, stop: usize) -> String {
    (start..=stop)
        .map(|n| format!("{}{} {}", spaceing(width - 2, n), n, new_line(n, stop)))
        .collect::<String>()
}

fn rline_number_formater(width: usize, start: usize, stop: usize) -> String {
    (start..=stop)
        .rev()
        .map(|n| format!("{}{} {}", spaceing(width - 2, n), n, new_line(n, start)))
        .collect::<String>()
}
#[test]
fn test_relative_number() {
    // fn relative_number(width: usize, height: usize, cursor_y: usize, line: usize) -> String {
    let line_numbers = relative_number(6, 20, 0, 0);
    eprintln!("{}", line_numbers);
    assert_eq!(
        line_numbers,
        "    0 \r\n    1 \r\n    2 \r\n    3 \r
    4 \r\n    5 \r\n    6 \r\n    7 \r\n    8 \r\n    9 \r\n   10 \r\n   11 \r
   12 \r\n   13 \r\n   14 \r\n   15 \r\n   16 \r\n   17 \r\n   18 \r\n   19 \r\n   20 "
            .to_string()
    );
    eprintln!("-----------------------------------------------");
    let line_numbers = relative_number(6, 20, 21, 0);
    eprintln!("{}", line_numbers);
    assert_eq!(
        line_numbers,
        "   20 \r\n   19 \r\n   18 \r\n   17 \r\n   16 \r\n   15 \r
   14 \r\n   13 \r\n   12 \r\n   11 \r\n   10 \r\n    9 \r\n    8 \r
    7 \r\n    6 \r\n    5 \r\n    4 \r\n    3 \r\n    2 \r\n    1 \r\n    0 "
            .to_string()
    );
}
