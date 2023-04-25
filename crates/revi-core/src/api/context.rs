use crate::Context;
use crate::Mode;

use rhai::{CustomType, TypeBuilder};


#[derive(Debug, Clone)]
pub struct ContextRhaiApi(pub(crate) Context);
impl ContextRhaiApi {
    fn get_mode(&mut self) -> rhai::ImmutableString {
        self.0.mode.borrow().to_string().into()
    }
    fn set_mode(&mut self, str_mode: rhai::ImmutableString) {
        let mode = match str_mode.as_str() {
            "insert" => Mode::Insert,
            _ => Mode::Normal,
        };
        // *self.0.panes[self.0.focused_pane].borrow_mut().mode = mode;
        // BUG: This doesnt set the current window status bar to current mode
        *self.0.mode.borrow_mut() = mode;
    }
}

impl CustomType for ContextRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Context")
            .with_get_set("mode", Self::get_mode, Self::set_mode)
            ;
    }
}
