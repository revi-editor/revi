// use crate::Input;
use crate::commandline::{argparser, from_path};
use crate::mode::Mode;
use crate::position::Position;
use crate::revi_command::ReViCommand;
use crate::window::Window;
use revi_ui::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct ReVi {
    pub is_running: bool,
    size: Position,
    windows: Vec<Window>,
    focused: usize,
    command: String,
    clipboard: String,
}

impl ReVi {
    pub fn new() -> Rc<RefCell<Self>> {
        let file_path = argparser();
        let (buffer, path) = from_path(file_path);
        let (w, h) = screen_size();
        let window = Window::new(w, h.saturating_sub(2), buffer, path);
        let windows = vec![window];
        let command = (0..w).map(|_| " ").collect::<String>();
        let revi = Self {
            size: Position::new_u16(w, h),
            is_running: true,
            windows,
            focused: 0,
            command,
            clipboard: String::new(),
        };
        Rc::new(RefCell::new(revi))
    }

    pub fn _windows_locations(&self) -> Vec<(u16, u16)> {
        self.windows
            .iter()
            .map(|w| w.offset().as_u16())
            .collect::<Vec<(u16, u16)>>()
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

    pub fn execute(&mut self, count: usize, commands: &[ReViCommand]) -> Vec<Render> {
        let mut render_commands = Vec::new();
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
                ReViCommand::Mode(m) => {
                    match m {
                        Mode::Normal => render_commands.push(Render::CursorShapeBlock),
                        Mode::Command => {}
                        Mode::Insert => render_commands.push(Render::CursorShapeLine),
                    }
                    *self.mode_mut() = *m;
                    self.focused_window_mut().adjust_cursor_x();
                }
                ReViCommand::MoveForwardByWord => self.focused_window_mut().move_forward_by_word(),
                ReViCommand::MoveBackwardByWord => {
                    self.focused_window_mut().move_backward_by_word()
                }
                ReViCommand::Save => self.focused_window().save(),
                ReViCommand::Quit => self.is_running = false,
            }
        }
        let window = self.focused_window();
        if cfg!(feature = "debug_input_number") {
            render_commands.push(Render::StatusBar {
                pos: (window.status_bar_pos() + Position::new_u16(0, 1)).as_u16(),
                text: format!("input-number: {}                ", count),
            });
        }
        render_commands.push(Render::StatusBar {
            pos: window.status_bar_pos().as_u16(),
            text: window.status_bar(),
        });
        render_commands.push(Render::Window {
            pos: window.offset().as_u16(),
            text: window.to_string(),
        });
        render_commands.push(Render::LineNumbers {
            pos: window.position().as_u16(),
            text: window.line_number(),
        });
        render_commands.push(Render::Cursor(window.cursor_screen().as_u16()));
        render_commands
    }
}
