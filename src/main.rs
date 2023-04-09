mod tui;
use std::{io::Write, thread::sleep, time::Duration};

use tui::{
    clear,
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    size,
    text::Text,
    widget::Widget,
};

fn main() {
    let (width, height) = size();
    let (_, file_name) = {
        let args = std::env::args().take(2).collect::<Vec<String>>();
        (args[0].clone(), args.get(1).unwrap_or(&"".into()).clone())
    };

    let mut writer = std::io::stdout();
    clear(&mut writer);

    let file_text = &std::fs::read_to_string(&file_name).expect("expected a file");

    let text_field = Text::new(file_text).with_comment("text file");

    let window = Container::new(Rect::new(Size::new(width, height - 2)), Stack::Horizontally)
        .with_comment("window with text file and numbers");

    let text_numbers = Text::new(
        &(1..=window.width())
            .map(|n| format!(" {} \n", n))
            .collect::<String>(),
    )
    .max_width(4)
    .with_comment("numbers");

    let window = window.push(text_numbers).push(text_field);

    let status_bar = Text::new(&format!("Normal Mode, {}", file_name))
        .max_height(1)
        .with_comment("status bar");
    let command_bar = Text::new("Command Bar, insert command here")
        .max_height(1)
        .with_comment("command bar");
    let revi = Container::new(Rect::new(Size::new(width, height)), Stack::Vertically)
        .with_comment("everything")
        .stack(Stack::Vertically)
        .push(window)
        .push(status_bar)
        .push(command_bar);

    revi.draw(
        &mut writer,
        Rect::with_position(Pos::new(0, 0), Size::new(width, height)),
    );
    writer.flush().unwrap();
    sleep(Duration::from_secs(3));
}
