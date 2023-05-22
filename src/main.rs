const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

mod buffer;
mod commandline;
mod map_keys;
mod message;
mod parse_keys;

use buffer::Buffer;
use map_keys::Mapper;
use message::Message;
use parse_keys::KeyParser;
use revi_ui::{
    application::App,
    container::Container,
    event::Event,
    layout::{Alignment, Pos, Rect, Size, Stack},
    size,
    text::Text,
    Attribute, Color, Keys, Result, SetCursorStyle, Subscription,
};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Insert,
    Command,
    Normal,
}

#[derive(Debug)]
struct Settings {
    buffers: Vec<Buffer>,
}

#[derive(Debug)]
struct Revi {
    focused: usize,
    buffers: Vec<Buffer>,
    command: Buffer,
    map_keys: Mapper,
    key_parse: KeyParser,
    mode: Mode,
    size: Size,
    is_running: bool,
}

impl Revi {
    fn get_focused_buffer(&self) -> &Buffer {
        match self.mode {
            Mode::Command => &self.command,
            _ => &self.buffers[self.focused],
        }
    }

    fn get_focused_buffer_mut(&mut self) -> &mut Buffer {
        match self.mode {
            Mode::Command => &mut self.command,
            _ => &mut self.buffers[self.focused],
        }
    }

    fn cursor_up(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_up();
        None
    }

    fn cursor_down(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_down();
        None
    }

    fn cursor_left(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_left();
        None
    }

    fn cursor_right(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_right();
        None
    }

    fn cursor_home(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_home();
        None
    }

    fn cursor_end(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_end();
        None
    }

    fn insert_at_end(&mut self) -> Option<Message> {
        self.change_mode(Mode::Insert);
        let buf = self.get_focused_buffer_mut();
        buf.cursor_end();
        None
    }

    fn backspace(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.backspace();
        None
    }
    fn key_press(&mut self, keys: Keys) -> Option<Message> {
        self.key_parse.push(keys);
        Some(Message::CheckForMapping)
    }

    fn insert_mode_insert(&mut self, c: impl Into<String>) -> Option<Message> {
        self.buffers[self.focused].insert(c);
        None
    }
    fn change_mode(&mut self, mode: Mode) -> Option<Message> {
        self.get_focused_buffer_mut().align_cursor();
        self.mode = mode;
        None
    }

    fn execute_command(&mut self) -> Option<Message> {
        let command = self
            .command
            .on_screen(2)
            .iter()
            .map(ToString::to_string)
            .collect::<String>();
        eprintln!("{command:?}");
        self.command = Buffer::default();
        self.change_mode(Mode::Normal);
        None
    }

    fn command_mode_insert(&mut self, c: impl Into<String>) -> Option<Message> {
        self.command.insert(c);
        None
    }

    fn check_for_mapping(&mut self) -> Option<Message> {
        let msg = self
            .map_keys
            .get_mapping(&self.mode, self.key_parse.get_keys());
        let is_possible_mapping = self
            .map_keys
            .is_possible_mapping(&self.mode, self.key_parse.get_keys());
        if !is_possible_mapping {
            let key_list = self.key_parse.get_keys();
            let input = key_list
                .iter()
                .filter_map(|k| {
                    k.as_char().and_then(|c| match c {
                        '\0' => None,
                        _ => Some(c),
                    })
                })
                .collect::<Vec<char>>()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("");
            self.key_parse.clear();
            let message = match self.mode {
                Mode::Command => Message::ModeCommandInsertStr(input),
                Mode::Insert => Message::ModeInsertInsertStr(input),
                _ => return None,
            };
            return Some(message);
        }
        if msg.is_some() {
            self.key_parse.clear();
        }
        msg
    }
}

impl App for Revi {
    type Settings = Settings;

    type Message = Message;

    fn new(settings: Self::Settings) -> Self {
        let buffers = if settings.buffers.is_empty() {
            vec![Buffer::default()]
        } else {
            settings.buffers
        };
        Self {
            focused: 0,
            buffers,
            command: Buffer::default(),
            map_keys: Mapper::default(),
            key_parse: KeyParser::default(),
            mode: Mode::Normal,
            size: size(),
            is_running: true,
        }
    }

    fn update(&mut self, message: Self::Message) -> Option<Self::Message> {
        match message {
            Message::CursorUp => self.cursor_up(),
            Message::CursorDown => self.cursor_down(),
            Message::CursorLeft => self.cursor_left(),
            Message::CursorRight => self.cursor_right(),
            Message::CursorHome => self.cursor_home(),
            Message::CursorEnd => self.cursor_end(),
            Message::InsertAtEnd => self.insert_at_end(),
            Message::BackSpace => self.backspace(),
            Message::KeyPress(keys) => self.key_press(keys),
            Message::CheckForMapping => self.check_for_mapping(),
            Message::ModeCommandInsertStr(s) => self.command_mode_insert(s),
            Message::ModeInsertInsertStr(s) => self.insert_mode_insert(s),
            Message::ChangeMode(mode) => self.change_mode(mode),
            Message::ExecuteCommand => self.execute_command(),
            Message::Resize(size) => {
                self.size = size;
                None
            }
            Message::Quit => {
                self.is_running = false;
                None
            }
        }
    }

    fn view(&self) -> revi_ui::widget::BoxWidget {
        let Size { width, height } = self.size;
        let rect = Rect::new(self.size);

        let buf = &self.buffers[self.focused];
        // ------ TEXT AREA --------
        let rect_text = Rect::new(Size {
            width,
            height: height - 2,
        });
        let text = buf
            .on_screen(height)
            .iter()
            .map(|line| Text::new(line.to_string().as_str()).max_width(width))
            .chain(std::iter::repeat(Text::new(" ").max_width(width)))
            .take(height as usize)
            .fold(Container::new(rect_text, Stack::Vertically), |acc, item| {
                acc.push(item)
            });

        // ------ CMD AREA --------
        let rect_cmd = Rect::new(Size { width, height: 1 });
        let src_cmd = self
            .command
            .on_screen(height)
            .iter()
            .map(ToString::to_string)
            .collect::<String>();
        let visable_colon = match self.mode {
            Mode::Command => ":",
            _ => " ",
        };
        let cmd = Container::new(rect_cmd, Stack::Horizontally)
            .push(Text::new(visable_colon).max_width(1))
            .push(Text::new(&src_cmd).max_width(width.saturating_sub(1)));

        // ------ Status Bar AREA --------
        let mode_status = Text::new(&format!("{:?}", self.mode))
            .max_width(8)
            .with_fg(Color::Black)
            .with_bg(Color::White)
            .with_atter(vec![Attribute::Bold, Attribute::Italic].as_slice());

        let filename_status = Text::new(&buf.name)
            .max_width(buf.name.len() as u16)
            .with_fg(Color::Black)
            .with_bg(Color::White)
            .with_atter(vec![Attribute::Bold, Attribute::Italic].as_slice());

        let cursor = buf.get_cursor();
        let cursor_pos_status_width =
            width - (mode_status.char_len() + filename_status.char_len()) as u16;
        let cursor_pos_status = Text::new(&format!("{}/{}", cursor.col(), cursor.row()))
            .max_width(cursor_pos_status_width)
            .with_alignment(Alignment::Right)
            .with_fg(Color::Black)
            .with_bg(Color::White)
            .with_atter(vec![Attribute::Bold, Attribute::Italic].as_slice());

        let rect_status = Rect::new(Size { width, height: 1 });
        let status = Container::new(rect_status, Stack::Horizontally)
            .push(mode_status)
            .push(filename_status)
            .push(cursor_pos_status);

        // ------ Status Bar && CMD combinding AREA --------
        let rect_info = Rect::new(Size { width, height: 2 });
        let info = Container::new(rect_info, Stack::Vertically)
            .push(status)
            .push(cmd);

        // ------ All widgets AREA --------
        Container::new(rect, Stack::Vertically)
            .push(text)
            .push(info)
            .into()
    }

    fn cursor_pos(&self) -> Option<Pos> {
        match self.mode {
            Mode::Command => {
                let cursor = self.command.get_cursor();
                let x = (cursor.col() + 1) as u16;
                let y = (cursor.row() as u16) + self.size.height;
                Some(Pos { x, y })
            }
            _ => {
                let buf = self.get_focused_buffer();
                let cursor = buf.get_cursor();
                Some(cursor.pos())
            }
        }
    }
    fn cursor_shape(&self) -> Option<SetCursorStyle> {
        match self.mode {
            Mode::Normal => Some(SetCursorStyle::BlinkingBlock),
            Mode::Command => Some(SetCursorStyle::BlinkingBar),
            Mode::Insert => Some(SetCursorStyle::BlinkingBar),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none().push(|event| match event {
            Event::Key(k) => Some(Message::KeyPress(Keys::from(k))),
            Event::Resize(w, h) => Some(Message::Resize(Size::new(w, h))),
            _ => None,
        })
    }

    fn quit(&self) -> bool {
        self.is_running
    }
}

fn main() -> Result<()> {
    let files = commandline::args();
    let buffers = files
        .iter()
        .map(|name| Buffer::from_path(name))
        .collect::<Vec<_>>();
    let settings = Settings { buffers };
    Revi::new(settings).run()
}
