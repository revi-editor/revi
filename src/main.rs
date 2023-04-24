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
    commands::{BoxedCommand, InsertChar}, Buffer, Context, ContextBuilder, KeyParser, Mapper, Settings,
    Window, CommandBar, Mode, Pane
};
use revi_ui::{
    tui::{
        application::App,
        container::Container,
        layout::{Pos, Rect, Size, Stack},
        size,
        widget::BoxWidget,
    },
    Key, Keys,
};

use std::{cell::{RefCell, RefMut, Ref}, rc::Rc};
fn execute(context: Context, commands: &[BoxedCommand]) {
    for boxed in commands {
        boxed.command.call(context.clone());
    }
}

#[derive(Debug, Default)]
struct Revi {
    context: Context,
    is_running: bool,
    parse_keys: KeyParser,
    map_keys: Mapper,
}

impl Revi {
    fn get_current_pane(&self) -> Ref<dyn Pane>{
        let id = self.context.focused_pane;
        self.context.panes[id].borrow()
    }

    fn get_current_pane_mut(&self) -> RefMut<dyn Pane>{
        let id = self.context.focused_pane;
        self.context.panes[id].borrow_mut()
    }
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
            is_running: true,
            ..Default::default()
        }
    }

    fn update(&mut self, keys: Keys) {
        if let Keys::KeyAndMod(Key::LC, Key::Ctrl) = keys {
            self.is_running = false;
        }
        let mode = *self.context.mode.borrow();
        self.parse_keys.push(keys);
        let commands = self.map_keys.get_mapping(&mode, self.parse_keys.get_keys());
        eprintln!("{mode:?}: {:?}", self.parse_keys.get_keys());
        if !self.map_keys.is_mapping(&mode, self.parse_keys.get_keys()) {
            self.parse_keys.clear();
        }
        if let Some(cmd) = commands {
            execute(self.context.clone(), cmd);
            self.parse_keys.clear();
        } else if let (None, Mode::Command, Some(c)) = (commands, mode, keys.as_char()){
            let command = InsertChar(c).into();
            execute(self.context.clone(), &[command]);
            self.parse_keys.clear();
        }
        let mode = *self.context.mode.borrow();
        let mut pane = self.get_current_pane_mut();
        pane.update(mode, keys)
    }

    fn quit(&self) -> bool {
        self.is_running
    }

    fn view(&self) -> BoxWidget {
        let id = self.context.focused_pane;
        let main_window = self.context.panes[id].borrow().view();
        let wsize = self.context.main_window_size();
        let command_bar = self.context.command_bar.borrow().view();
        Container::new(Rect::new(wsize), Stack::Vertically)
            .with_child_box(main_window)
            .with_child_box(command_bar)
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
