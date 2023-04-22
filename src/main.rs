// #![warn(clippy::all)]
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

mod commandline;

use revi_core::{
    commands::BoxedCommand, Buffer, Context, ContextBuilder, Input, Mapper, Mode, Settings, Window,
};
use revi_ui::{
    tui::{
        application::App,
        container::Container,
        layout::{Pos, Rect, Size, Stack},
        size,
        text::Text,
        widget::BoxWidget,
    },
    Key, Keys,
};

use std::{cell::RefCell, rc::Rc};
fn execute(context: Context, commands: &[BoxedCommand]) {
    for boxed in commands {
        boxed.command.call(context.clone());
    }
}

#[derive(Debug, Default)]
struct Revi {
    context: Context,
    is_running: bool,
    input: Input,
    mode: Mode,
    keymapper: Mapper,
}

impl App for Revi {
    type Settings = Settings;
    fn new(settings: Self::Settings) -> Self {
        let (width, height) = size();
        let buffers = settings
            .files
            .into_iter()
            .map(|filename| Rc::new(RefCell::new(Buffer::from_path(filename.as_str()))))
            .collect::<Vec<Rc<RefCell<Buffer>>>>();
        let pane = Rc::new(RefCell::new(
            Window::new(
                Pos::default(),
                Size {
                    width,
                    height: height - 1,
                },
                buffers[0].clone(),
            )
            .with_status_bar(true)
            .with_line_numbers(true),
        ));
        // .with_status_bar(true).with_line_numbers(true)
        let context = ContextBuilder::default()
            .with_buffers(buffers)
            .with_panes(vec![pane])
            .with_focused_pane(0)
            .with_on_screen(vec![0])
            .with_window_size(Size::new(width, height))
            .with_show_command_bar(true)
            .build();
        Self {
            context,
            is_running: true,
            ..Default::default()
        }
    }

    fn update(&mut self, keys: Keys) {
        match &keys {
            Keys::KeyAndMod(Key::LC, Key::Ctrl) => self.is_running = false,
            _ => {}
        }
        let event = match keys {
            Keys::Key(key) => (key, Key::Null),
            Keys::KeyAndMod(key, kmod) => (kmod, key),
        };
        self.input.input(self.mode, event);
        let commands = self.keymapper.get_mapping(self.mode, self.input.keys());
        if let Some(cmd) = dbg!(commands) {
            execute(self.context.clone(), cmd);
            self.input.clear();
        }
    }

    fn quit(&self) -> bool {
        self.is_running
    }

    fn view(&self) -> BoxWidget {
        let id = self.context.focused_pane;
        let main_window = self.context.panes[id].borrow().view();
        let wsize = self.context.main_window_size();
        Container::new(Rect::new(wsize), Stack::Vertically)
            .with_child_box(main_window)
            .with_child(Text::new("Command Bar").max_height(1))
            .into()
    }

    fn cursor(&self) -> Option<Pos> {
        let id = self.context.focused_pane;
        self.context.panes[id]
            .borrow()
            .get_cursor_pos()
            .map(|c| c.pos)
    }
}

fn main() {
    let files = commandline::args();
    let settings = Settings { files };
    Revi::new(settings).run();
}
