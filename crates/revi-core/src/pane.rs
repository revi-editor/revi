use revi_ui::{tui::widget::BoxWidget, Keys};

pub trait Pane: std::fmt::Debug {
    fn view(&self) -> BoxWidget;
    fn update(&mut self, keys: Keys);
}

pub trait Cursor {
    fn cursor_pos(&self) -> Option<(u16, u16)> {None}
    fn move_cursor_up(&mut self) {}
    fn move_cursor_down(&mut self) {}
    fn move_cursor_left(&mut self) {}
    fn move_cursor_right(&mut self) {}
}

pub trait Scrollable {
    fn scroll_up(&mut self) {}
    fn scroll_down(&mut self) {}
    fn scroll_left(&mut self) {}
    fn scroll_right(&mut self) {}
}
