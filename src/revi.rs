// use crate::Input;
use crate::mode::Mode;
use crate::position::Position;
use crate::revi_command::ReViCommand;
use crate::ui;
use crate::window::Window;
use crate::InputState;
use ropey::Rope;

#[derive(Debug, Clone)]
pub struct ReVi {
    pub size: Position,
    pub is_running: bool,
    pub windows: Vec<Window>,
    pub focused: usize,
    pub command: String,
}

impl ReVi {
    pub fn new(buffer: Rope, path: Option<String>) -> Self {
        let (w, h) = ui::screen_size();
        let window = Window::new(w, h.saturating_sub(2), buffer, path);
        let windows = vec![window];
        let command = (0..w).map(|_| " ").collect::<String>();
        Self {
            size: Position::new_u16(w, h),
            is_running: true,
            windows,
            focused: 0,
            command,
        }
    }

    pub fn _windows_locations(&self) -> Vec<(u16, u16)> {
        self.windows
            .iter()
            .map(|w| w.window_offset.as_u16())
            .collect::<Vec<(u16, u16)>>()
    }

    pub fn cursor_position_u16(&self) -> (u16, u16) {
        self.windows[self.focused].cursor_screen().as_u16()
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.windows[self.focused].cursor = Position::new_u16(x, y);
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

    fn _command_bar_pos(&self) -> Position {
        Position::new_u16(0, self.size.as_u16_y())
    }

    pub fn execute(
        &mut self,
        count: usize,
        commands: &[ReViCommand],
    ) -> (InputState, Vec<ui::Render>) {
        let state = InputState::Clear;
        let mut render_commands = Vec::new();
        for command in commands {
            match command {
                ReViCommand::StartUp => {
                    // ReLoadPlugins
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::CursorUp => {
                    if self.focused_window_mut().move_cursor_up(count) {
                        render_commands.push(ui::Render::Window {
                            pos: self.focused_window().window_offset,
                            text: self.focused_window().to_string(),
                        });
                    }
                    render_commands.push(ui::Render::StatusBar {
                        pos: self.focused_window().status_bar_pos(),
                        text: self.focused_window().status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(self.focused_window().cursor_screen()));
                }
                ReViCommand::CursorDown => {
                    if self.focused_window_mut().move_cursor_down(count) {
                        render_commands.push(ui::Render::Window {
                            pos: self.focused_window().window_offset,
                            text: self.focused_window().to_string(),
                        });
                    }
                    render_commands.push(ui::Render::StatusBar {
                        pos: self.focused_window().status_bar_pos(),
                        text: self.focused_window().status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(self.focused_window().cursor_screen()));
                }
                ReViCommand::CursorLeft => {
                    self.focused_window_mut().move_cursor_left(count);
                    let window = self.focused_window();
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::CursorRight => {
                    self.focused_window_mut().move_cursor_right(count);
                    let window = self.focused_window().clone();
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::Home => {
                    self.focused_window_mut().home();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::End => {
                    self.focused_window_mut().end();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::DeleteChar => {
                    self.focused_window_mut().delete();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::DeleteLine => {
                    self.focused_window_mut().delete_line();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::NewLine => {
                    self.focused_window_mut().insert_enter();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::Backspace => {
                    self.focused_window_mut().backspace();
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::InsertChar(c) => {
                    let window = self.focused_window_mut();
                    window.insert_char(*c);
                    let window = self.focused_window();
                    render_commands.push(ui::Render::Window {
                        pos: window.window_offset,
                        text: window.to_string(),
                    });
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                    render_commands.push(ui::Render::Cursor(window.cursor_screen()));
                }
                ReViCommand::Mode(m) => {
                    match m {
                        Mode::Normal => render_commands.push(ui::Render::CursorShapeBlock),
                        Mode::Command => {}
                        Mode::Insert => render_commands.push(ui::Render::CursorShapeLine),
                    }
                    *self.mode_mut() = *m;
                    let window = self.focused_window().clone();
                    render_commands.push(ui::Render::StatusBar {
                        pos: window.status_bar_pos(),
                        text: window.status_bar(),
                    });
                }
                ReViCommand::Save => self.focused_window().save(),
                ReViCommand::Quit => self.is_running = false,
            }
        }
        (state, render_commands)
    }
}
