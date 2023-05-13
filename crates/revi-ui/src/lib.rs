mod key;
pub mod tui;
pub use crossterm::{cursor::SetCursorStyle, style::Color, Result};
pub use key::string_to_keys;
pub use key::Key;
pub use key::Keys;
