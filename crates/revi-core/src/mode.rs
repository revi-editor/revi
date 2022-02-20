use revi_ui::CursorShape::{Block, Line};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
        write!(f, "{}", mode)
    }
}

impl TryFrom<&str> for Mode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "command" => Ok(Self::Command),
            "insert" => Ok(Self::Insert),
            _ => Err("this is not a command type"),
        }
    }
}
