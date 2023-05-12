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
    commands::{CmdRc, InsertChar},
    Buffer, CommandBar, Context, ContextBuilder, Event, KeyParser, Mode, Settings, Window,
};
use revi_ui::{
    tui::{
        application::App,
        container::Container,
        layout::{Pos, Rect, Size, Stack},
        size,
        widget::BoxWidget,
    },
    Key, Keys, Result, SetCursorStyle,
};

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Revi {
    context: Context,
    parse_keys: KeyParser,
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
        let context = ContextBuilder::default()
            .with_buffers(buffers)
            .with_panes(vec![pane])
            .with_command_bar(CommandBar::new(Pos { x: 0, y: 0 }, width))
            .with_focused_pane(0)
            .with_on_screen(vec![0])
            .with_window_size(Size::new(width, height))
            .with_show_command_bar(true)
            .build();

        Self {
            context,
            parse_keys: KeyParser::default(),
        }
    }

    fn update(&mut self, keys: Keys) {
        if let Keys::KeyAndMod(Key::LC, Key::Ctrl) = keys {
            *self.context.is_running.borrow_mut() = false;
        }
        let mode = *self.context.mode.borrow();
        self.parse_keys.push(keys);
        let commands = self
            .context
            .map_keys
            .borrow()
            .get_mapping(&mode, self.parse_keys.get_keys());
        let is_possible_mapping = self
            .context
            .map_keys
            .borrow()
            .is_possible_mapping(&mode, self.parse_keys.get_keys());
        if !is_possible_mapping {
            self.parse_keys.clear();
        }
        if let Some(cmd) = commands {
            for _ in 0..self.parse_keys.multiplier {
                cmd.call(self.context.clone());
            }
            self.parse_keys.clear();
        } else if let (None, Mode::Command | Mode::Insert, Some(c)) =
            (commands, mode, keys.as_char())
        {
            let command: CmdRc = InsertChar(c).into();
            command.call(self.context.clone());
            self.parse_keys.clear();
        }
        let mode = *self.context.mode.borrow();
        let pane = self.context.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.update(mode, keys);
        let event = *self.context.event.borrow();
        if let Event::Message = event {
            let closing = pane.close();
            if closing {
                self.context.panes.borrow_mut().pop();
                *self.context.focused_pane.borrow_mut() -= 1;
                *self.context.event.borrow_mut() = Event::None;
                *self.context.mode.borrow_mut() = Mode::Normal;
            }
        }
    }

    fn quit(&self) -> bool {
        *self.context.is_running.borrow()
    }

    fn view(&self) -> BoxWidget {
        let pane = self.context.focused_pane();
        let pane = pane.borrow();
        let main_window = pane.view();
        let wsize = self.context.main_window_size();
        let event = *self.context.event.borrow();
        let mut c = Container::new(Rect::new(wsize), Stack::Vertically).with_child_box(main_window);
        if let Event::None = event {
            let command_bar = self.context.command_bar.borrow().view();
            c.push_box(command_bar);
        }
        c.into()
    }

    fn cursor(&self) -> (Option<Pos>, Option<SetCursorStyle>) {
        let mode = *self.context.mode.borrow();
        let event = *self.context.event.borrow();
        if let Event::Message = event {
            return (None, None);
        }
        match mode {
            Mode::Command => {
                // let pane = self.context.focused_pane();
                // let pane = pane.borrow();
                // let height = pane.view().height();
                let height = self.context.window_size().height;
                let bar = self.context.command_bar.borrow();
                // let pos = pane.cursor();
                let pos = bar.get_cursor_pos().map(|c| {
                    let x = c.pos.x + 1;
                    let y = height;
                    Pos { x, y }
                });
                let style = Some(SetCursorStyle::BlinkingBar);
                (pos, style)
            }
            Mode::Insert => {
                let pane = self.context.focused_pane();
                let pane = pane.borrow();
                let pos = pane.cursor();
                let style = Some(SetCursorStyle::BlinkingBar);
                (pos, style)
            }
            Mode::Normal => {
                let pane = self.context.focused_pane();
                let pane = pane.borrow();
                let pos = pane.cursor();
                let style = Some(SetCursorStyle::BlinkingBlock);
                (pos, style)
            }
        }
    }
}

fn main() -> Result<()> {
    let files = commandline::args();
    let settings = Settings { files };
    Revi::new(settings).run()
}
