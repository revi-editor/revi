use crate::mode::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReViCommand {
    StartUp,
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    Backspace,
    NewLine,
    DeleteChar,
    InsertChar(char),
    Mode(Mode),
    Save,
    Quit,
}
