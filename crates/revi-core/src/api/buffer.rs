use rhai::{CustomType, TypeBuilder};

use crate::Buffer;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct BufferRhaiApi(pub Rc<RefCell<Buffer>>);
impl BufferRhaiApi {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Buffer::new())))
    }
    fn insert(&mut self, idx: rhai::INT, txt: rhai::ImmutableString) {
        self.0.borrow_mut().insert(idx as usize, &txt);
    }
}
impl CustomType for BufferRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Buffer")
            .with_fn("new_buffer", Self::new)
            .with_fn("insert", Self::insert)
            ;
    }
}
