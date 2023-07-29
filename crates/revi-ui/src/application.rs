use crate::{layout::Pos, subscription::Subscription, widget::BoxWidget};
use crossterm::{cursor::SetCursorStyle, Result};
pub trait App: Sized {
    type Settings: std::fmt::Debug;
    type Message: std::fmt::Debug;
    fn new(settings: Self::Settings) -> Self;
    fn update(&mut self, message: Self::Message) -> Option<Self::Message>;
    fn view(&self) -> BoxWidget;
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
    fn cursor_pos(&self) -> Option<Pos> {
        None
    }
    fn cursor_shape(&self) -> Option<SetCursorStyle> {
        None
    }
    fn quit(&self) -> bool {
        true
    }
    fn run(&mut self) -> Result<()> {
        crate::runtime::run(self)
    }
}
