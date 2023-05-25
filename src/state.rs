use revi_ui::application::App;
use revi_ui::container::Container;
use revi_ui::event::Event;
use revi_ui::layout::{Alignment, Pos, Rect, Stack};
use revi_ui::text::Text;
use revi_ui::{layout::Size, size, Keys};
use revi_ui::{Attribute, Color, SetCursorStyle, Subscription};

use super::{Mode, Settings};
use crate::buffer::Buffer;
use crate::map_keys::Mapper;
use crate::message::Message;
use crate::message::UserMessageBuilder;
use crate::parse_keys::KeyParser;

#[derive(Debug)]
pub struct State {
    pub focused: usize,
    pub buffers: Vec<Buffer>,
    pub messages: Vec<UserMessageBuilder>,
    pub command: Buffer,
    pub map_keys: Mapper,
    pub key_parse: KeyParser,
    pub mode: Mode,
    pub size: Size,
    pub is_running: bool,
}

impl State {
    pub fn set_new_buffer_as_focused(&mut self, buf: Buffer) {
        let idx = self.buffers.len();
        self.buffers.push(buf);
        self.focused = idx;
    }

    pub fn get_focused_buffer(&self) -> &Buffer {
        match self.mode {
            Mode::Command => &self.command,
            _ => &self.buffers[self.focused],
        }
    }

    pub fn get_focused_buffer_mut(&mut self) -> &mut Buffer {
        match self.mode {
            Mode::Command => &mut self.command,
            _ => &mut self.buffers[self.focused],
        }
    }

    pub fn cursor_up(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        if buf.cursor_up() {
            return None;
        }
        Some(Message::ScrollUp)
    }

    pub fn cursor_down(&mut self) -> Option<Message> {
        let Size { height, .. } = self.size;
        let height = (height - 3) as usize;
        let buf = self.get_focused_buffer_mut();
        if buf.cursor_down(height) {
            return None;
        }
        Some(Message::ScrollDown)
    }

    pub fn cursor_left(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_left();
        None
    }

    pub fn cursor_right(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_right();
        None
    }

    pub fn cursor_home(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_home();
        None
    }

    pub fn cursor_end(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.cursor_end();
        None
    }

    pub fn scroll_up(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.scroll_up();
        None
    }

    pub fn scroll_down(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.scroll_down();
        None
    }

    pub fn insert_at_end(&mut self) -> Option<Message> {
        self.change_mode(Mode::Insert);
        let buf = self.get_focused_buffer_mut();
        buf.cursor_end();
        None
    }

    pub fn backspace(&mut self) -> Option<Message> {
        let buf = self.get_focused_buffer_mut();
        buf.backspace();
        None
    }

    pub fn user_message(&mut self, builder: UserMessageBuilder) -> Option<Message> {
        self.messages.push(builder);
        None
    }

    pub fn key_press(&mut self, keys: Keys) -> Option<Message> {
        self.key_parse.push(keys);
        Some(Message::CheckForMapping)
    }

    pub fn insert_mode_insert(&mut self, c: impl Into<String>) -> Option<Message> {
        self.buffers[self.focused].insert(c);
        None
    }
    pub fn change_mode(&mut self, mode: Mode) -> Option<Message> {
        self.get_focused_buffer_mut().align_cursor();
        self.mode = mode;
        None
    }

    pub fn execute_command(&mut self) -> Option<Message> {
        let command = self
            .command
            .on_screen(2)
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .trim()
            .to_string();
        self.command = Buffer::default();
        let (cmd, tail) = command.split_once(' ').unwrap_or((command.as_str(), ""));
        self.change_mode(Mode::Normal);
        match cmd {
            "quit" | "exit" | "q" => Some(Message::Quit),
            "ls" => Some(Message::BufferList),
            "edit" | "e" => Some(Message::EditFile(tail.to_string())),
            "buffer" | "b" => Some(Message::SwapBuffer(tail.to_string())),
            _ => Some(
                UserMessageBuilder::default()
                    .message(command)
                    .footer("UnKnown Command")
                    .fg(Color::Red)
                    .build(),
            ),
        }
    }

    pub fn buffer_list_command(&mut self) -> Option<Message> {
        let paths = self
            .buffers
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{} {}", i, b.name))
            .collect::<Vec<String>>();
        let msg = paths.join("\n");
        Some(
            UserMessageBuilder::default()
                .message(msg)
                .footer("ls List Buffers")
                .fg(Color::Red)
                .build(),
        )
    }

    pub fn edit_file_command(&mut self, filename: &str) -> Option<Message> {
        let buf = Buffer::from_path(filename);
        self.set_new_buffer_as_focused(buf);
        None
    }

    pub fn swap_buffer_command(&mut self, arg: &str) -> Option<Message> {
        if arg.is_empty() {
            unimplemented!("Message to user");
        }
        if let Ok(idx) = arg.parse::<usize>() {
            if self.buffers.get(idx).is_none() {
                unimplemented!("Message to user");
                // Message(
                //     "buffer id does not exsist".into(),
                //     command.into())
                //     .call(ctx.clone());
                // return;
            };
            self.focused = idx;
            return None;
        }
        let Some(idx) = self.buffers
            .iter()
            .enumerate()
            .find(|(_, b)| b.name == arg)
            .map(|(i, _)|i)
        else {
            unimplemented!("Message to user");
            // Message(
            //     "name buffer open with that name or path".into(),
            //     command.into())
            //     .call(ctx.clone());
            // return;
        };
        self.focused = idx;
        None
    }

    pub fn close_message(&mut self) -> Option<Message> {
        self.messages.pop();
        None
    }

    pub fn command_mode_insert(&mut self, c: impl Into<String>) -> Option<Message> {
        self.command.insert(c);
        None
    }

    pub fn check_for_mapping(&mut self) -> Option<Message> {
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

impl App for State {
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
            messages: Vec::new(),
            command: Buffer::default(),
            map_keys: Mapper::default(),
            key_parse: KeyParser::default(),
            mode: Mode::Normal,
            size: size(),
            is_running: true,
        }
    }

    fn view(&self) -> revi_ui::widget::BoxWidget {
        let Size { width, height } = self.size;
        let rect = Rect::new(self.size);

        if let Some(builder) = self.messages.last() {
            return builder.as_view(rect).into();
        }

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
        let pos = cursor.pos();
        let row = pos.x;
        let col = pos.y;
        let cursor_pos_status = Text::new(&format!("{col}/{row}"))
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

    fn update(&mut self, message: Self::Message) -> Option<Self::Message> {
        match message {
            Message::CursorUp => self.cursor_up(),
            Message::CursorDown => self.cursor_down(),
            Message::CursorLeft => self.cursor_left(),
            Message::CursorRight => self.cursor_right(),
            Message::CursorHome => self.cursor_home(),
            Message::CursorEnd => self.cursor_end(),
            Message::ScrollUp => self.scroll_up(),
            Message::ScrollDown => self.scroll_down(),
            Message::InsertAtEnd => self.insert_at_end(),
            Message::BackSpace => self.backspace(),
            Message::UserMessage(builder) => self.user_message(builder),
            Message::KeyPress(keys) => self.key_press(keys),
            Message::CheckForMapping => self.check_for_mapping(),
            Message::ModeCommandInsertStr(s) => self.command_mode_insert(s),
            Message::ModeInsertInsertStr(s) => self.insert_mode_insert(s),
            Message::ChangeMode(mode) => self.change_mode(mode),
            Message::ExecuteCommand => self.execute_command(),
            Message::BufferList => self.buffer_list_command(),
            Message::EditFile(ref filename) => self.edit_file_command(filename),
            Message::SwapBuffer(ref arg) => self.swap_buffer_command(arg),
            Message::CloseCurrentPaneOnKeyPress => self.close_message(),
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

    fn cursor_pos(&self) -> Option<Pos> {
        match self.mode {
            Mode::Command => {
                let cursor = self.command.get_cursor();
                let x = cursor.pos.x + 1;
                let y = cursor.pos.y + self.size.height;
                Some(Pos { x, y })
            }
            _ => {
                let buf = self.get_focused_buffer();
                let cursor = buf.get_cursor();
                Some(cursor.pos)
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
        let close_message = !self.messages.is_empty();
        Subscription::none().push(move |event| match event {
            Event::Key(k) => {
                if close_message {
                    return Some(Message::CloseCurrentPaneOnKeyPress);
                }
                Some(Message::KeyPress(Keys::from(k)))
            }
            Event::Resize(w, h) => Some(Message::Resize(Size::new(w, h))),
            _ => None,
        })
    }

    fn quit(&self) -> bool {
        self.is_running
    }
}
