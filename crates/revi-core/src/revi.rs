// TODO: turn pop_up_window into its own method and make a new method called
// message_window that works more like vim's message window.
// TODO: implement Undo
// TODO: implement Redo
use crate::buffer::Buffer;
use crate::commands::BoxedCommand;
use crate::line_number::LineNumberKind;
use crate::mode::Mode;
use crate::position::Position;
use crate::window::Window;
use revi_ui::screen_size;
use revi_ui::Stylize;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub tab_width: usize,
    pub line_number_kind: LineNumberKind,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tab_width: 4,
            line_number_kind: LineNumberKind::None,
        }
    }
}

#[derive(Debug)]
pub struct ReVi {
    pub size: (u16, u16),
    pub is_running: bool,
    pub windows: Vec<Window>,
    pub command_bar: (bool, Window),
    pub queue: Vec<usize>,
    pub buffers: Vec<Rc<RefCell<Buffer>>>,
    pub focused: usize,
    pub clipboard: String,
    pub executed_commands: Vec<BoxedCommand>,
    pub settings: Settings,
    pub message_window: bool,
}

impl ReVi {
    #[must_use]
    pub fn new(settings: Settings, files: &[String]) -> Rc<RefCell<Self>> {
        let mut windows = vec![];
        let buffers: Vec<Rc<RefCell<Buffer>>> = files
            .iter()
            .map(|f| Rc::new(RefCell::new(Buffer::from_path(f.as_str()))))
            .collect();

        let (w, h) = screen_size();

        // We subtract 1 from the height here to count for the command bar.
        let h = h.saturating_sub(1);

        if !buffers.is_empty() {
            eprintln!("LineNumberKind::{:?}", settings.line_number_kind);
            windows.push(
                Window::new(w, h, Clone::clone(&buffers[0]))
                    .with_status_bar(true)
                    .with_line_numbers(settings.line_number_kind),
            );
        }

        let cbuffer = Rc::new(RefCell::new(Buffer::new()));
        let command_window = Window::new(w, 1, cbuffer).with_position((0, h + 2).into());
        let command_bar = (true, command_window);

        let queue = windows
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let revi = Self {
            size: (w, h),
            is_running: true,
            windows,
            command_bar,
            queue,
            buffers,
            focused: 0,
            clipboard: String::new(),
            executed_commands: vec![],
            settings,
            message_window: false,
        };
        Rc::new(RefCell::new(revi))
    }

    #[must_use]
    pub fn width(&self) -> u16 {
        self.size.0
    }

    #[must_use]
    pub fn height(&self) -> u16 {
        self.size.1
    }

    pub fn create_message_window(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        let width = self.width();
        let has_status_bar = self.get_current_window().has_status_bar();
        let height = (msg.lines().count() as u16 + 2) + u16::from(has_status_bar);
        let header = " ".repeat(width as usize).on_grey();
        let reset = "".reset();
        let msg = format!("{header}{reset}\n{msg}");
        let y = self.height().saturating_sub(height) as usize;
        let pos = Position::new(0, y);
        let mut buffer = Buffer::new_str(msg.trim());
        buffer.read_only = true;
        let buffer = Rc::new(RefCell::new(buffer));
        let mut window = Window::new(width, height, buffer).with_position(pos);
        window.mode = Mode::Command;
        let id = self.windows.len();
        self.queue.push(id);
        self.windows.push(window);
        // self.focused = id;
        self.message_window = true;
    }

    pub fn pop_up_window(&mut self, msg: impl Into<String>, pos: Option<Position>) {
        let msg = msg.into();
        let width = msg
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u16;
        let height = msg.lines().count() as u16;
        // NOTE: for some reason adding ╭ fancy char
        // cause the window to not render right.
        const TLC: &str = "+"; //"╭";
        const BRC: &str = "+"; //"╯";
        const TRC: &str = "+"; //"╮";
        const BLC: &str = "+"; //"╰";
        const H: &str = "-"; //"⎼";
        const V: &str = "|"; //"│";
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
            let y = self
                .focused_window()
                .height()
                .saturating_sub(height as usize)
                .saturating_sub(1);
            Position::new(0, y)
        });

        let buffer = Rc::new(RefCell::new(Buffer::new_str(msg.trim())));
        let window = Window::new(width + 3, height + 2, buffer).with_position(pos);
        let id = self.windows.len();
        self.queue.push(id);
        self.windows.push(window);
        self.focused = id;
    }

    #[must_use]
    pub fn get_command_window(&mut self) -> &Window {
        &self.command_bar.1
    }

    #[must_use]
    pub fn get_command_window_mut(&mut self) -> &mut Window {
        &mut self.command_bar.1
    }

    // gets current &Window
    #[must_use]
    pub fn get_current_window(&self) -> &Window {
        &self.windows[self.focused]
    }

    // gets current &mut Window
    #[must_use]
    pub fn get_current_window_mut(&mut self) -> &mut Window {
        &mut self.windows[self.focused]
    }

    #[must_use]
    pub fn cursor_position_u16(&self) -> (u16, u16) {
        self.windows[self.focused].cursor_screen().as_u16()
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.windows[self.focused].set_cursor(Position::new_u16(x, y));
    }

    /// gets the current focused &Window or command line
    #[must_use]
    pub fn focused_window(&self) -> &Window {
        let mode = self.get_current_window().mode;
        eprintln!("focused window: {mode}");
        match mode {
            Mode::Command => &self.command_bar.1,
            _ => self.get_current_window(),
        }
    }

    /// gets the current focused &mut Window or command line
    #[must_use]
    pub fn focused_window_mut(&mut self) -> &mut Window {
        let mode = self.get_current_window().mode;
        match mode {
            Mode::Command => &mut self.command_bar.1,
            _ => self.get_current_window_mut(),
        }
    }

    #[must_use]
    pub fn queued(&mut self) -> Vec<usize> {
        let queue = self.queue.clone();
        self.queue.clear();
        queue
    }

    pub fn error_message(&mut self, msgs: &str) {
        let reset = "".reset();
        let msg = msgs.black().on_red();
        let msg = format!("{msg}{reset}\n");
        self.create_message_window(&msg);
    }
    pub fn list_buffers(&mut self) {
        let list_of_buffers = self
            .buffers
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, b)| {
                let name = b
                    .as_ref()
                    .borrow()
                    .name()
                    .clone()
                    .unwrap_or("no name(temp)".into());
                format!("{acc}{i} {name}\n")
            });
        self.create_message_window(list_of_buffers);
    }

    pub fn close_message_window(&mut self) {
        self.windows.pop();
        // self.focused = self.focused.saturating_sub(1);
        self.queue.push(self.focused + 1);
        // HACK: Not what i wanted to do but eh.
        // self.message_window = false;
    }

    pub fn close_current_window(&mut self) {
        if self.windows.len() == 1 {
            self.exit();
            return;
        }
        self.windows.remove(self.focused);
        self.focused = self.focused.saturating_sub(1);
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
            self.focused_window_mut().set_buffer(b);
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
        let read_only = self.focused_window().is_read_only();
        let wread_only = self.get_current_window().is_read_only();
        eprintln!("command bar is_read_only: {read_only}, window is read only: {wread_only}");
        if read_only && mode == Mode::Insert {
            self.error_message("file is in read only mode");
            return;
        }
        self.focused_window_mut().mode = mode;
        self.focused_window_mut().adjust_cursor_x();
    }

    pub fn enter_command_mode(&mut self) {
        // TODO: Create a new Window type that is able to reflex the Command bar better.
        // Maybe a trait would be much better suited for the Window struct.
        // This would enable the the creation of BoarderedWindow struct that impl Window
        // trait Window: revi_ui::Display { }
        self.get_current_window_mut().mode = Mode::Command;
        self.get_command_window_mut().mode = Mode::Insert;
        let window = self.get_command_window_mut();
        // Checks to see if the ':' needs to be inserted or not
        let on_last_line = window.is_on_last_line();
        window.jump_to_last_line_buffer();
        window.move_cursor_down(1);
        if on_last_line {
            window.insert_char(':');
        }
    }

    pub fn exit_command_mode(&mut self) {
        self.focused_window_mut().mode = Mode::Normal;
        self.queue.push(self.focused);
    }

    pub fn execute_command_line(&mut self) {
        let window = self.get_command_window_mut();
        let mut line = window.get_current_line();
        if !line.is_empty() {
            line.remove(0);
        }
        self.run_command_line(line.trim());
    }

    /// command arg needs to be not have the ':' prefex
    pub fn run_command_line(&mut self, command: &str) {
        let mut items: Vec<&str> = command.trim().split(' ').collect();
        match items.remove(0) {
            num if num.parse::<usize>().ok().is_some() => {
                let window = self.focused_window_mut();
                let x = window.cursor_file().as_usize_x();
                let max_y = window.buffer().len_lines();
                let y = std::cmp::min(max_y, num.parse::<usize>().unwrap());
                let pos = Position::new(x, y);
                window.goto(pos);
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
                self.create_message_window(msg);
            }
            "e" => self.add_buffer(items.join(" ")),
            "q" => self.close_current_window(),
            "w" => self.focused_window_mut().save(),
            "wq" => {
                self.focused_window_mut().save();
                self.exit();
            }
            "b" if !items.is_empty() => {
                if let Some(idx) = items.first().and_then(|i| i.parse::<usize>().ok()) {
                    self.switch_buffers_by_index(idx)
                }
            }
            "ls" => self.list_buffers(),
            e => self.error_message(&format!("unknown command: {e}")),
        }
    }
}

impl revi_ui::Display<String> for ReVi {
    fn render(&mut self, mut func: impl FnMut(u16, u16, Vec<String>)) {
        if let (true, win) = &self.command_bar {
            if let Some(((x, y), text)) = win.get_text_feild() {
                func(x, y, text);
            }
        }
        for id in self.queued() {
            let Some(window) = &self.windows.get(id) else {
                continue;
            };
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
    let blank = ' '; // std::char::from_u32(9608).unwrap_or('&');
    line.chars()
        .chain(std::iter::repeat(blank))
        .take(width)
        .collect::<String>()
}
