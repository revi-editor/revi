const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

mod buffer;
mod cli;
mod command;
mod key;
mod map_keys;
mod parse_keys;
mod trie;
mod tui;
use anyhow::Result;
use clap::Parser;
use tui::{App, LineNumbers, Pane, StatusBar, Theme};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

use map_keys::Mapper;
use parse_keys::KeyParser;
use ratatui::{
    layout::{Position, Size},
    prelude::*,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Editor::new()?.run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Command,
    Normal,
}

type Tui = Terminal<CrosstermBackend<Stdout>>;

struct Editor {
    mode: Mode,
    buffers: Vec<buffer::Buffer>,
    buffer_index: usize,
    is_running: bool,
    map_keys: Mapper,
    key_parse: KeyParser,
    current_pane_size: Size,
}

impl Editor {
    fn new() -> Result<Self> {
        let args = cli::Cli::parse();
        let mut buffers = args
            .files
            .iter()
            .map(buffer::Buffer::from_path)
            .collect::<Vec<_>>();
        // Add welcome message if no file is provided
        if buffers.is_empty() {
            // This should be a welcome message not an empty buffer
            buffers.push(buffer::Buffer::default());
        }
        // First buffer is always command buffer
        buffers.insert(0, buffer::Buffer::default());
        Ok(Self {
            mode: Mode::Normal,
            buffers,
            buffer_index: 1,
            is_running: true,
            map_keys: Mapper::default(),
            key_parse: KeyParser::default(),
            current_pane_size: Size::default(),
        })
    }

    fn get_current_buffer(&self) -> &buffer::Buffer {
        &self.buffers[self.buffer_index]
    }

    fn get_current_buffer_mut(&mut self) -> &mut buffer::Buffer {
        &mut self.buffers[self.buffer_index]
    }

    fn get_cursor(&self) -> Position {
        let cursor = self.get_current_buffer().cursor.pos;
        Position::new(cursor.x + 1, cursor.y + 0)
    }

    fn run(&mut self) -> Result<()> {
        let mut terminal: Tui = Terminal::new(CrosstermBackend::new(stdout()))?;
        while self.is_running {
            terminal.draw(|f| self.handle_render(f))?;
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Some(cmd) = self.handle_event(event::read()?)? {
                    cmd.call(self);
                }
            }
        }
        Ok(())
    }

    fn handle_render(&mut self, frame: &mut Frame) {
        let app = App {
            status_bar: StatusBar {
                mode: self.mode.clone(),
                cursor: self.buffers[self.buffer_index].cursor.pos.clone(),
                filename: self.buffers[self.buffer_index].name.clone(),
                file_saved: true,
                theme: Theme::default(),
            },
            line_number: LineNumbers {
                theme: Theme::default(),
            },
            panes: vec![Pane {
                buffer: self.buffers[self.buffer_index].clone(),
                theme: Theme::default(),
            }],
            current_pane: self.buffer_index.saturating_sub(1),
        };
        let rect = app.get_cursor_position(frame.size());
        let pos = rect.as_position();
        self.current_pane_size = rect.as_size();
        let cursor = self.get_cursor();
        frame.set_cursor(cursor.x + pos.x, cursor.y + pos.y);
        frame.render_widget(app, frame.size());
    }

    fn handle_event(&mut self, event: Event) -> Result<Option<command::CmdRc>> {
        let Event::Key(event) = event else {
            return Ok(None);
        };
        let key = key::Keys::from(event);
        self.key_parse.push(key);
        let cmd = self
            .map_keys
            .get_mapping(&self.mode, self.key_parse.get_keys());
        let is_possible_mapping = self
            .map_keys
            .is_possible_mapping(&self.mode, self.key_parse.get_keys());
        if !is_possible_mapping {
            let key_list = self.key_parse.get_keys();
            let _input = key_list
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
            let _message = match self.mode {
                Mode::Command => {}
                Mode::Insert => {}
                _ => {}
            };
            // return Some(message);
        }
        if cmd.is_some() {
            self.key_parse.clear();
        }
        Ok(cmd)
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode");
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}
