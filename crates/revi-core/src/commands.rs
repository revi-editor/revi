use crate::mode::Mode;
use crate::revi::ReVi;
use std::fmt;

pub trait Command: fmt::Debug {
    fn call(&self, revi: &mut ReVi, count: usize);
    fn id(&self) -> usize;
}

pub struct BoxedCommand {
    pub command: Box<dyn Command>,
}

impl std::fmt::Debug for BoxedCommand {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "BoxedCommand {{ {:?} }}", self.command)
    }
}

impl PartialEq for BoxedCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command.id() == other.command.id()
    }
}

#[derive(Debug, PartialEq)]
pub struct CursorUp;
impl CursorUp {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(CursorUp),
        }
    }
}

impl Command for CursorUp {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().move_cursor_up(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        0
    }
}

#[derive(Debug, PartialEq)]
pub struct CursorDown;
impl CursorDown {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(CursorDown),
        }
    }
}

impl Command for CursorDown {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().move_cursor_down(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        1
    }
}

#[derive(Debug, PartialEq)]
pub struct CursorLeft;
impl CursorLeft {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(CursorLeft),
        }
    }
}

impl Command for CursorLeft {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().move_cursor_left(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        2
    }
}

#[derive(Debug, PartialEq)]
pub struct CursorRight;
impl CursorRight {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(CursorRight),
        }
    }
}

impl Command for CursorRight {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().move_cursor_right(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        3
    }
}

#[derive(Debug, PartialEq)]
pub struct ScrollUp;
impl ScrollUp {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(ScrollUp),
        }
    }
}

impl Command for ScrollUp {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().scroll_up(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        4
    }
}

#[derive(Debug, PartialEq)]
pub struct ScrollDown;
impl ScrollDown {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(ScrollDown),
        }
    }
}

impl Command for ScrollDown {
    fn call(&self, revi: &mut ReVi, count: usize) {
        revi.focused_window_mut().scroll_down(count);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        5
    }
}

#[derive(Debug, PartialEq)]
pub struct Home;
impl Home {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Home),
        }
    }
}

impl Command for Home {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().home();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        6
    }
}

#[derive(Debug, PartialEq)]
pub struct End;
impl End {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(End),
        }
    }
}

impl Command for End {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().end();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        7
    }
}
#[derive(Debug, PartialEq)]
pub struct MoveForwardByWord;
impl MoveForwardByWord {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(MoveForwardByWord),
        }
    }
}

impl Command for MoveForwardByWord {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().move_forward_by_word();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        8
    }
}

#[derive(Debug, PartialEq)]
pub struct MoveBackwardByWord;
impl MoveBackwardByWord {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(MoveBackwardByWord),
        }
    }
}

impl Command for MoveBackwardByWord {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().move_backward_by_word();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        9
    }
}

#[derive(Debug, PartialEq)]
pub struct JumpToFirstLineBuffer;
impl JumpToFirstLineBuffer {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(JumpToFirstLineBuffer),
        }
    }
}

impl Command for JumpToFirstLineBuffer {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().jump_to_first_line_buffer();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        10
    }
}

#[derive(Debug, PartialEq)]
pub struct JumpToLastLineBuffer;
impl JumpToLastLineBuffer {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(JumpToLastLineBuffer),
        }
    }
}

impl Command for JumpToLastLineBuffer {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().jump_to_last_line_buffer();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        11
    }
}

#[derive(Debug, PartialEq)]
pub struct Backspace;
impl Backspace {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Backspace),
        }
    }
}

impl Command for Backspace {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().backspace();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        12
    }
}

#[derive(Debug, PartialEq)]
pub struct NewLine;
impl NewLine {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(NewLine),
        }
    }
}

impl Command for NewLine {
    fn call(&self, revi: &mut ReVi, _: usize) {
        if revi.focused != 0 {
            revi.focused_window_mut().insert_newline();
            revi.queue.push(revi.focused);
        }
    }
    fn id(&self) -> usize {
        13
    }
}

#[derive(Debug, PartialEq)]
pub struct FirstCharInLine;
impl FirstCharInLine {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(FirstCharInLine),
        }
    }
}

impl Command for FirstCharInLine {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().first_char_in_line();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        14
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteChar;
impl DeleteChar {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(DeleteChar),
        }
    }
}

impl Command for DeleteChar {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().delete();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        15
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteLine;
impl DeleteLine {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(DeleteLine),
        }
    }
}

impl Command for DeleteLine {
    fn call(&self, revi: &mut ReVi, _: usize) {
        let line = revi.focused_window_mut().delete_line();
        revi.queue.push(revi.focused);
        revi.clipboard.push_str(line.as_str());
    }
    fn id(&self) -> usize {
        16
    }
}

#[derive(Debug, PartialEq)]
pub struct YankLine;
impl YankLine {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(YankLine),
        }
    }
}

impl Command for YankLine {
    fn call(&self, revi: &mut ReVi, _: usize) {
        let yanked_line;
        {
            let cursor = revi.focused_window().cursor_file();
            let line = cursor.as_usize_y();
            let buffer = revi.focused_window().buffer();
            yanked_line = buffer.line(line);
        }
        revi.clipboard.push_str(yanked_line.as_str());
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        17
    }
}

#[derive(Debug, PartialEq)]
pub struct Paste;
impl Paste {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Paste),
        }
    }
}

impl Command for Paste {
    fn call(&self, revi: &mut ReVi, _: usize) {
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
    fn id(&self) -> usize {
        18
    }
}

#[derive(Debug, PartialEq)]
pub struct PasteBack;
impl PasteBack {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(PasteBack),
        }
    }
}

impl Command for PasteBack {
    fn call(&self, revi: &mut ReVi, _: usize) {
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
    fn id(&self) -> usize {
        19
    }
}

#[derive(Debug, PartialEq)]
pub struct InsertChar(pub char);
impl InsertChar {
    pub fn new_box(c: char) -> BoxedCommand {
        BoxedCommand {
            command: Box::new(InsertChar(c)),
        }
    }
}

impl Command for InsertChar {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window_mut().insert_char(self.0);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        20
    }
}

#[derive(Debug, PartialEq)]
pub struct ChangeMode(Mode);
impl ChangeMode {
    pub fn new_box(mode: Mode) -> BoxedCommand {
        BoxedCommand {
            command: Box::new(ChangeMode(mode)),
        }
    }
}

impl Command for ChangeMode {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.change_modes(self.0);
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        21
    }
}

#[derive(Debug, PartialEq)]
pub struct EnterCommandMode;
impl EnterCommandMode {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(EnterCommandMode),
        }
    }
}

impl Command for EnterCommandMode {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.enter_command_mode();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        22
    }
}

#[derive(Debug, PartialEq)]
pub struct ExitCommandMode;
impl ExitCommandMode {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(ExitCommandMode),
        }
    }
}

impl Command for ExitCommandMode {
    fn call(&self, revi: &mut ReVi, _: usize) {
        if revi.focused == 0 {
            revi.exit_command_mode();
            revi.queue.push(revi.focused);
        }
    }
    fn id(&self) -> usize {
        23
    }
}

#[derive(Debug, PartialEq)]
pub struct ExcuteCommandLine;
impl ExcuteCommandLine {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(ExcuteCommandLine),
        }
    }
}

impl Command for ExcuteCommandLine {
    fn call(&self, revi: &mut ReVi, _: usize) {
        if revi.focused == 0 {
            revi.execute_command_line();
        }
    }
    fn id(&self) -> usize {
        24
    }
}

#[derive(Debug, PartialEq)]
pub struct NextWindow;
impl NextWindow {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(NextWindow),
        }
    }
}

impl Command for NextWindow {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.next_window();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        25
    }
}

#[derive(Debug, PartialEq)]
pub struct Print(String);
impl Print {
    pub fn new_box(string: &str) -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Print(string.to_string())),
        }
    }
}

impl Command for Print {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.print(&self.0);
        revi.queue.push(0);
    }
    fn id(&self) -> usize {
        26
    }
}

#[derive(Debug, PartialEq)]
pub struct Save;
impl Save {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Save),
        }
    }
}

impl Command for Save {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.focused_window().save();
        revi.queue.push(revi.focused);
    }
    fn id(&self) -> usize {
        27
    }
}

#[derive(Debug, PartialEq)]
pub struct Quit;
impl Quit {
    pub fn new_box() -> BoxedCommand {
        BoxedCommand {
            command: Box::new(Quit),
        }
    }
}

impl Command for Quit {
    fn call(&self, revi: &mut ReVi, _: usize) {
        revi.exit();
    }
    fn id(&self) -> usize {
        28
    }
}

#[macro_export]
macro_rules! commands {
    ( $( $x:ident ),* ) => {
        {
            vec![$($x::new_box()),*]
        }
    };
    ( $( $x:ident $($args:expr)*),* ) => {
        {
            vec![$($x::new_box($($args)*)),*]
        }
    };
}
