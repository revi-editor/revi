use revi_ui::CursorShape::{Block, Line};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Command,
    Insert,
}
impl Mode {
    #[must_use]
    pub fn shape(self) -> revi_ui::CursorShape {
        match self {
            Self::Normal | Self::Command => Block,
            Self::Insert => Line,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Normal => "Normal",
            Self::Command => "Command",
            Self::Insert => "Insert",
        };
        write!(f, "{mode}")
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

// trait Scroll {
//     fn scroll_up(&mut self);
//     fn scroll_down(&mut self);
//     fn scroll_left(&mut self);
//     fn scroll_right(&mut self);
// }
//
// trait Movement {
//     fn move_cursor_up(&mut self);
//     fn move_cursor_down(&mut self);
//     fn move_cursor_left(&mut self);
//     fn move_cursor_right(&mut self);
// }
//
//
// /// Inserting text into buffer and basic movement.
// trait Insert: Movement {
//     fn insert_char(&mut self);
//     fn backspace(&mut self);
//     fn delete(&mut self);
//     fn new_line(&mut self);
// }
// /// This is all of your Movement commands
// trait Normal: Movement + Scroll {
// }
//
// /// Command Bar behavior
// trait Comand: Insert {}
// trait ModalComand: Insert + Normal {}
//
// /// Fuzzy File
// trait Fuzzy: Normal + Insert {}
