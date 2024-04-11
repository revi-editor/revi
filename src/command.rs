use super::{Editor, Mode};

use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub trait Command: fmt::Debug {
    fn call(&self, editor: &mut Editor);
    fn equal(&self, other: &dyn Command) -> bool;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! command {
    ($([doc:  $doc:expr])? $name:ident$(($($ty:ty $(,)?)*))?, $caller:expr) => {
        $(#[doc=$doc])?
        #[derive(Debug, PartialEq)]
        pub struct $name $(($(pub $ty, )*))?;
        impl Command for $name {
            fn call(&self, editor:&mut Editor) {
                $caller(&self, editor);
            }
            fn equal(&self, other: &dyn Command) -> bool {
                other.as_any().downcast_ref::<Self>().map_or(false, |i| self==i)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl From<$name> for CmdRc {
            fn from(value: $name) -> Self {
                Self(Rc::new(value))
            }
        }
    };
}

#[derive(Clone)]
pub struct CmdRc(Rc<dyn Command>);

impl CmdRc {
    pub fn call(&self, editor: &mut Editor) {
        self.0.call(editor);
    }
}

impl fmt::Debug for CmdRc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialEq for CmdRc {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&*other.0)
    }
}

command!(ChangeMode(Mode), |s: &ChangeMode, editor: &mut Editor| {
    editor.mode = s.0.clone();
});

command!(Quit, |_: &Quit, editor: &mut Editor| {
    editor.is_running = false;
});

command!(CursorUp, |_: &CursorUp, editor: &mut Editor| {
    let buf = editor.get_current_buffer_mut();
    if buf.cursor_up() {
        return;
    }
    buf.scroll_up();
});

command!(CursorDown, |_: &CursorDown, editor: &mut Editor| {
    let height: usize = editor.get_current_pane_size().height.into();
    let buf = editor.get_current_buffer_mut();
    if buf.cursor_down(height) {
        return;
    }
    buf.scroll_down(height);
});

command!(CursorRight, |_: &CursorRight, editor: &mut Editor| {
    let width: usize = editor.get_current_pane_size().width.into();
    let buf = editor.get_current_buffer_mut();
    if buf.cursor_right(width) {
        return;
    }
    buf.scroll_right(width);
});

command!(CursorLeft, |_: &CursorLeft, editor: &mut Editor| {
    let buf = editor.get_current_buffer_mut();
    if buf.cursor_left() {
        return;
    }
    buf.scroll_left();
});

command!(CursorHome, |_: &CursorHome, editor: &mut Editor| {
    let buf = editor.get_current_buffer_mut();
    buf.cursor_home();
});

command!(CursorEnd, |_: &CursorEnd, editor: &mut Editor| {
    let buf = editor.get_current_buffer_mut();
    buf.cursor_end();
});

command!(
    CursorTopOfBuffer,
    |_: &CursorTopOfBuffer, editor: &mut Editor| {
        let buf = editor.get_current_buffer_mut();
        // let last_line = buf.len_lines();
        let cursor = buf.get_cursor_mut();
        cursor.pos.y = 0;
        cursor.scroll.y = 0;
    }
);

command!(
    CursorToBottomOfBuffer,
    |_: &CursorToBottomOfBuffer, editor: &mut Editor| {
        let height = editor.get_current_pane_size().height;
        let buf = editor.get_current_buffer_mut();
        let last_line = buf.len_lines() as u16;
        let cursor = buf.get_cursor_mut();
        cursor.pos.y = height;
        cursor.scroll.y = last_line.saturating_sub(height).min(last_line);
        buf.align_cursor();
    }
);

command!(
    InsertChar(String),
    |text: &InsertChar, editor: &mut Editor| {
        let buf = editor.get_current_buffer_mut();
        buf.insert(&text.0);
    }
);
