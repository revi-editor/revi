use std::{cell::RefCell, rc::Rc};
extern crate revi_core;
use revi_core::{
    panes::{Cursor, Pane, Window},
    Buffer, Pos, Size,
};

fn build_pane_with(src: &str) -> Rc<RefCell<dyn Pane>> {
    let buf = Buffer::new_str("test file", src);
    let buf = Rc::new(RefCell::new(buf));
    let pos = Pos::default();
    let size = Size::new(10, 10);
    let win = Window::new(pos, size, buf);
    Rc::new(RefCell::new(win))
}

#[test]
fn window_move_cursor_right() {
    let pane = build_pane_with("this is a txt file");
    pane.borrow_mut().move_cursor_right();
    let pane = pane.borrow();
    let left = pane.get_cursor_pos().unwrap().clone();
    let mut right = Cursor::default();
    right.pos.x += 1;
    right.max.x += 1;
    assert_eq!(left, right);
}
