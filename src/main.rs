mod tui;
use std::{io::Write, thread::sleep, time::Duration};

use tui::{
    clear,
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    size,
    text::Text,
    widget::Widget,
    widget::BoxWidget,
};

trait App: Sized {
    fn new(_: String, _: String) -> Self;
    fn update(&mut self);
    fn view(&self) -> BoxWidget;
    fn run(&mut self) {
        run(self);
    }
}

fn run<A>(app: &mut A) where A: App{
    let mut writer = std::io::stdout();
    clear(&mut writer);
    let (width, height) = size();
    loop {
        app.update();
        let widgets = app.view();
        widgets.draw(
            &mut writer,
            Rect::with_position(Pos::new(0, 0), Size::new(width, height)),
        );
        writer.flush().unwrap();
    }
}

#[derive(Default)]
struct Revi {
    contents: String,
    file_name: String,
    count: u16,
}
impl App for Revi {
    fn new(contents: String, file_name: String) -> Self {
        Self {
            contents, file_name, ..Default::default()
        }
    }
    fn update(&mut self) {
        self.count += 1;
    }
    fn view(&self) -> BoxWidget {
        let (width, height) = size();
        let text_field = Text::new(&self.contents).with_comment("text file");

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

        let status_bar = Text::new(&format!("Normal Mode, {}", self.file_name))
            .max_height(1)
            .with_comment("status bar");

        let command_bar = Text::new(&format!("Command Bar, insert command here [count is: {}]", self.count))
            .max_height(1)
            .with_comment("command bar");

        Container::new(Rect::new(Size::new(width, height)), Stack::Vertically)
            .with_comment("everything")
            .stack(Stack::Vertically)
            .push(window)
            .push(status_bar)
            .push(command_bar)
            .into()
        }
}

fn main() {
    // let (width, height) = size();
    // let (_, file_name) = {
    //     let args = std::env::args().take(2).collect::<Vec<String>>();
    //     (args[0].clone(), args.get(1).unwrap_or(&"".into()).clone())
    // };
    //
    // let mut writer = std::io::stdout();
    // clear(&mut writer);
    //
    // let file_text = &std::fs::read_to_string(&file_name).expect("expected a file");
    //
    // let text_field = Text::new(file_text).with_comment("text file");
    //
    // let window = Container::new(Rect::new(Size::new(width, height - 2)), Stack::Horizontally)
    //     .with_comment("window with text file and numbers");
    //
    // let text_numbers = Text::new(
    //     &(1..=window.width())
    //         .map(|n| format!(" {} \n", n))
    //         .collect::<String>(),
    // )
    // .max_width(4)
    // .with_comment("numbers");
    //
    // let window = window.push(text_numbers).push(text_field);
    //
    // let status_bar = Text::new(&format!("Normal Mode, {}", file_name))
    //     .max_height(1)
    //     .with_comment("status bar");
    // let command_bar = Text::new("Command Bar, insert command here")
    //     .max_height(1)
    //     .with_comment("command bar");
    // let revi = Container::new(Rect::new(Size::new(width, height)), Stack::Vertically)
    //     .with_comment("everything")
    //     .stack(Stack::Vertically)
    //     .push(window)
    //     .push(status_bar)
    //     .push(command_bar);
    //
    // revi.draw(
    //     &mut writer,
    //     Rect::with_position(Pos::new(0, 0), Size::new(width, height)),
    // );
    // writer.flush().unwrap();

    let (_, file_name) = {
        let args = std::env::args().take(2).collect::<Vec<String>>();
        (args[0].clone(), args.get(1).unwrap_or(&"".into()).clone())
    };
    let file_text = std::fs::read_to_string(&file_name).expect("expected a file");

    Revi::new(file_text, file_name).run();
    sleep(Duration::from_secs(3));
}
