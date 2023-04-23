// #![warn(clippy::all, clippy::pedantic)]
// ▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
// ▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
// ▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
// ▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀

// mod command_bar;
mod context;
mod pane;
mod settings;
// pub mod api;
pub mod buffer;
pub mod commands;
mod key_parser;
pub mod mode;
mod window;

pub mod keymapper;

pub use buffer::Buffer;
pub use context::{Context, ContextBuilder};
pub use key_parser::KeyParser;
pub use keymapper::Mapper;
pub use mode::Mode;
pub use pane::Pane;
pub use settings::Settings;
pub use window::Window;
