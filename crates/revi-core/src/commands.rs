use crate::mode::Mode;
use crate::revi::ReVi;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Command {
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    ScrollUp,
    ScrollDown,
    Home,
    End,
    MoveForwardByWord,
    MoveBackwardByWord,
    JumpToFirstLineBuffer,
    JumpToLastLineBuffer,
    Backspace,
    NewLine,
    FirstCharInLine,
    DeleteChar,
    DeleteLine,
    YankLine,
    Paste,
    PasteBack,
    InsertChar(char),
    ChangeMode(Mode),
    EnterCommandMode,
    ExitCommandMode,
    ExcuteCommandLine,
    NextWindow,
    Print(String),
    Save,
    Quit,
}
impl Command {
    pub fn call(&self, revi: &mut ReVi, count: usize) {
        match self {
            Self::CursorUp => cursor_up(revi, count),
            Self::CursorDown => cursor_down(revi, count),
            Self::CursorLeft => cursor_left(revi, count),
            Self::CursorRight => cursor_right(revi, count),
            Self::ScrollUp => scroll_up(revi, count),
            Self::ScrollDown => scroll_down(revi, count),
            Self::Home => home(revi, count),
            Self::End => end(revi, count),
            Self::MoveForwardByWord => move_forward_by_word(revi, count),
            Self::MoveBackwardByWord => move_backward_by_word(revi, count),
            Self::JumpToFirstLineBuffer => jump_to_first_line_buffer(revi, count),
            Self::JumpToLastLineBuffer => jump_to_last_line_buffer(revi, count),
            Self::Backspace => backspace(revi, count),
            Self::NewLine => new_line(revi, count),
            Self::FirstCharInLine => first_char_in_line(revi, count),
            Self::DeleteChar => delete_char(revi, count),
            Self::DeleteLine => delete_line(revi, count),
            Self::YankLine => yank_line(revi, count),
            Self::Paste => paste(revi, count),
            Self::PasteBack => paste_back(revi, count),
            Self::InsertChar(c) => insert_char(revi, count, *c),
            Self::ChangeMode(m) => change_mode(revi, count, *m),
            Self::EnterCommandMode => enter_command_mode(revi, count),
            Self::ExitCommandMode => exit_command_mode(revi, count),
            Self::ExcuteCommandLine => excute_command_line(revi, count),
            Self::NextWindow => next_window(revi, count),
            Self::Print(s) => printer(revi, count, s.as_str()),
            Self::Save => save(revi, count),
            Self::Quit => quit(revi, count),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "cursorup" => Ok(Self::CursorUp),
            "cursordown" => Ok(Self::CursorDown),
            "cursorleft" => Ok(Self::CursorLeft),
            "cursorright" => Ok(Self::CursorRight),
            "scrollup" => Ok(Self::ScrollUp),
            "scrolldown" => Ok(Self::ScrollDown),
            "home" => Ok(Self::Home),
            "end" => Ok(Self::End),
            "moveforwardbyword" => Ok(Self::MoveForwardByWord),
            "movebackwardbyword" => Ok(Self::MoveBackwardByWord),
            "jumptofirstlinebuffer" => Ok(Self::JumpToFirstLineBuffer),
            "jumptolastlinebuffer" => Ok(Self::JumpToLastLineBuffer),
            "backspace" => Ok(Self::Backspace),
            "newline" => Ok(Self::NewLine),
            "firstcharinline" => Ok(Self::FirstCharInLine),
            "deletechar" => Ok(Self::DeleteChar),
            "deleteline" => Ok(Self::DeleteLine),
            "yankline" => Ok(Self::YankLine),
            "paste" => Ok(Self::Paste),
            "pasteback" => Ok(Self::PasteBack),
            // "insertchar" => Ok(Self::InsertChar),
            // "changemode" => Ok(Self::ChangeMode),
            "entercommandmode" => Ok(Self::EnterCommandMode),
            "exitcommandmode" => Ok(Self::ExitCommandMode),
            "excutecommandline" => Ok(Self::ExcuteCommandLine),
            "nextwindow" => Ok(Self::NextWindow),
            // "print" => Ok(Self::Print),
            "save" => Ok(Self::Save),
            "quit" => Ok(Self::Quit),
            _ => Err("this is not a command type"),
        }
    }
}

fn cursor_up(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().move_cursor_up(count);
    revi.queue.push(revi.focused);
}

fn cursor_down(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().move_cursor_down(count);
    revi.queue.push(revi.focused);
}

fn cursor_left(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().move_cursor_left(count);
    revi.queue.push(revi.focused);
}
fn cursor_right(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().move_cursor_right(count);
    revi.queue.push(revi.focused);
}
fn scroll_up(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().scroll_up(count);
    revi.queue.push(revi.focused);
}
fn scroll_down(revi: &mut ReVi, count: usize) {
    revi.focused_window_mut().scroll_down(count);
    revi.queue.push(revi.focused);
}
fn home(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().home();
    revi.queue.push(revi.focused);
}
fn end(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().end();
    revi.queue.push(revi.focused);
}
fn move_forward_by_word(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().move_forward_by_word();
    revi.queue.push(revi.focused);
}
fn move_backward_by_word(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().move_backward_by_word();
    revi.queue.push(revi.focused);
}
fn jump_to_first_line_buffer(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().jump_to_first_line_buffer();
    revi.queue.push(revi.focused);
}
fn jump_to_last_line_buffer(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().jump_to_last_line_buffer();
    revi.queue.push(revi.focused);
}
fn backspace(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().backspace();
    revi.queue.push(revi.focused);
}
fn new_line(revi: &mut ReVi, _: usize) {
    if revi.focused != 0 {
        revi.focused_window_mut().insert_newline();
        revi.queue.push(revi.focused);
    }
}
fn first_char_in_line(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().first_char_in_line();
    revi.queue.push(revi.focused);
}
fn delete_char(revi: &mut ReVi, _: usize) {
    revi.focused_window_mut().delete();
    revi.queue.push(revi.focused);
}
fn delete_line(revi: &mut ReVi, _: usize) {
    let line = revi.focused_window_mut().delete_line();
    revi.queue.push(revi.focused);
    revi.clipboard.clear();
    revi.clipboard.push_str(line.as_str());
}
fn yank_line(revi: &mut ReVi, _: usize) {
    let yanked_line;
    {
        let cursor = revi.focused_window().cursor_file();
        let line = cursor.as_usize_y();
        let buffer = revi.focused_window().buffer();
        yanked_line = buffer.line(line);
    }
    revi.clipboard.clear();
    revi.clipboard.push_str(yanked_line.as_str());
    revi.queue.push(revi.focused);
}
fn paste(revi: &mut ReVi, _: usize) {
    revi.queue.push(revi.focused);
    // TODO: Fix this cloning.
    let clipboard = revi.clipboard.clone();
    {
        let window = revi.focused_window_mut();
        let line_idx = window.cursor_file().as_usize_y();
        let mut buffer = window.buffer_mut();
        buffer.insert_line(line_idx + 1, &clipboard);
    }
    revi.focused_window_mut().move_cursor_down(1);
}
fn paste_back(revi: &mut ReVi, _: usize) {
    revi.queue.push(revi.focused);
    // TODO: Fix this cloning.
    let clipboard = revi.clipboard.clone();
    {
        let window = revi.focused_window_mut();
        let line_idx = window.cursor_file().as_usize_y();
        let mut buffer = window.buffer_mut();
        buffer.insert_line(line_idx + 1, &clipboard);
    }
    revi.focused_window_mut().move_cursor_down(1);
}
fn insert_char(revi: &mut ReVi, _: usize, c: char) {
    revi.focused_window_mut().insert_char(c);
    revi.queue.push(revi.focused);
}
fn change_mode(revi: &mut ReVi, _: usize, mode: Mode) {
    revi.change_modes(mode);
    revi.queue.push(revi.focused);
}
fn enter_command_mode(revi: &mut ReVi, _: usize) {
    revi.enter_command_mode();
    revi.queue.push(revi.focused);
}
fn exit_command_mode(revi: &mut ReVi, _: usize) {
    if revi.focused == 0 {
        revi.exit_command_mode();
        revi.queue.push(revi.focused);
    }
}
fn excute_command_line(revi: &mut ReVi, _: usize) {
    if revi.focused == 0 {
        revi.execute_command_line();
    }
}
fn next_window(revi: &mut ReVi, _: usize) {
    revi.next_window();
    revi.queue.push(revi.focused);
}
fn printer(revi: &mut ReVi, _: usize, string: &str) {
    revi.print(string);
    revi.queue.push(0);
}
fn save(revi: &mut ReVi, _: usize) {
    revi.focused_window().save();
    revi.queue.push(revi.focused);
}
fn quit(revi: &mut ReVi, _: usize) {
    revi.exit();
}

// #[macro_export]
// macro_rules! commands {
//     ( $( $x:ident $(($($args:expr),*))? ),* ) => {
//         vec![$(BoxedCommand { command: Box::new($x $(($($args),*))?) }),*]
//     }
//
// }
