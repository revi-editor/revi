use crate::mode::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReViCommand {
    StartUp,
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    Home,
    End,
    Backspace,
    NewLine,
    DeleteChar,
    DeleteLine,
    InsertChar(char),
    Mode(Mode),
    Save,
    Quit,
}
