#![warn(clippy::all, clippy::pedantic)]
// ▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
// ▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
// ▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
// ▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀

mod api;
pub mod buffer;
pub mod commands;
mod key_parser;
pub mod line_number;
pub mod mode;
pub mod position;
pub mod revi;
mod text_formater;
pub mod window;

pub mod keymapper;

pub use buffer::Buffer;
pub use keymapper::Mapper;
pub use mode::Mode;
pub use position::Position;
pub use revi::ReVi;
