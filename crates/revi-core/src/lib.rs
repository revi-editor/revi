// #![warn(clippy::all, clippy::pedantic)]
// ▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
// ▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
// ▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
// ▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀

mod pane;
mod input;
mod settings;
mod context;
mod command_bar;
// pub mod api;
pub mod buffer;
pub mod commands;
mod key_parser;
pub mod line_number;
pub mod mode;
pub mod position;
// pub mod revi;
mod text_formater;
mod window;

pub mod keymapper;

pub use buffer::Buffer;
pub use keymapper::Mapper;
pub use input::Input;
pub use mode::Mode;
pub use settings::Settings;
pub use context::Context;
pub use pane::Pane;
pub use window::Window;
