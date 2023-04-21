use std::{
    rc::Rc,
    cell::RefCell,
};


use revi_ui::tui::{
    widget::Widget,
    layout::{
        Size,
        Pos,
        Rect,
        Stack,
    },
    text::Text,
    container::Container,
};

use crate::{Pane, Buffer};

#[derive(Debug)]
pub struct Window {
    pos: Pos,
    size: Size,
    buffer: Rc<RefCell<Buffer>>,
}

impl Window {
    pub fn new(pos: Pos, size: Size, buffer: Rc<RefCell<Buffer>>) -> Self {
        Self {
            pos,
            size,
            buffer,
        }
    }
}

impl Pane for Window {
    fn view(&self) -> revi_ui::tui::widget::BoxWidget {
        let Size { width, height } = self.size;
        let top = 0;
        let bottom = height as usize;
        let buffer = self.buffer.borrow();
        let contents = buffer.on_screen(top, bottom);
        let text_field = Text::new(&contents).with_comment("text file");

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

        let file_name = buffer.name().unwrap_or("N/A");
        let status_bar = Text::new(&format!("Normal Mode, {}", file_name))
            .max_height(1)
            .with_comment("status bar");

        let command_bar = Text::new("Command Bar, insert command here")
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

    fn update(&mut self, _: revi_ui::Keys) { }
}
