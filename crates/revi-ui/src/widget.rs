use crate::{layout::Rect, text::Text};
use std::io::Stdout;
pub trait Widget: std::fmt::Debug + dyn_clone::DynClone {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn draw(&self, stdout: &mut Stdout, bounds: Rect);
    fn debug_name(&self) -> String {
        "DEFAULT".to_string()
    }
}

dyn_clone::clone_trait_object!(Widget);

#[derive(Debug, Clone)]
pub struct BoxWidget {
    widget: Box<dyn Widget>,
}

impl BoxWidget {
    pub fn new<W>(widget: W) -> Self
    where
        W: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
        }
    }

    #[must_use]
    pub fn x(&self) -> u16 {
        self.widget.x()
    }

    #[must_use]
    pub fn y(&self) -> u16 {
        self.widget.y()
    }

    #[must_use]
    pub fn width(&self) -> u16 {
        self.widget.width()
    }

    #[must_use]
    pub fn height(&self) -> u16 {
        self.widget.height()
    }

    pub fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
        self.widget.draw(stdout, bounds);
    }

    #[must_use]
    pub fn _debug_name(&self) -> String {
        self.widget.debug_name()
    }
}

impl From<Text> for BoxWidget {
    fn from(value: Text) -> Self {
        Self::new(value)
    }
}
