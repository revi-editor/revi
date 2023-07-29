mod context;

pub use self::context::ContextRhaiApi;
use crate::Context;
use rhai::{Engine, EvalAltResult, Scope, AST};

#[derive(Debug, Default)]
pub struct Rhai {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub ast: AST,
}

impl Rhai {
    pub fn run_ast<T>(&mut self, ast: &AST) -> Result<T, Box<EvalAltResult>>
    where
        T: Clone + 'static,
    {
        self.engine.eval_ast_with_scope::<T>(&mut self.scope, ast)
    }

    pub fn compile(&mut self, filename: &str) {
        if let Ok(ast) = self
            .engine
            .compile_file_with_scope(&self.scope, filename.into())
        {
            self.ast = ast;
        }
    }

    pub fn run_ast_with_scope(&mut self) -> Result<(), Box<EvalAltResult>> {
        self.engine.run_ast_with_scope(&mut self.scope, &self.ast)?;
        Ok(())
    }
}

pub fn init(/* ctx: Context */) -> Result<(), Box<EvalAltResult>> {
    // let c = ctx.clone();
    // let mut rhai = c.rhai.borrow_mut();
    // rhai.engine.build_type::<ContextRhaiApi>();
    // rhai.scope.push("revi", ContextRhaiApi(ctx));
    // rhai.compile("./userspace/init.rhai");
    // rhai.run_ast_with_scope()?;
    Ok(())
}
