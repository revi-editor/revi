use crate::layout::{Alignment, Rect};
use crate::widget::Widget;
use crossterm::style::{Attribute, Color, ContentStyle, ResetColor, SetAttribute, SetStyle};
use crossterm::{cursor, queue, style};
use std::io::Stdout;
#[derive(Debug, Default, Clone)]
pub struct Text {
    content: String,
    align: Alignment,
    style: ContentStyle,
    width: u16,
    height: u16,
    comment: Option<String>,
}

impl Text {
    pub fn new(content: &str) -> Self {
        let content = content.replace('\n', " ");
        let width = content.chars().max().unwrap_or_default() as u16;
        let height = content.lines().count() as u16;
        Self {
            content,
            align: Alignment::Left,
            style: ContentStyle::new(),
            width,
            height,
            comment: None,
        }
    }

    pub fn char_len(&self) -> usize {
        self.content.len().max(self.width as usize)
    }

    pub fn with_alignment(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn with_fg(mut self, fg: Color) -> Self {
        self.style.foreground_color = Some(fg);
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.style.background_color = Some(bg);
        self
    }

    pub fn with_atter(mut self, atter: impl Into<style::Attributes>) -> Self {
        self.style.attributes = atter.into();
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn max_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn max_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }
}

impl Widget for Text {
    #[inline]
    fn x(&self) -> u16 {
        0
    }

    #[inline]
    fn y(&self) -> u16 {
        0
    }

    #[inline]
    fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    fn height(&self) -> u16 {
        self.height
    }

    fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
        queue!(stdout, SetStyle(self.style)).expect("failed to set style");
        let width = bounds.width() as usize;
        let x = bounds.x() + self.x();
        let y = bounds.y() + self.y();
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            style::Print(format_line(&self.content, width, &self.align)),
        )
        .expect("Failed to queue Text");
        queue!(stdout, ResetColor, SetAttribute(Attribute::Reset))
            .expect("failed to queue reset color and  attribute");
    }
    fn debug_name(&self) -> String {
        self.comment.clone().unwrap_or_default()
    }
}

fn format_line(line: &str, width: usize, align: &Alignment) -> String {
    match align {
        Alignment::Left => format!("{:<width$}", line),
        Alignment::Right => format!("{:>width$}", line),
        Alignment::Center => format!("{:^width$}", line),
    }
    .chars()
    .take(width)
    .collect()
}

#[test]
fn test_format_line() {
    let width = 20;
    let default = format_line("hello", width, &Alignment::Left);
    assert_eq!(default, "hello               ".to_string());
    let default = format_line("hello", width, &Alignment::Right);
    assert_eq!(default, "               hello".to_string());
    let default = format_line("hello", 3, &Alignment::Left);
    assert_eq!(default, "hel".to_string());
}
