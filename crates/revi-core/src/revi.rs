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
    buffers: Vec<Buffer>,
    focused: usize,
    clipboard: String,
}

impl ReVi {
    pub fn new(files: &[String]) -> Rc<RefCell<Self>> {
        let mut buffers: Vec<Buffer> = files.iter().map(|f| Buffer::from(f.as_str())).collect();
        buffers.insert(0, Buffer::new());
        let (w, h) = screen_size();

        // We subtract 1 from the hight here to count for the command bar.
        let h = h.saturating_sub(1);
        let window = Window::new(w, h, Some(1))
            .with_status_bar(true)
            .with_line_numbers(LineNumbers::RelativeNumber);
        let command_bar = Window::new(w, 1, Some(0)).with_position((0, h + 2).into());
        let windows = vec![command_bar, window];
        let revi = Self {
            size: Position::new_u16(w, h),
            is_running: true,
            windows,
            buffers,
            focused: 1,
            clipboard: String::new(),
        };
        Rc::new(RefCell::new(revi))
    }

    pub fn get_current_buffer(&self) -> Option<&Buffer> {
        self.focused_window()
            .get_buffer_id()
            .map(|id| self.buffers.get(id))
            .flatten()
    }

    pub fn get_current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        let window = &mut self.windows[self.focused];
        if let Some(id) = window.get_buffer_id() {
            return self.buffers.get_mut(id);
        }
        None
    }

    pub fn cursor_position_u16(&self) -> (u16, u16) {
        if let Some(buffer) = self.get_current_buffer() {
            return self.windows[self.focused].cursor_screen(buffer).as_u16();
        }
        (0, 0)
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

    pub fn focused_window_with_buffer(&self) -> (&Window, Option<&Buffer>) {
        let window = &self.windows[self.focused];
        let buffer = window
            .get_buffer_id()
            .map(|id| self.buffers.get(id))
            .flatten();
        (window, buffer)
    }

    pub fn focused_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.focused]
    }

    pub fn focused_window_mut_with_buffer_mut(&mut self) -> (&mut Window, Option<&mut Buffer>) {
        let window = &mut self.windows[self.focused];
        if let Some(id) = window.get_buffer_id() {
            return (window, self.buffers.get_mut(id));
        }
        (window, None)
    }

    pub fn focused_window_mut_with_buffer(&mut self) -> (&mut Window, Option<&Buffer>) {
        let window = &mut self.windows[self.focused];
        if let Some(id) = window.get_buffer_id() {
            return (window, self.buffers.get(id));
        }
        (window, None)
    }

    pub fn execute(&mut self, count: usize, commands: &[ReViCommand]) {
        for command in commands {
            match command {
                ReViCommand::StartUp => {}
                ReViCommand::CursorUp => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.move_cursor_up(count, b);
                    }
                }
                ReViCommand::CursorDown => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.move_cursor_down(count, b);
                    }
                }
                ReViCommand::ScrollUp => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.scroll_up(count, b);
                    }
                }
                ReViCommand::ScrollDown => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.scroll_down(count, b);
                    }
                }
                ReViCommand::CursorLeft => self.focused_window_mut().move_cursor_left(count),
                ReViCommand::CursorRight => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.move_cursor_right(count, b);
                    }
                }
                ReViCommand::Home => self.focused_window_mut().home(),
                ReViCommand::End => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.end(b);
                    }
                }
                ReViCommand::FirstCharInLine => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.first_char_in_line(b);
                    }
                }
                ReViCommand::JumpToFirstLineBuffer => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.jump_to_first_line_buffer(b);
                    }
                }
                ReViCommand::JumpToLastLineBuffer => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.jump_to_last_line_buffer(b);
                    }
                }
                ReViCommand::DeleteChar => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer_mut() {
                        w.delete(b);
                    }
                }
                ReViCommand::DeleteLine => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer_mut() {
                        w.delete_line(b);
                    }
                }
                ReViCommand::NewLine => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer_mut() {
                        w.insert_newline(b);
                    }
                }
                ReViCommand::Backspace => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer_mut() {
                        w.backspace(b);
                    }
                }
                ReViCommand::InsertChar(c) => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer_mut() {
                        w.insert_char(*c, b);
                    }
                }
                ReViCommand::Mode(m) => {
                    *self.mode_mut() = *m;
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.adjust_cursor_x(b);
                    }
                }
                ReViCommand::MoveForwardByWord => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.move_forward_by_word(b);
                    }
                }
                ReViCommand::MoveBackwardByWord => {
                    if let (w, Some(b)) = self.focused_window_mut_with_buffer() {
                        w.move_backward_by_word(b);
                    }
                }
                ReViCommand::Save => {
                    if let Some(buffer) = self.get_current_buffer() {
                        self.focused_window().save(buffer);
                    }
                }
                ReViCommand::Quit => self.is_running = false,
            }
        }
    }
}

impl revi_ui::Display<()> for ReVi {
    fn render<F: FnMut(u16, u16, String)>(&self, _: &(), func: F) {
        if let Some(b) = self.get_current_buffer() {
            self.focused_window().render(b, func);
        }
    }
    fn line_numbers<F: FnMut(u16, u16, String)>(&self, _: &(), func: F) {
        if let Some(b) = self.get_current_buffer() {
            self.focused_window().line_numbers(b, func);
        }
    }
    fn status_bar<F: FnMut(u16, u16, String)>(&self, _: &(), func: F) {
        if let Some(b) = self.get_current_buffer() {
            self.focused_window().status_bar(b, func);
        }
    }
    fn cursor<F: FnMut(u16, u16, Option<CursorShape>)>(&self, _: &(), func: F) {
        if let Some(b) = self.get_current_buffer() {
            self.focused_window().cursor(b, func);
        }
    }
}
