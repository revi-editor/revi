const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";
mod buffer;
mod cli;
mod command;
mod key;
mod map_keys;
// mod message;
mod parse_keys;
// mod state;
mod trie;
use buffer::Buffer;
use clap::Parser;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

use map_keys::Mapper;
use parse_keys::KeyParser;
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    App::run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Command,
    Normal,
}

struct Editor {
    mode: Mode,
    buffers: Vec<Buffer>,
    map_keys: Mapper,
    key_parse: KeyParser,
    is_running: bool,
}

struct App;
impl App {
    fn run() -> io::Result<()> {
        enable_raw_mode()?;
        let args = cli::Cli::parse();
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let buffers = args.files.iter().map(Buffer::from_path).collect::<Vec<_>>();

        let mut editor = Editor {
            mode: Mode::Normal,
            map_keys: Mapper::default(),
            key_parse: KeyParser::default(),
            buffers,
            is_running: true,
        };

        while editor.is_running {
            terminal.draw(ui(&editor))?;
            let Some(command) = handle_events(&mut editor)? else {
                continue;
            };
            command.call(&mut editor);
        }
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        for buffer in &self.buffers {
            let _ = buffer.save(None);
        }
        disable_raw_mode().expect("Could not disable raw mode");
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}

fn handle_events(editor: &mut Editor) -> io::Result<Option<command::CmdRc>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            let key = key::Keys::from(key);
            editor.key_parse.push(key);
            let cmd = editor
                .map_keys
                .get_mapping(&editor.mode, editor.key_parse.get_keys());
            let is_possible_mapping = editor
                .map_keys
                .is_possible_mapping(&editor.mode, editor.key_parse.get_keys());
            if !is_possible_mapping {
                let key_list = editor.key_parse.get_keys();
                let _input = key_list
                    .iter()
                    .filter_map(|k| {
                        k.as_char().and_then(|c| match c {
                            '\0' => None,
                            _ => Some(c),
                        })
                    })
                    .collect::<Vec<char>>()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("");
                editor.key_parse.clear();
                let _message = match editor.mode {
                    Mode::Command => {}
                    Mode::Insert => {}
                    _ => {}
                };
                // return Some(message);
            }
            if cmd.is_some() {
                editor.key_parse.clear();
            }
            return Ok(cmd);
        }
    }
    Ok(None)
}

fn ui(editor: &Editor) -> impl Fn(&mut Frame) + '_ {
    let ui = move |frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new("hello".to_string()).block(
                Block::default()
                    .title(format!("{:?}", editor.mode.clone()))
                    .borders(Borders::ALL),
            ),
            frame.size(),
        );
    };
    ui
}
//
// mod buffer;
// mod commandline;
// mod map_keys;
// mod message;
// mod parse_keys;
// mod state;
// mod trie;
//
// use buffer::Buffer;
// use clap::Parser;
// use message::Message;
// use revi_ui::{application::App, layout::Pos, SetCursorStyle, Subscription};
// use state::State;
// use std::cell::RefCell;
// use std::rc::Rc;
//
// #[derive(Debug, Clone, Copy)]
// pub enum Mode {
//     Insert,
//     Command,
//     Normal,
// }
//
// #[derive(Debug)]
// pub struct Settings {
//     pub buffers: Vec<Buffer>,
// }
//
// enum Revi {
//     Editor(Rc<RefCell<State>>),
// }
//
// impl App for Revi {
//     type Settings = Settings;
//
//     type Message = Message;
//
//     fn new(settings: Self::Settings) -> Self {
//         let state = State::new(settings);
//         let state = Rc::new(RefCell::new(state));
//         Self::Editor(state)
//     }
//
//     fn update(&mut self, message: Self::Message) -> Option<Self::Message> {
//         match self {
//             Self::Editor(state) => state.borrow_mut().update(message),
//         }
//     }
//
//     fn view(&self) -> revi_ui::widget::BoxWidget {
//         match self {
//             Self::Editor(ref state) => state.borrow().view(),
//         }
//     }
//
//     fn cursor_pos(&self) -> Option<Pos> {
//         match self {
//             Self::Editor(state) => state.borrow().cursor_pos(),
//         }
//     }
//     fn cursor_shape(&self) -> Option<SetCursorStyle> {
//         match self {
//             Self::Editor(state) => state.borrow().cursor_shape(),
//         }
//     }
//
//     fn subscription(&self) -> Subscription<Self::Message> {
//         match self {
//             Self::Editor(state) => state.borrow().subscription(),
//         }
//     }
//
//     fn quit(&self) -> bool {
//         match self {
//             Self::Editor(state) => state.borrow().quit(),
//         }
//     }
// }
//
// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let files = commandline::Cli::parse().files;
//     let buffers = files
//         .iter()
//         .map(|name| Buffer::from_path(name))
//         .collect::<Vec<_>>();
//     let settings = Settings { buffers };
//     Revi::new(settings).run()?;
//     Ok(())
// }
