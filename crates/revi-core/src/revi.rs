use crate::buffer::Buffer;
use crate::line_number::LineNumbers;
use crate::mode::Mode;
use crate::position::Position;
use crate::revi_command::ReViCommand;
use crate::window::Window;
use revi_ui::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ReVi {
    pub is_running: bool,
    size: Position,
    windows: Vec<Window>,
    queue: Vec<usize>,
    buffers: Vec<Rc<RefCell<Buffer>>>,
    focused: usize,
    last_focused: usize,
    clipboard: String,
}

impl ReVi {
    pub fn new(files: &[String]) -> Rc<RefCell<Self>> {
        let mut buffers: Vec<Rc<RefCell<Buffer>>> = files
            .iter()
            .map(|f| Rc::new(RefCell::new(Buffer::from(f.as_str()))))
            .collect();
        if buffers.is_empty() {
            buffers.push(Rc::new(RefCell::new(Buffer::new())));
        }

        let cbuffer = Rc::new(RefCell::new(Buffer::new()));
        buffers.insert(0, Clone::clone(&cbuffer));

        let (w, h) = screen_size();

        // We subtract 1 from the hight here to count for the command bar.
        let h = h.saturating_sub(1);

        // let w1 = w / 2;
        // let w2 = w - w1;
        let window1 = Window::new(w, h, Clone::clone(&buffers[1]))
            .with_status_bar(true)
            .with_line_numbers(LineNumbers::RelativeNumber);
        // let window2 = Window::new(w2, h, Clone::clone(&buffers[2]))
        //     .with_status_bar(true)
        //     .with_line_numbers(LineNumbers::RelativeNumber)
        //     .with_position((window1.width(), 0).into());

        let command_bar = Window::new(w, 1, cbuffer).with_position((0, h + 2).into());

        let windows = vec![command_bar, window1];
        let queue = windows
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let revi = Self {
            size: Position::new_u16(w, h),
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

    pub fn cursor_position_u16(&self) -> (u16, u16) {
        self.windows[self.focused].cursor_screen().as_u16()
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.windows[self.focused].set_cursor(Position::new_u16(x, y));
    }

    pub fn mode(&self) -> &Mode {
        &self.focused_window().mode
    }

    pub fn mode_mut(&mut self) -> &mut Mode {
        &mut self.focused_window_mut().mode
    }

    pub fn focused_window(&self) -> &Window {
        &self.windows[self.focused]
    }

    pub fn focused_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.focused]
    }

    pub fn queued(&self) -> &[usize] {
        &self.queue
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn execute(&mut self, count: usize, commands: &[ReViCommand]) {
        for command in commands {
            match command {
                ReViCommand::StartUp => {}
                ReViCommand::CursorUp => self.focused_window_mut().move_cursor_up(count),
                ReViCommand::CursorDown => self.focused_window_mut().move_cursor_down(count),
                ReViCommand::ScrollUp => self.focused_window_mut().scroll_up(count),
                ReViCommand::ScrollDown => self.focused_window_mut().scroll_down(count),
                ReViCommand::CursorLeft => self.focused_window_mut().move_cursor_left(count),
                ReViCommand::CursorRight => self.focused_window_mut().move_cursor_right(count),
                ReViCommand::Home => self.focused_window_mut().home(),
                ReViCommand::End => self.focused_window_mut().end(),
                ReViCommand::FirstCharInLine => self.focused_window_mut().first_char_in_line(),
                ReViCommand::JumpToFirstLineBuffer => {
                    self.focused_window_mut().jump_to_first_line_buffer()
                }
                ReViCommand::JumpToLastLineBuffer => {
                    self.focused_window_mut().jump_to_last_line_buffer()
                }
                ReViCommand::DeleteChar => self.focused_window_mut().delete(),
                ReViCommand::DeleteLine => self.focused_window_mut().delete_line(),
                ReViCommand::NewLine => self.focused_window_mut().insert_newline(),
                ReViCommand::Backspace => self.focused_window_mut().backspace(),
                ReViCommand::InsertChar(c) => self.focused_window_mut().insert_char(*c),
                ReViCommand::EnterCommandMode => {
                    *self.mode_mut() = Mode::Command;
                    self.last_focused = self.focused.max(1);
                    self.focused = 0;
                    *self.mode_mut() = Mode::Insert;
                }
                ReViCommand::ExitCommandMode => {
                    *self.mode_mut() = Mode::Normal;
                    self.focused = self.last_focused;
                }
                ReViCommand::ExcuteCommandLine if self.focused == 0 => {
                    let string = self.focused_window().buffer().contents();
                    self.focused_window_mut().buffer_mut().clear();
                    self.run_command_line(&string);
                }
                ReViCommand::ExcuteCommandLine => {}
                ReViCommand::NextWindow => {
                    self.focused = if self.focused < self.windows.len().saturating_sub(1) {
                        self.focused + 1
                    } else {
                        1
                    }
                }
                ReViCommand::Mode(m) => {
                    *self.mode_mut() = *m;
                    self.focused_window_mut().adjust_cursor_x();
                }
                ReViCommand::MoveForwardByWord => self.focused_window_mut().move_forward_by_word(),
                ReViCommand::MoveBackwardByWord => {
                    self.focused_window_mut().move_backward_by_word()
                }
                ReViCommand::Save => self.focused_window().save(),
                ReViCommand::Quit => self.exit(),
            }
        }
    }

    fn run_command_line(&mut self, command: &str) {
        match command {
            "q" => self.exit(),
            _ => {}
        }
    }
}

impl revi_ui::Display for ReVi {
    fn render(&self, mut func: impl FnMut(u16, u16, String)) {
        for id in self.queued() {
            let window = &self.windows[*id];
            for data in vec![
                window.get_text_feild(),
                window.get_line_number(),
                window.get_status_bar(),
            ] {
                if let Some(((x, y), text)) = data {
                    func(x, y, text);
                }
            }
        }
    }

    fn cursor(&self, mut func: impl FnMut(u16, u16, Option<revi_ui::CursorShape>)) {
        let window = self.focused_window();
        let (x, y) = window.cursor_screen().as_u16();
        func(x, y, Some(window.mode.shape()));
    }
}
