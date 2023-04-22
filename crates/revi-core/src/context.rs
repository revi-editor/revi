use super::{Buffer, Pane};

use revi_ui::tui::layout::Size;

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct ContextBuilder {
    buffers: Vec<Rc<RefCell<Buffer>>>,
    panes: Vec<Rc<RefCell<dyn Pane>>>,
    focused_pane: usize,
    on_screen: Vec<usize>,
    count: usize,
    window_size: Size,
    show_command_bar: bool,
}

impl ContextBuilder {
    pub fn with_buffers(mut self, buffers: Vec<Rc<RefCell<Buffer>>>) -> Self {
        self.buffers = buffers;
        self
    }
    pub fn with_panes(mut self, panes: Vec<Rc<RefCell<dyn Pane>>>) -> Self {
        self.panes = panes;
        self
    }
    pub fn with_focused_pane(mut self, focused: usize) -> Self {
        self.focused_pane = focused;
        self
    }
    pub fn with_on_screen(mut self, on_screen: Vec<usize>) -> Self {
        self.on_screen = on_screen;
        self
    }
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
    pub fn with_window_size(mut self, size: Size) -> Self {
        self.window_size = size;
        self
    }
    pub fn with_show_command_bar(mut self, flag: bool) -> Self {
        self.show_command_bar = flag;
        self
    }
    pub fn build(self) -> Context {
        Context {
            buffers: self.buffers,
            panes: self.panes,
            focused_pane: self.focused_pane,
            on_screen: self.on_screen,
            count: self.count,
            window_size: self.window_size,
            show_command_bar: self.show_command_bar,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub buffers: Vec<Rc<RefCell<Buffer>>>,
    pub panes: Vec<Rc<RefCell<dyn Pane>>>,
    pub focused_pane: usize,
    pub on_screen: Vec<usize>,
    pub count: usize,
    window_size: Size,
    show_command_bar: bool,
}

impl Context {
    pub fn window_size(&self) -> Size {
        let height = self.window_size.height;
        let offset = self.show_command_bar as u16;
        let height = height.saturating_sub(offset);
        let width = self.window_size.width;
        dbg!(self.window_size, self.show_command_bar);
        dbg!(Size { width, height })
    }

    pub fn main_window_size(&self) -> Size {
        self.window_size
    }
}

impl Default for Context {
    fn default() -> Self {
        ContextBuilder::default()
            .with_show_command_bar(true)
            .build()
    }
}
