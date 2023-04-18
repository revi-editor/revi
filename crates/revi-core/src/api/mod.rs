mod revi;
mod window;
mod buffer;
use rhai::{Engine, Scope, EvalAltResult};

use crate::revi::ReVi;
use std::rc::Rc;
use std::cell::RefCell;

use self::{revi::ReViRhaiApi, window::WindowRhaiApi, buffer::BufferRhaiApi};

pub fn init_api<'a>(revi: Rc<RefCell<ReVi>>) -> Result<(Engine, Scope<'a>), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.build_type::<ReViRhaiApi>();
    engine.build_type::<WindowRhaiApi>();
    engine.build_type::<BufferRhaiApi>();
    let mut scope = Scope::new();
    scope.push("revi", ReViRhaiApi(revi));
    Ok((engine, scope))
}
