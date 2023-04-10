use crate::buffer::Buffer;
use crate::commands::BoxedCommand;
use crate::line_number::LineNumberKind;
use crate::mode::Mode;
use crate::position::Position;
use crate::window::Window;
use revi_ui::screen_size;
use revi_ui::Stylize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ReVi {
    pub is_running: bool,
    pub windows: Vec<Window>,
    pub queue: Vec<usize>,
    pub buffers: Vec<Rc<RefCell<Buffer>>>,
    pub focused: usize,
    pub last_focused: usize,
    pub clipboard: String,
    pub tab_width: usize,
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
            .with_line_numbers(LineNumberKind::Both);

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
            tab_width: 4,
        };
        Rc::new(RefCell::new(revi))
    }

    pub fn pop_up_window(&mut self, msg: impl Into<String>, pos: Option<Position>) {
        let msg = msg.into();
        let width = msg.lines().map(|line| line.chars().count()).max().unwrap_or(0) as u16;
        let height = msg.lines().count() as u16;
        // NOTE: for some reason adding ╭ fancy char
        // cause the window to not render right.
        const TLC: &str = "+";//"╭";
        const BRC: &str = "+";//"╯";
        const TRC: &str = "+";//"╮";
        const BLC: &str = "+";//"╰";
        const H: &str = "-";  //"⎼";
        const V: &str = "|";  //"│";
        let mut top = TLC.to_string();
        top += &H.repeat(width as usize);
        top += TRC;

        let mut msg = msg.lines().fold(top, |acc, line| {
            format!("{acc}\n{V}{}{V}", format_line(line, width as usize))
        });
        msg += "\n";
        let mut bottom = BLC.to_string();
        bottom += &H.repeat(width as usize);
        bottom += BRC;
        msg += &bottom;

        let pos = pos.unwrap_or({
            let y = self.last_focused_window().height()
                .saturating_sub(height as usize)
                .saturating_sub(1);
            Position::new(0, y)
        });

        let buffer = Rc::new(RefCell::new(Buffer::new_str(msg.trim())));
        let window = Window::new(width+3,height+2,buffer).with_position(pos);
        let id = self.windows.len();
        self.queue.push(id);
        self.windows.push(window);
        self.focused = id;
    }

    #[must_use]
    pub fn get_command_window_mut(&mut self) -> &mut Window {
        &mut self.windows[0]
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
        let end = self.buffers[0].borrow().len_chars();
        self.buffers[0].borrow_mut().insert(end, msg);
        let y = self.buffers[0].borrow().len_lines().saturating_sub(1);
        self.windows[0].goto(Position::new(1, y));
        self.queue.push(0);
    }

    pub fn error_message(&mut self, msgs: &[&str]) {
        self.print(
            &vec![
                msgs[0].black().on_red().to_string(),
                msgs[1].reset().to_string(),
            ]
            .join(" "),
        );
    }
    pub fn list_buffers(&mut self) {
        let list_of_windows = self.buffers
            .iter()
            .enumerate()
            .skip(1)
            .fold(String::new(), |acc, (i, b)| {
            let name = b.borrow()
                .name()
                .clone()
                .unwrap_or("no name(temp)".into());
            format!("{acc}{i} {name}\n")
        });
        self.pop_up_window(list_of_windows, Some(self.last_focused_window().cursor_screen()));
    }

    pub fn close_current_window(&mut self) {
        if self.windows.len() <= 2 {
            self.exit();
        }
        self.windows.remove(self.last_focused);
        self.last_focused -= 1;
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn add_buffer(&mut self, name: impl Into<String>) {
        let buffer = Buffer::from_path(name.into().as_str());
        let idx = self.buffers.len();
        self.buffers.push(Rc::new(RefCell::new(buffer)));
        self.switch_buffers_by_index(idx);
    }

    pub fn switch_buffers_by_index(&mut self, idx: usize) {
        let buffer = self.buffers.get(idx).map(Clone::clone);
        if let Some(b) = buffer {
            self.last_focused_window_mut().set_buffer(b);
        }
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
        let end = self.buffers[0].borrow().len_chars();
        let last_char = self.buffers[0].borrow().get_char(end.saturating_sub(1));
        if last_char == Some('\n') || last_char.is_none() {
            self.buffers[0].borrow_mut().insert_char(end, ':');
        } else if last_char != Some(':') {
            self.buffers[0].borrow_mut().insert(end, "\n:");
        }
        let y = self.buffers[0].borrow().len_lines().saturating_sub(1);
        self.windows[0].goto(Position::new(1, y));
        self.windows[0].move_cursor_right(1);
    }

    pub fn exit_command_mode(&mut self) {
        self.focused = self.last_focused;
        *self.mode_mut() = Mode::Normal;
    }

    pub fn execute_command_line(&mut self) {
        let end = self.buffers[0].borrow().len_lines().saturating_sub(1);
        let mut command = self.buffers[0].borrow().line(end);
        if !command.is_empty() {
            command.remove(0);
        }
        let end = self.buffers[0].borrow().len_chars();
        self.buffers[0].borrow_mut().insert_char(end, '\n');
        self.run_command_line(&command);
    }

    pub fn execute(&mut self, count: usize, commands: &[BoxedCommand]) {
        for boxed in commands {
            boxed.command.call(self, count);
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
            num if num.parse::<usize>().ok().is_some() => {
                let x = self.windows[self.last_focused].cursor_file().as_usize_x();
                let max_y = self.windows[self.last_focused].buffer().len_lines();
                let y = std::cmp::min(max_y, num.parse::<usize>().unwrap());
                let pos = Position::new(x, y);
                self.windows[self.last_focused].goto(pos);
            }
            c if c.starts_with('!') => {
                let command = c[1..].to_string();
                use std::process::Command;
                let output = Command::new(command)
                    .args(items)
                    .output()
                    .expect("failed to run terminal command from revi editor");
                let msgerr = String::from_utf8(output.stderr).expect("failed to turn stderr into a string");
                let msgout = String::from_utf8(output.stdout).expect("failed to turn stdout into a string");
                let msg = format!("{msgerr}{msgout}");
                self.pop_up_window(msg, None);
            }
            "pos" => self.print(&format!("pos: {}", self.last_focused_window().cursor_screen())),
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
            "e" => self.add_buffer(items.join(" ")),
            "q" => self.close_current_window(),
            "w" => self.windows[self.last_focused].save(),
            "wq" => {
                self.windows[self.last_focused].save();
                self.exit();
            }
            "b" if !items.is_empty() => {
                if let Some(idx) = items.first().and_then(|i| i.parse::<usize>().ok()) {
                    self.switch_buffers_by_index(idx)
                }
            }
            "clipboard" => self.print(self.clipboard.clone().as_str()),
            "print" => self.print(&items.join(" ")),
            "set" if !items.is_empty() => match items.first().copied().unwrap_or_default() {
                "number" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::AbsoluteNumber);
                }
                "relativenumber" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::RelativeNumber);
                }
                "nonumber" | "norelativenumber" => {
                    self.windows[self.last_focused].set_number(LineNumberKind::None);
                }
                e => self.error_message(&["unknown command: ", e]),
            },
            "help" => {
                self.error_message(&["help command is not implemented just yet", "help"]);
            }
            "ls" => self.list_buffers(),
            e => self.error_message(&["unknown command: ", e]),
        }
    }
}

impl revi_ui::Display<String> for ReVi {
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
        debug_assert_eq!(self.queue.len(), 0);
    }

    fn cursor(&self, mut func: impl FnMut(u16, u16, Option<revi_ui::CursorShape>)) {
        let window = self.focused_window();
        let (x, y) = window.cursor_screen().as_u16();
        func(x, y, Some(window.mode.shape()));
    }
}

fn format_line(line: &str, width: usize) -> String {
    // 9608 is the block char for debugging
    let blank = ' ';// std::char::from_u32(9608).unwrap_or('&');
    line.chars()
        .chain(std::iter::repeat(blank))
        .take(width)
        .collect::<String>()
}

