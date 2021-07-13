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

    pub fn next_window(&mut self) {
        self.focused = if self.focused < self.windows.len().saturating_sub(1) {
            self.focused + 1
        } else {
            1
        }
    }

    pub fn change_modes(&mut self, mode: &Mode) {
        *self.mode_mut() = *mode;
        self.focused_window_mut().adjust_cursor_x();
    }

    pub fn enter_command_mode(&mut self) {
        *self.mode_mut() = Mode::Command;
        self.last_focused = self.focused.max(1);
        self.focused = 0;
        *self.mode_mut() = Mode::Insert;
    }

    pub fn exit_command_mode(&mut self) {
        *self.mode_mut() = Mode::Normal;
        self.focused = self.last_focused;
    }

    pub fn execute_command_line(&mut self) {
        let string = self.focused_window().buffer().contents();
        let new_buffer = Rc::new(RefCell::new(Buffer::new()));
        let _ = self.buffers.remove(0);
        self.buffers.insert(0, Clone::clone(&new_buffer));
        self.focused_window_mut().set_buffer(new_buffer);
        self.run_command_line(&string);
    }

    pub fn execute(&mut self, count: usize, commands: &[ReViCommand]) {
        use ReViCommand::*;
        for command in commands {
            match command {
                StartUp => {}
                CursorUp => self.focused_window_mut().move_cursor_up(count),
                CursorDown => self.focused_window_mut().move_cursor_down(count),
                ScrollUp => self.focused_window_mut().scroll_up(count),
                ScrollDown => self.focused_window_mut().scroll_down(count),
                CursorLeft => self.focused_window_mut().move_cursor_left(count),
                CursorRight => self.focused_window_mut().move_cursor_right(count),
                Home => self.focused_window_mut().home(),
                End => self.focused_window_mut().end(),
                FirstCharInLine => self.focused_window_mut().first_char_in_line(),
                JumpToFirstLineBuffer => self.focused_window_mut().jump_to_first_line_buffer(),
                JumpToLastLineBuffer => self.focused_window_mut().jump_to_last_line_buffer(),
                DeleteChar => self.focused_window_mut().delete(),
                DeleteLine => self.focused_window_mut().delete_line(),
                NewLine => self.focused_window_mut().insert_newline(),
                Backspace => self.focused_window_mut().backspace(),
                InsertChar(c) => self.focused_window_mut().insert_char(*c),
                EnterCommandMode => self.enter_command_mode(),
                ExitCommandMode => self.exit_command_mode(),
                ExcuteCommandLine if self.focused == 0 => self.execute_command_line(),
                ExcuteCommandLine => {}
                NextWindow => self.next_window(),
                Mode(m) => self.change_modes(m),
                MoveForwardByWord => self.focused_window_mut().move_forward_by_word(),
                MoveBackwardByWord => self.focused_window_mut().move_backward_by_word(),
                Save => self.focused_window().save(),
                Quit => self.exit(),
            }
        }
    }

    fn run_command_line(&mut self, command: &str) {
        let mut items: Vec<&str> = command.split(' ').collect();
        match items.remove(0) {
            "q" => self.exit(),
            "b" if !items.is_empty() => {
                if let Some(i) = items.get(0).map(|i| i.parse::<usize>().ok()).flatten() {
                    let buffer = self.buffers.get(i).map(|rc| Clone::clone(rc));
                    if let Some(b) = buffer {
                        self.focused = self.last_focused;
                        self.focused_window_mut().set_buffer(b);
                    }
                }
            }
            _ => {}
        }
    }
}

impl revi_ui::Display for ReVi {
    fn render(&self, mut func: impl FnMut(u16, u16, String)) {
        for id in self.queued() {
            let window = &self.windows[*id];
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
    }

    fn cursor(&self, mut func: impl FnMut(u16, u16, Option<revi_ui::CursorShape>)) {
        let window = self.focused_window();
        let (x, y) = window.cursor_screen().as_u16();
        func(x, y, Some(window.mode.shape()));
    }
}
