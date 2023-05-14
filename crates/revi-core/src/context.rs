use crate::{api::Rhai, Settings};
use rhai::FnPtr;

use super::{Buffer, CommandBar, Event, Mapper, Mode, Pane};

use revi_ui::tui::layout::Size;

use std::{cell::RefCell, rc::Rc};
type Panes = Vec<Rc<RefCell<dyn Pane>>>;
type Buffers = Vec<Rc<RefCell<Buffer>>>;

#[derive(Debug, Default)]
pub struct ContextBuilder {
    buffers: Vec<Rc<RefCell<Buffer>>>,
    panes: Panes,
    command_bar: CommandBar,
    mode: Mode,
    focused_pane: usize,
    on_screen: Vec<usize>,
    window_size: Size,
    show_command_bar: bool,
    settings: Settings,
}

impl ContextBuilder {
    pub fn with_buffers(mut self, buffers: Vec<Rc<RefCell<Buffer>>>) -> Self {
        self.buffers = buffers;
        self
    }
    pub fn with_panes(mut self, panes: Panes) -> Self {
        self.panes = panes;
        self
    }

    pub fn with_command_bar(mut self, cb: CommandBar) -> Self {
        self.command_bar = cb;
        self
    }

    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
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
    pub fn with_window_size(mut self, size: Size) -> Self {
        self.window_size = size;
        self
    }
    pub fn with_show_command_bar(mut self, flag: bool) -> Self {
        self.show_command_bar = flag;
        self
    }
    pub fn with_settings(mut self, settings: Settings) -> Self {
        self.settings = settings;
        self
    }
    pub fn build(self) -> Context {
        let ctx = Context {
            buffers: Rc::new(RefCell::new(self.buffers)),
            panes: Rc::new(RefCell::new(self.panes)),
            command_bar: Rc::new(RefCell::new(self.command_bar)),
            map_keys: Rc::new(RefCell::new(Mapper::default())),
            mode: Rc::new(RefCell::new(self.mode)),
            rhai_commands: Rc::new(RefCell::new(Vec::new())),
            rhai: Rc::new(RefCell::new(Rhai::default())),
            focused_pane: Rc::new(RefCell::new(self.focused_pane)),
            on_screen: self.on_screen,
            is_running: Rc::new(RefCell::new(true)),
            event: Rc::new(RefCell::new(Event::None)),
            settings: Rc::new(RefCell::new(self.settings)),
            window_size: self.window_size,
            show_command_bar: self.show_command_bar,
        };

        crate::api::init(ctx.clone()).expect("failed to init scripting engine");
        ctx
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub buffers: Rc<RefCell<Buffers>>,
    pub panes: Rc<RefCell<Panes>>,
    pub command_bar: Rc<RefCell<dyn Pane>>,
    pub map_keys: Rc<RefCell<Mapper>>,
    pub mode: Rc<RefCell<Mode>>,
    pub rhai_commands: Rc<RefCell<Vec<FnPtr>>>,
    pub rhai: Rc<RefCell<Rhai>>,
    pub focused_pane: Rc<RefCell<usize>>,
    pub on_screen: Vec<usize>,
    pub is_running: Rc<RefCell<bool>>,
    pub event: Rc<RefCell<Event>>,
    pub settings: Rc<RefCell<Settings>>,
    window_size: Size,
    show_command_bar: bool,
}

impl Context {
    pub fn focused_pane(&self) -> Rc<RefCell<dyn Pane>> {
        let id = *self.focused_pane.borrow();
        self.panes.borrow()[id].clone()
    }

    pub fn window_size(&self) -> Size {
        let height = self.window_size.height;
        let offset = self.show_command_bar as u16;
        let height = height.saturating_sub(offset);
        let width = self.window_size.width;
        Size { width, height }
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
