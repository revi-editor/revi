use std::{cell::RefCell, rc::Rc, any::Any};
use rhai::{CustomType, TypeBuilder, Engine, EvalAltResult, Scope, AST};
use crate::state::State;

#[derive(Debug, Default)]
pub struct Rhai {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub ast: AST,
}

// pub fn init(mut state: &mut State) -> Result<Rhai, Box<EvalAltResult>> {
//     let context = ContextRhaiApi(state));
//     let mut rhai = Rhai::default();
//     rhai.engine.build_type::<ContextRhaiApi>();
//     rhai.scope.push("revi", context.clone());
//     // state = *context.as_any().downcast_ref::<State>().unwrap();
//     // rhai.compile("./userspace/init.rhai");
//     // rhai.run_ast_with_scope()?;
//     // rhai.engine.build_type::<
//     Ok((state, rhai))
// }


#[derive(Debug, Clone)]
pub struct ContextRhaiApi(pub Rc<RefCell<State>>);
impl ContextRhaiApi {
    fn as_any(&self) -> &dyn Any {
        &self.0
    }

    fn move_cursor_down(&mut self) {
        self.0.borrow_mut().cursor_down();
    }
}
impl CustomType for ContextRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Context")
            .with_fn("move_cursor_down", Self::move_cursor_down);

            // .with_fn("nmap", Self::nmap_from_str)
            // .with_fn("nmap", Self::nmap_function)
            // .with_fn("set_cursor_row", Self::set_cursor_row)
            // .with_fn("set_cursor_col", Self::set_cursor_col)
            // .with_fn("set_scroll_row", Self::set_scroll_row)
            // .with_fn("message", Self::message)
            // .with_fn("export_command", Self::export_command)
            // .with_get_set("mode", Self::get_mode, Self::set_mode);
    }
}
