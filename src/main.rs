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
// mod message;
mod parse_keys;
// mod state;
mod trie;
use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

use map_keys::Mapper;
use parse_keys::KeyParser;
use ratatui::{
    layout::Position,
    prelude::*,
    style::Styled,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Editor::new()?.run()
}

type Tui = Terminal<CrosstermBackend<Stdout>>;

struct Editor {
    mode: Mode,
    buffers: Vec<buffer::Buffer>,
    buffer_index: usize,
    is_running: bool,
    map_keys: Mapper,
    key_parse: KeyParser,
}

impl Editor {
    fn new() -> Result<Self> {
        let args = cli::Cli::parse();
        let mut buffers = args
            .files
            .iter()
            .map(buffer::Buffer::from_path)
            .collect::<Vec<_>>();
        buffers.insert(0, buffer::Buffer::default());
        Ok(Self {
            mode: Mode::Normal,
            buffers,
            buffer_index: 0,
            is_running: true,
            map_keys: Mapper::default(),
            key_parse: KeyParser::default(),
        })
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
        };
        frame.set_cursor(10, 10);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Command,
    Normal,
}

#[derive(Debug, Clone)]
pub struct Theme {
    fg: Color,
    bg: Color,
    highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            fg: Color::White,
            bg: Color::Rgb(40, 40, 40),
            highlight: Color::Gray,
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    status_bar: StatusBar,
    line_number: LineNumbers,
    panes: Vec<Pane>,
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = self
            .panes
            .iter()
            .map(|_| Constraint::Min(1))
            .collect::<Vec<_>>();
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(area);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(3), Constraint::Min(1)].as_ref())
            .split(main_layout[0]);
        let text_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[1]);

        for (idx, pane) in self.panes.into_iter().enumerate() {
            pane.render(text_layout[idx], buf);
        }
        self.line_number.render(layout[0], buf);
        self.status_bar.render(main_layout[1], buf);
    }
}

#[derive(Debug, Clone)]
struct Pane {
    buffer: buffer::Buffer,
    theme: Theme,
}

impl Widget for Pane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_on_screen = self
            .buffer
            .on_screen(area.width, area.height)
            .into_iter()
            .map(Line::from)
            .collect::<Vec<Line>>();
        let block = Block::default().borders(Borders::LEFT);
        Paragraph::new(text_on_screen)
            .block(block)
            .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
            .render(area, buf);
    }
}

#[derive(Debug, Clone, Default)]
struct LineNumbers {
    theme: Theme,
}

impl Widget for LineNumbers {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = (0..area.height).fold(Vec::new(), |mut acc, num| {
            acc.push(Line {
                spans: vec![format!("{num}").into()],
                style: Style::default().bg(self.theme.bg).fg(self.theme.fg).dim(),
                alignment: Some(Alignment::Right),
            });
            acc
        });

        Paragraph::new(text)
            .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
            .render(area, buf);
    }
}

#[derive(Debug, Clone)]
struct StatusBar {
    mode: Mode,
    cursor: Position,
    filename: String,
    file_saved: bool,
    theme: Theme,
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let file_saved = if self.file_saved { "✔ " } else { "✘ " };
        let file_saved_style = if self.file_saved {
            // TODO: make colors configurable
            Style::default().green().on_dark_gray().bold()
        } else {
            Style::default().red().on_dark_gray().bold()
        };
        Line::from(vec![
            Span::styled(
                format!("{:?} ", self.mode),
                Style::default().bg(self.theme.bg).fg(self.theme.fg).bold(),
            ),
            Span::styled(
                format!("{} ", self.filename),
                Style::default().bg(self.theme.bg).fg(self.theme.fg).bold(),
            ),
            Span::styled(file_saved, file_saved_style),
        ])
        .render(area, buf);
        Line {
            spans: vec![Span::raw(format!("{}/{}", self.cursor.x, self.cursor.y))],
            style: Style::default()
                .bg(self.theme.highlight)
                .fg(self.theme.fg)
                .bold(),
            alignment: Some(Alignment::Right),
        }
        .render(area, buf);
    }
}
