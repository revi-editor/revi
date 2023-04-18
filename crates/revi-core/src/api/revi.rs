use rhai::{CustomType, TypeBuilder};
use super::window::WindowRhaiApi;

use super::ReVi;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct ReViRhaiApi(pub Rc<RefCell<ReVi>>);

impl ReViRhaiApi {
    fn width(&mut self) -> rhai::INT {
        self.0.borrow().width().into()
    }
    fn height(&mut self) -> rhai::INT {
        self.0.borrow().height().into()
    }
    fn cursor_up(&mut self, count: usize) {
        self.0.borrow_mut().focused_window_mut().move_cursor_up(count)
    }
    fn cursor_down(&mut self, count: usize) {
        self.0.borrow_mut().focused_window_mut().move_cursor_down(count)
    }
    fn cursor_left(&mut self, count: usize) {
        self.0.borrow_mut().focused_window_mut().move_cursor_left(count)
    }
    fn cursor_right(&mut self, count: usize) {
        self.0.borrow_mut().focused_window_mut().move_cursor_right(count)
    }
    fn create_window(&mut self, window: WindowRhaiApi) {
        let mut revi = self.0.borrow_mut();
        let window = window.0.borrow().clone();
        let buffer = window.buffer.clone();
        let id = revi.windows.len();
        revi.buffers.push(buffer);
        revi.windows.push(window);
        revi.queue.push(id);
    }
    fn buffer_count(&mut self) -> rhai::INT {
        (self.0.borrow().buffers.len() as i64).into()
    }
}

impl CustomType for ReViRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ReVi")
            .with_get("width", Self::width)
            .with_get("height", Self::height)
            .with_fn("cursor_up", Self::cursor_up)
            .with_fn("cursor_down", Self::cursor_down)
            .with_fn("cursor_left", Self::cursor_left)
            .with_fn("cursor_right", Self::cursor_right)
            .with_fn("create_window", Self::create_window)
            .with_fn("buffer_count", Self::buffer_count)
            ;
    }
}


