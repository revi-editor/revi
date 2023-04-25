mod context;

use rhai::{Engine, Scope, EvalAltResult, AST};
use crate::Context;
use self::context::ContextRhaiApi;

#[derive(Debug)]
pub struct Rhai {
    engine: Engine,
    scope: Scope<'static>,
}

impl Rhai {
    pub fn run_ast<T>(&mut self, ast: &AST) -> Result<T, Box<EvalAltResult>>
        where T: Clone + 'static
    {
        self.engine.eval_ast_with_scope::<T>(&mut self.scope, ast)
    }

    pub fn run_file<T>(&mut self, filename: &str) -> Result<T, Box<EvalAltResult>>
        where T: Clone + 'static
    {
        self.engine.eval_file_with_scope::<T>(&mut self.scope, filename.into())
    }
}

pub fn init<'a>(ctx: Context) -> Result<Rhai, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.build_type::<ContextRhaiApi>();
    let mut scope = Scope::new();
    scope.push("revi", ContextRhaiApi(ctx));
    Ok(Rhai {
        engine, scope
    })
}
