use rhai::{CustomType, TypeBuilder};

use crate::window::Window;
use std::rc::Rc;
use std::cell::RefCell;
use super::BufferRhaiApi;

#[derive(Debug, Clone)]
pub struct WindowRhaiApi(pub Rc<RefCell<Window>>);

impl WindowRhaiApi {
    fn new(width: rhai::INT, height: rhai::INT, buffer: BufferRhaiApi) -> Self {
        let width = width as u16;
        let height = height as u16;
        let buffer = buffer.0;
        let inner = Rc::new(RefCell::new(Window::new(width, height, buffer)));
        Self(inner)
    }
}

impl CustomType for WindowRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Window")
            .with_fn("new_window", Self::new);
    }
}
