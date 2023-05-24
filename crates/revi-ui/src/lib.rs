pub mod application;
pub mod container;
mod key;
pub mod layout;
pub mod runtime;
pub mod subscription;
pub mod text;
pub mod widget;

pub use crossterm::{cursor::SetCursorStyle, event, style, style::Attribute, style::Color, Result};

pub use key::string_to_keys;
pub use key::Key;
pub use key::Keys;
pub use subscription::{Command, Subscription};

use layout::Size;

#[must_use]
pub fn size() -> Size {
    let (width, height) = crossterm::terminal::size().unwrap_or((0, 0));
    Size { width, height }
}
