// #![warn(clippy::all)]
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

mod commandline;


use revi_ui::{
    Keys,
    Key,
    tui::{
        layout::{Size, Pos},
        widget::BoxWidget,
        application::App,
        size,
    }
};
use revi_core::{
    Mode,
    Mapper,
    commands::BoxedCommand,
    Buffer,
    Settings,
    Input,
    Context,
    Window,
};

use std::{
    cell::RefCell,
    rc::Rc,
};
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

impl Revi {
}

impl App for Revi {
    type Settings = Settings;
    fn new(settings: Self::Settings) -> Self {
        let (width, height) = size();
        let buffers = settings.files.into_iter().map(|filename| {
                Rc::new(RefCell::new(Buffer::from_path(filename.as_str())))
            }).collect::<Vec<Rc<RefCell<Buffer>>>>();
        let pane = Rc::new(Window::new(Pos::default(), Size {width, height}, buffers[0].clone()));
        let context = Context {
            buffers,
            panes: vec![pane],
            focused_pane: 0,
            on_screan: vec![0],
            count: 0,
            window_size: Size::new(width, height),
        };
        Self {
            context,
            is_running: true,
            ..Default::default()
        }
    }

    fn update(&mut self, keys: Keys) {
        match &keys {
            Keys::KeyAndMod(Key::LC, Key::Ctrl) => self.is_running = false,
            _ => {},
        }
        let event = match keys {
            Keys::Key(key) => (key, Key::Null),
            Keys::KeyAndMod(key, kmod) => (key, kmod),
        };
        self.input.input(self.mode, event);
        let commands = self.keymapper.get_mapping(self.mode, self.input.keys());
        match (self.mode, commands) {
            (_, Some(cmd)) => {
                execute(self.context.clone(), cmd);
                self.input.clear();
            }
            (Mode::Insert, None) => {},//insert_chars(&mut tui, &mut input, revi.clone()),
            _ => {}
        }
    }

    fn quit(&self) -> bool {
        self.is_running
    }

    fn view(&self) -> BoxWidget {
        let id = self.context.focused_pane;
        self.context.panes[id].view()
    }
}

fn main() {
    let files = commandline::args();
    let settings = Settings {
        files,
    };
    Revi::new(settings).run();
}



// const LINUX_CONFIG_PATH: &str = "/.config/revi/init.rhai";
//
// use revi_core::{
//     commands::{BoxedCommand, InsertChar},
//     Mapper, Mode, ReVi, Settings,
// };
// use revi_ui::{Key, Tui};
//
// use std::cell::RefCell;
// use std::rc::Rc;
// fn execute(revi: Rc<RefCell<ReVi>>, count: usize, commands: &[BoxedCommand]) {
//     for boxed in commands {
//         boxed.command.call(revi.clone(), count);
//     }
// }
//
// fn insert_chars(tui: &mut Tui, input: &mut Input, revi: Rc<RefCell<ReVi>>) {
//     let input_chars = input
//         .as_chars()
//         .iter()
//         .filter(|c| **c != '\0')
//         .map(|c| InsertChar(*c).into())
//         .collect::<Vec<BoxedCommand>>();
//     execute(revi.clone(), input.number_usize(), &input_chars);
//     input.clear();
//     tui.update(&mut *revi.borrow_mut());
// }
//
// fn main() {
//     let home_path = env!("HOME").to_string();
//     let config_file_path = format!("{home_path}{LINUX_CONFIG_PATH}");
//     // let _config_file = std::fs::read_to_string()
//     //     .expect(&format!(
//     //         "failed to read in config file at path '{config_file_path}{LINUX_CONFIG_PATH}'"
//     //     ));
//     let files = commandline::args();
//
//     let settings = Settings::default();
//     let keymapper = Mapper::default();
//     let revi = ReVi::new(settings, &files);
//     let (engine, mut scope) = revi_core::api::init(revi.clone()).expect("failed to init api");
//     engine.eval_file_with_scope::<()>(&mut scope, config_file_path.into()).expect("failed to eval init file");
//
//     let mut tui = Tui::default();
//     let mut input = Input::default();
//
//     input.clear();
//     tui.update(&mut *revi.borrow_mut());
//
//     while revi.borrow().is_running {
//         let mode = revi.borrow().get_current_window().mode;
//
//         if tui.poll_read(std::time::Duration::from_millis(50)) {
//             let keys = tui.get_key_press();
//             input.input(mode, keys);
//             let commands = keymapper.get_mapping(mode, input.keys());
//             match (mode, commands) {
//                 (_, Some(cmd)) => {
//                     execute(revi.clone(), input.number_usize(), cmd);
//                     tui.update(&mut *revi.borrow_mut());
//                     input.clear();
//                 }
//                 (Mode::Insert, None) => insert_chars(&mut tui, &mut input, revi.clone()),
//                 _ => {}
//             }
//         }
//     }
// }
