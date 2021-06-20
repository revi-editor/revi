use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Command,
    Insert,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Normal => "Normal",
            Self::Command => "Command",
            Self::Insert => "Insert",
        };
        write!(f, "{}", mode)
    }
}
