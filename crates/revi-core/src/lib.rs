// #![warn(clippy::all, clippy::pedantic)]
// ▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
// ▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
// ▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
// ▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀

// mod command_bar;
mod context;
// mod input;
mod pane;
mod settings;
// pub mod api;
pub mod buffer;
pub mod commands;
mod key_parser;
// pub mod line_number;
pub mod mode;
pub mod position;
// pub mod revi;
// mod text_formater;
mod window;

pub mod keymapper;

pub use buffer::Buffer;
pub use context::{Context, ContextBuilder};
// pub use input::Input;
pub use key_parser::KeyParser;
pub use keymapper::Mapper;
pub use mode::Mode;
pub use pane::Pane;
pub use settings::Settings;
pub use window::Window;
