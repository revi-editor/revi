use revi_ui::{
    tui::{layout::{Pos, Rect}, widget::BoxWidget},
    Keys,
};

pub trait Pane: std::fmt::Debug + CursorMovement + Scrollable {
    fn view(&self) -> BoxWidget;
    fn update(&mut self, keys: Keys);
}

#[derive(Debug, Default)]
pub struct Cursor {
    pub pos: Pos,
    pub max: Pos,
}

pub trait CursorMovement {
    fn get_cursor_pos(&self) -> Option<&Cursor> { None }
    fn get_cursor_pos_mut(&mut self) -> Option<&mut Cursor> { None }
    fn get_cursor_bounds(&self) -> Option<Rect> { None }
    fn get_line_above_bounds(&self) -> Option<Rect> { None }
    fn get_line_below_bounds(&self) -> Option<Rect> { None }
    fn move_cursor_up(&mut self) {
        let Some(bounds) = self.get_cursor_bounds() else {
            return;
        };
        let above = self.get_line_above_bounds();
        let Some(cursor) = self.get_cursor_pos_mut() else {
            return;
        };

        cursor.max.x = cursor.pos.x.max(cursor.max.x);
        if let Some(above) = above {
            cursor.pos.x = cursor.max.x.min(above.width);
        }
        cursor.pos.y = cursor.pos.y.saturating_sub(1).max(bounds.y);
    }
    fn move_cursor_down(&mut self) {
        let Some(bounds) = self.get_cursor_bounds() else {
            return;
        };
        let below = self.get_line_below_bounds();
        let Some(cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.max.x = cursor.pos.x.max(cursor.max.x);
        if let Some(below) = below {
            cursor.pos.x = cursor.max.x.min(below.width.min(cursor.max.x));
        }
        cursor.pos.y = cursor.pos.y.saturating_add(1).min(bounds.height);
    }
    fn move_cursor_left(&mut self) {
        let Some(bounds) = self.get_cursor_bounds() else {
            return;
        };
        let Some(cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        if cursor.pos.x > bounds.x {
            cursor.pos.x -= 1;
            cursor.max.x = cursor.pos.x;
        }
    }
    fn move_cursor_right(&mut self) {
        let Some(bounds) = self.get_cursor_bounds() else {
            return;
        };
        let Some(cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        if cursor.pos.x < bounds.width {
            cursor.pos.x += 1;
        }
        cursor.max.x = cursor.pos.x.max(cursor.max.x);
    }
}

pub trait Scrollable: CursorMovement {
    fn scroll_up(&mut self) {}
    fn scroll_down(&mut self) {}
    fn scroll_left(&mut self) {}
    fn scroll_right(&mut self) {}
}
