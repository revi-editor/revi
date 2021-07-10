/* revi-core/src/text_formater.rs
 */

fn count_char(string: &str, chr: char) -> usize {
    let mut counter = 0;
    for c in string.chars() {
        if c == chr {
            counter += 1;
        }
    }
    counter
}

pub(crate) fn format_screen(view: &str, offset: usize, width: usize, height: usize) -> String {
    let filler = ' '; // std::char::from_u32(9608).unwrap_or('&');
    let mut new = String::new();
    for (y, line) in view.lines().enumerate() {
        if y == height {
            break;
        }
        new.push_str(format_line(line, filler, offset, width).as_str());
    }
    fill_rest_of_screen(&mut new, filler, width, height);
    new
}

fn fill_rest_of_screen(formated_text: &mut String, filler: char, width: usize, height: usize) {
    for _ in 0..(height.saturating_sub(count_char(&formated_text, '\n'))) {
        formated_text.push_str(&vec![filler; width].iter().collect::<String>());
        formated_text.push_str("\r\n");
    }
}

fn format_line(line: &str, filler: char, offset: usize, width: usize) -> String {
    // if let Some(s) = line.get(offset..width + offset) {
    //     return format!("{}\r\n", remove_new_line(&s));
    let len = line.len();
    if let Some(s) = line.get(offset.min(len)..len.min(width.saturating_add(offset))) {
        let new = remove_new_line(&s);
        let space = vec![filler; width.saturating_sub(new.len())]
            .iter()
            .collect::<String>();
        return format!("{}{}\r\n", new, space);
    }
    format!(
        "EMPTY{}\r\n",
        vec![filler; width].iter().collect::<String>()
    )
}

fn remove_new_line(line: &str) -> String {
    line.chars().filter(|c| c != &'\n').collect()
}

#[test]
fn test_formate_line() {
    let line = "Wow\n";
    let width = 10;
    let offset = 0;
    let filler = ' ';
    let new = format_line(line, filler, offset, width);
    assert_eq!(new, "Wow       \r\n".to_string());
    let new = format_line(line, filler, offset + 1, width);
    assert_eq!(new, "ow        \r\n".to_string());
    let new = format_line(line, filler, offset + 2, width);
    assert_eq!(new, "w         \r\n".to_string());
    let new = format_line(line, filler, offset + 3, width);
    assert_eq!(new, "          \r\n".to_string());
    let new = format_line(line, filler, offset + 5, width);
    assert_eq!(new, "          \r\n".to_string());
}
#[test]
fn test_formate_long_line() {
    let line = "1 2 3 4 5 6 7 8 9 0\n";
    let width = 10;
    let offset = 0;
    let filler = ' ';
    let new = format_line(line, filler, offset, width);
    assert_eq!(new, "1 2 3 4 5 \r\n".to_string());

    let new = format_line(line, filler, offset + 1, width);
    assert_eq!(new, " 2 3 4 5 6\r\n".to_string());

    let new = format_line(line, filler, offset + 2, width);
    assert_eq!(new, "2 3 4 5 6 \r\n".to_string());
}

#[test]
fn test_format_screen() {
    let view = "Wow\nThis is amazing cause it works\nSOOOOOOO Well\nRemoving the hard stufffffffffffffff\nnot really.";
    let width = 10;
    let height = 3;
    let offset = 5;
    let new = format_screen(view, offset, width, height);
    eprint!("{}", new);
    assert_eq!(
        new,
        "          \r\nis amazing\r\nOOO Well  \r\n".to_string()
    );
}

#[test]
fn test_format_screen_with_filler() {
    use crate::Buffer;
    let text = "Wow hey there this line should be way to long.\nThis is amazing cause it works\nSOOOOOOO Well\nRemoving the hard stufffffffffffffff\nnot really.";
    let buffer = Buffer::from(text);
    let width = 10;
    let height = 6;
    let offset = 15;
    let view = buffer.on_screen(0, height);
    let new = format_screen(view.as_str(), offset, width, height);
    eprint!("{}", new);
    assert_eq!(
        new,
        "his line s\r\n cause it \r\n          \r\nrd stuffff\r\n          \r\n          \r\n"
            .to_string()
    );
}
