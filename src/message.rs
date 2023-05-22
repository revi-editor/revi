use revi_ui::{layout::Size, Keys};

use crate::Mode;

#[derive(Debug, Clone)]
pub enum Message {
    CursorDown,
    CursorUp,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    InsertAtEnd,
    BackSpace,
    KeyPress(Keys),
    CheckForMapping,
    ModeCommandInsertStr(String),
    ModeInsertInsertStr(String),
    ChangeMode(Mode),
    ExecuteCommand,
    Resize(Size),
    Quit,
}
