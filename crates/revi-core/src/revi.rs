use crate::buffer::Buffer;
use crate::line_number::LineNumberKind;
use crate::mode::Mode;
use crate::position::Position;
use crate::revi_command::ReViCommand::{
    self, Backspace, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    InsertChar, JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord,
    NewLine, NextWindow, Paste, PasteBack, Print, Quit, Save, ScrollDown, ScrollUp, StartUp,
    YankLine,
};
use crate::window::Window;
use revi_ui::screen_size;
use revi_ui::Stylize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ReVi {
    pub is_running: bool,
    windows: Vec<Window>,
    queue: Vec<usize>,
    buffers: Vec<Rc<RefCell<Buffer>>>,
    focused: usize,
    last_focused: usize,
    clipboard: String,
}

impl ReVi {
    #[must_use]
    pub fn new(files: &[String]) -> Rc<RefCell<Self>> {
        let mut buffers: Vec<Rc<RefCell<Buffer>>> = files
            .iter()
            .map(|f| Rc::new(RefCell::new(Buffer::from_path(f.as_str()))))
            .collect();
        if buffers.is_empty() {
            buffers.push(Rc::new(RefCell::new(Buffer::new())));
        }

        let cbuffer = Rc::new(RefCell::new(Buffer::new()));
        buffers.insert(0, Clone::clone(&cbuffer));

        let (w, h) = screen_size();

        // We subtract 1 from the height here to count for the command bar.
        let h = h.saturating_sub(1);

        let main_window = Window::new(w, h, Clone::clone(&buffers[1]))
            .with_status_bar(true)
            .with_line_numbers(LineNumberKind::RelativeNumber);

        let command_bar = Window::new(w, 1, cbuffer).with_position((0, h + 2).into());

        let windows = vec![command_bar, main_window];
        let queue = windows
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let revi = Self {
            is_running: true,
            windows,
            queue,
            buffers,
            focused: 1,
            last_focused: 1,
            clipboard: String::new(),
        };
        Rc::new(RefCell::new(revi))
    }

    #[must_use]
    pub fn cursor_position_u16(&self) -> (u16, u16) {
        self.windows[self.focused].cursor_screen().as_u16()
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.windows[self.focused].set_cursor(Position::new_u16(x, y));
    }

    #[must_use]
    pub fn mode(&self) -> &Mode {
        &self.focused_window().mode
    }

    #[must_use]
    pub fn mode_mut(&mut self) -> &mut Mode {
        &mut self.focused_window_mut().mode
    }

    #[must_use]
    pub fn last_focused_window(&self) -> &Window {
        &self.windows[self.last_focused]
    }

    #[must_use]
    pub fn last_focused_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.last_focused]
    }

    #[must_use]
    pub fn focused_window(&self) -> &Window {
        &self.windows[self.focused]
    }

    #[must_use]
    pub fn focused_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.focused]
    }

    #[must_use]
    pub fn queued(&mut self) -> Vec<usize> {
        let queue = self.queue.clone();
        self.queue.clear();
        queue
    }

    pub fn print(&mut self, msg: &str) {
        self.buffers[0].borrow_mut().insert(0, msg);
        self.queue.push(0);
    }

    pub fn error_message(&mut self, msgs: Vec<&str>) {
        self.print(
            &vec![
                msgs[0].black().on_red().to_string(),
                msgs[1].reset().to_string(),
            ]
            .join(" "),
        );
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn next_window(&mut self) {
        self.focused = if self.focused < self.windows.len().saturating_sub(1) {
            self.focused + 1
        } else {
            1
        }
    }

    pub fn change_modes(&mut self, mode: Mode) {
        *self.mode_mut() = mode;
        self.focused_window_mut().adjust_cursor_x();
    }

    pub fn enter_command_mode(&mut self) {
        *self.mode_mut() = Mode::Command;
        self.last_focused = self.focused.max(1);
        self.focused = 0;
        *self.mode_mut() = Mode::Insert;
        self.buffers[0].borrow_mut().clear();
        self.buffers[0].borrow_mut().insert_char(0, ':');
        self.windows[0].move_cursor_right(1);
    }

    pub fn exit_command_mode(&mut self) {
        self.focused = self.last_focused;
        *self.mode_mut() = Mode::Normal;
    }

    pub fn execute_command_line(&mut self) {
        let string = self.focused_window().buffer().contents();
        let new_buffer = Rc::new(RefCell::new(Buffer::new()));
        self.buffers.remove(0);
        self.buffers.insert(0, Clone::clone(&new_buffer));
        self.focused_window_mut().set_buffer(new_buffer);
        self.run_command_line(&string);
    }

    pub fn execute(&mut self, count: usize, commands: &[ReViCommand]) {
        // TODO Each Command needs to send there own update as needed.
        for command in commands {
            match command {
                StartUp => {}
                CursorUp => {
                    self.focused_window_mut().move_cursor_up(count);
                    self.queue.push(self.focused);
                }
                CursorDown => {
                    self.focused_window_mut().move_cursor_down(count);
                    self.queue.push(self.focused);
                }
                ScrollUp => {
                    self.focused_window_mut().scroll_up(count);
                    self.queue.push(self.focused);
                }
                ScrollDown => {
                    self.focused_window_mut().scroll_down(count);
                    self.queue.push(self.focused);
                }
                CursorLeft => {
                    self.focused_window_mut().move_cursor_left(count);
                    self.queue.push(self.focused);
                }
                CursorRight => {
                    self.focused_window_mut().move_cursor_right(count);
                    self.queue.push(self.focused);
                }
                Home => {
                    self.focused_window_mut().home();
                    self.queue.push(self.focused);
                }
                End => {
                    self.focused_window_mut().end();
                    self.queue.push(self.focused);
                }
                FirstCharInLine => {
                    self.focused_window_mut().first_char_in_line();
                    self.queue.push(self.focused);
                }
                JumpToFirstLineBuffer => {
                    self.focused_window_mut().jump_to_first_line_buffer();
                    self.queue.push(self.focused);
                }
                JumpToLastLineBuffer => {
                    self.focused_window_mut().jump_to_last_line_buffer();
                    self.queue.push(self.focused);
                }
                DeleteChar => {
                    self.focused_window_mut().delete();
                    self.queue.push(self.focused);
                }
                DeleteLine => {
                    let line = self.focused_window_mut().delete_line();
                    self.queue.push(self.focused);
                    self.clipboard.push_str(line.as_str());
                }
                YankLine => {
                    let yanked_line;
                    {
                        let cursor = self.focused_window().cursor_file();
                        let line = cursor.as_usize_y();
                        let buffer = self.focused_window().buffer();
                        yanked_line = buffer.line(line);
                    }
                    self.clipboard.push_str(yanked_line.as_str());
                    self.queue.push(self.focused);
                }
                PasteBack => {
                    self.queue.push(self.focused);
                    // TODO: Fix this cloning.
                    let clipboard = self.clipboard.clone();
                    {
                        let window = self.focused_window_mut();
                        let line_idx = window.cursor_file().as_usize_y();
                        let mut buffer = window.buffer_mut();
                        buffer.insert_line(line_idx, &clipboard);
                    }
                }
                Paste => {
                    self.queue.push(self.focused);
                    // TODO: Fix this cloning.
                    let clipboard = self.clipboard.clone();
                    {
                        let window = self.focused_window_mut();
                        let line_idx = window.cursor_file().as_usize_y();
                        let mut buffer = window.buffer_mut();
                        buffer.insert_line(line_idx + 1, &clipboard);
                    }
                    self.focused_window_mut().move_cursor_down(1);
                }
                NewLine if self.focused != 0 => {
                    self.focused_window_mut().insert_newline();
                    self.queue.push(self.focused);
                }
                Backspace => {
                    self.focused_window_mut().backspace();
                    self.queue.push(self.focused);
                }
                InsertChar(c) => {
                    self.focused_window_mut().insert_char(*c);
                    self.queue.push(self.focused);
                }
                EnterCommandMode => {
                    self.enter_command_mode();
                    self.queue.push(self.focused);
                }
                ExitCommandMode if self.focused == 0 => {
                    self.exit_command_mode();
                    self.queue.push(self.focused);
                }
                ExcuteCommandLine if self.focused == 0 => self.execute_command_line(), // Maybe
                NextWindow => {
                    self.next_window();
                    self.queue.push(self.focused);
                }
                ChangeMode(m) => {
                    self.change_modes(*m);
                    self.queue.push(self.focused);
                }
                MoveForwardByWord => {
                    self.focused_window_mut().move_forward_by_word();
                    self.queue.push(self.focused);
                }
                MoveBackwardByWord => {
                    self.focused_window_mut().move_backward_by_word();
                    self.queue.push(self.focused);
                }
                Print(msg) => {
                    self.print(msg);
                    self.queue.push(0);
                }
                Save => {
                    self.focused_window().save();
                    self.queue.push(self.focused);
                }
                Quit => self.exit(),
                _ => {}
            }
        }
    }

    // TODO: Make a lexer and parser for this.
    fn run_command_line(&mut self, command: &str) {
        let command: String = if let Some(collon) = command.get(0..1) {
            if collon == ":" {
                command[1..].to_string()
            } else {
                command.to_string()
            }
        } else {
            command.to_string()
        };
        let mut items: Vec<&str> = command.split(' ').collect();
        match items.remove(0) {
            "line" => {
                let line_number = self.windows[self.last_focused].cursor_file().as_usize_y();
                let text = self.windows[self.last_focused].buffer().line(line_number);
                self.print(text.as_str());
            }
            "len" => {
                let line_number = self.windows[self.last_focused].cursor_file().as_usize_y();
                let text = self.windows[self.last_focused].buffer().line(line_number);
                self.print(text.len().to_string().as_str());
            }
            "q" => self.exit(),
            "b" if !items.is_empty() => {
                if let Some(i) = items.get(0).and_then(|i| i.parse::<usize>().ok()) {
                    let buffer = self.buffers.get(i).map(|rc| Clone::clone(rc));
                    if let Some(b) = buffer {
                        // self.focused = self.last_focused;
                        self.last_focused_window_mut().set_buffer(b);
                    }
                }
            }
            "clipboard" => self.print(self.clipboard.clone().as_str()),
            "print" => self.print(&items.join(" ")),
            "set" if !items.is_empty() => match items.get(0).copied().unwrap_or_default() {
                "number" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::AbsoluteNumber)
                }
                "relativenumber" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::RelativeNumber)
                }
                "nonumber" | "norelativenumber" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::None)
                }
                e => self.error_message(vec!["unknown command: ", e]),
            },
            e => self.error_message(vec!["unknown command: ", e]),
        }
    }
}

impl revi_ui::Display for ReVi {
    fn render(&mut self, mut func: impl FnMut(u16, u16, Vec<String>)) {
        for id in self.queued() {
            let window = &self.windows[id];
            if let Some(((x, y), text)) = window.get_text_feild() {
                func(x, y, text);
            }
            if let Some(((x, y), text)) = window.get_line_number() {
                func(x, y, text);
            }
            if let Some(((x, y), text)) = window.get_status_bar() {
                func(x, y, text);
            }
        }
        assert_eq!(self.queue.len(), 0);
    }

    fn cursor(&self, mut func: impl FnMut(u16, u16, Option<revi_ui::CursorShape>)) {
        let window = self.focused_window();
        let (x, y) = window.cursor_screen().as_u16();
        func(x, y, Some(window.mode.shape()));
    }
}
