use crate::state::State;
use rhai::{CustomType, Engine, EvalAltResult, Scope, TypeBuilder, AST};
use std::{any::Any, cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct Rhai {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub ast: AST,
}

impl Rhai {
    pub fn eval_init(&mut self) {
        self.engine
            .eval_file_with_scope::<()>(&mut self.scope, "./userspace/init.rhai".into())
            .unwrap();
    }
}

pub fn init(state: State) -> Result<(Rc<RefCell<State>>, Rhai), Box<EvalAltResult>> {
    let state = Rc::new(RefCell::new(state));
    let context = ContextRhaiApi(state.clone());
    let mut rhai = Rhai::default();
    rhai.engine.build_type::<ContextRhaiApi>();
    rhai.scope.push("revi", context.clone());
    Ok((state, rhai))
}

#[derive(Debug, Clone)]
pub struct ContextRhaiApi(pub Rc<RefCell<State>>);
impl ContextRhaiApi {
    fn move_cursor_down(&mut self) {
        self.0.borrow_mut().cursor_down();
    }
    fn set_cursor_row(&mut self, row: i64) {
        self.0.borrow_mut().set_cursor_row(row as usize);
    }
}
impl CustomType for ContextRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Context")
            .with_fn("move_cursor_down", Self::move_cursor_down);
            // .with_fn("nmap", Self::nmap_from_str)
            // .with_fn("nmap", Self::nmap_function)
            .with_fn("set_cursor_row", Self::set_cursor_row)
            // .with_fn("set_cursor_col", Self::set_cursor_col)
            .with_fn("set_scroll_row", Self::set_scroll_row)
            // .with_fn("message", Self::message)
            // .with_fn("export_command", Self::export_command)
            // .with_get_set("mode", Self::get_mode, Self::set_mode);
    }
}
