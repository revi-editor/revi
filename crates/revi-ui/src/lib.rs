mod key;
pub mod tui;
pub use crossterm::{style::Stylize, Result};
pub use key::string_to_keys;
pub use key::Key;
pub use key::Keys;
