use super::{
    Buffer,
    Pane,
};

use revi_ui::tui::layout::Size;

use std::{
    rc::Rc,
    cell::RefCell,
};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub buffers: Vec<Rc<RefCell<Buffer>>>,
    pub panes: Vec<Rc<dyn Pane>>,
    pub focused_pane: usize,
    pub on_screan: Vec<usize>,
    pub count: usize,
    pub window_size: Size,
}
