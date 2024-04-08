use revi_ui::{
    container::Container,
    layout::{Rect, Size, Stack},
    style::ContentStyle,
    text::Text,
    Color, Keys,
};

use crate::Mode;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum Message {
    CursorDown,
    CursorUp,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    InsertAtEnd,
    BackSpace,
    Delete,
    KeyPress(Keys),
    CheckForMapping,
    ModeCommandInsertStr(String),
    ModeInsertInsertStr(String),
    ChangeMode(Mode),
    ExecuteCommand,
    BufferList,
    EditFile(String),
    SwapBuffer(String),
    UserMessage(UserMessageBuilder),
    CloseCurrentPaneOnKeyPress,
    NextAvailableCommand,
    Resize(Size),
    Save(Option<String>),
    Quit,
}

#[derive(Debug, Clone, Default)]
pub struct UserMessageBuilder {
    pub message: String,
    pub footer: String,
    pub style: ContentStyle,
}

impl UserMessageBuilder {
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.style.foreground_color = Some(fg);
        self
    }

    pub fn _bg(mut self, bg: Color) -> Self {
        self.style.background_color = Some(bg);
        self
    }

    pub fn build(self) -> Message {
        Message::UserMessage(self)
    }

    pub fn build_container(&self, width: u16) -> Container {
        use revi_ui::widget::Widget;
        let stack = Stack::Vertically;
        let msg = Text::new(&self.message);
        let ft = Text::new(&self.footer).with_bg(Color::Grey);
        let height = msg.height() + ft.height();
        let rect = Rect::new(Size { width, height });
        Container::new(rect, stack).push(msg).push(ft)
    }
}
