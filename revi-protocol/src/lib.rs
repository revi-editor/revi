use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Command {
    OpenFile {
        path: String,
    },
    InsertText {
        buffer_id: u32,
        position: usize,
        text: String,
    },
    DeleteText {
        buffer_id: u32,
        range: (usize, usize),
    },
    MoveCursor {
        buffer_id: u32,
        direction: Direction,
    },
    SaveFile {
        buffer_id: u32,
    },
    Subscribe {
        buffer_id: u32,
    },
}

// Event from server to client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    FileOpened { buffer_id: u32, content: String },
    BufferUpdated { buffer_id: u32, content: String },
    CursorMoved { buffer_id: u32, position: usize },
    FileSaved { buffer_id: u32 },
    Error { message: String },
}
