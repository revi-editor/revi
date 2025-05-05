use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct ReviTerminal {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    buffer: String,
}

impl ReviTerminal {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            buffer: String::from("Waiting for content..."),
        })
    }

    pub fn set_buffer(&mut self, content: String) {
        self.buffer = content;
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("Revi").borders(Borders::ALL);
            let paragraph = Paragraph::new::<&str>(self.buffer.as_ref())
                .block(block)
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, size);
        })?;
        Ok(())
    }

    pub fn poll_input(&self, timeout: Duration) -> io::Result<Option<CEvent>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}

impl Drop for ReviTerminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}
