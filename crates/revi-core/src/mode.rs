use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Command,
    Insert,
}
// impl Mode {
//     #[must_use]
//     pub fn shape(self) -> revi_ui::CursorShape {
//         match self {
//             Self::Normal | Self::Command => Block,
//             Self::Insert => Line,
//         }
//     }
// }

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
