const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

mod api;
mod buffer;
mod commandline;
mod map_keys;
mod message;
mod parse_keys;
mod state;
mod trie;

// use api::Rhai;
use buffer::Buffer;
use message::Message;
use revi_ui::{application::App, layout::Pos, SetCursorStyle, Subscription};
use state::State;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Insert,
    Command,
    Normal,
}

#[derive(Debug)]
pub struct Settings {
    pub buffers: Vec<Buffer>,
}

enum Revi {
    Editor((Rc<RefCell<State>>, api::Rhai)),
}

impl App for Revi {
    type Settings = Settings;

    type Message = Message;

    fn new(settings: Self::Settings) -> Self {
        let state = State::new(settings);
        let (state, mut rhai) = api::init(state).expect("failed to init rhai");
        rhai.eval_init();
        Self::Editor((state, rhai))
    }

    fn update(&mut self, message: Self::Message) -> Option<Self::Message> {
        match self {
            Self::Editor((state, _)) => state.borrow_mut().update(message),
        }
    }

    fn view(&self) -> revi_ui::widget::BoxWidget {
        match self {
            Self::Editor((ref state, _)) => state.borrow().view(),
        }
    }

    fn cursor_pos(&self) -> Option<Pos> {
        match self {
            Self::Editor((state, _)) => state.borrow().cursor_pos(),
        }
    }
    fn cursor_shape(&self) -> Option<SetCursorStyle> {
        match self {
            Self::Editor((state, _)) => state.borrow().cursor_shape(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self {
            Self::Editor((state, _)) => state.borrow().subscription(),
        }
    }

    fn quit(&self) -> bool {
        match self {
            Self::Editor((state, _)) => state.borrow().quit(),
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let files = commandline::args();
    let buffers = files
        .iter()
        .map(|name| Buffer::from_path(name))
        .collect::<Vec<_>>();
    let settings = Settings { buffers };
    Revi::new(settings).run()?;
    Ok(())
}
