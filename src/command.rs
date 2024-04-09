use super::{Editor, Mode};

use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub trait Command: fmt::Debug {
    fn call(&self, editor: &mut Editor);
    fn equal(&self, other: &dyn Command) -> bool;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! command {
    ($([doc:  $doc:expr])? $name:ident$(($($ty:ty $(,)?)*))?, $caller:expr) => {
        $(#[doc=$doc])?
        #[derive(Debug, PartialEq)]
        pub struct $name $(($(pub $ty, )*))?;
        impl Command for $name {
            fn call(&self, editor:&mut Editor) {
                $caller(&self, editor);
            }
            fn equal(&self, other: &dyn Command) -> bool {
                other.as_any().downcast_ref::<Self>().map_or(false, |i| self==i)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl From<$name> for CmdRc {
            fn from(value: $name) -> Self {
                Self(Rc::new(value))
            }
        }
    };
}

#[derive(Clone)]
pub struct CmdRc(Rc<dyn Command>);

impl CmdRc {
    pub fn call(&self, editor: &mut Editor) {
        self.0.call(editor);
    }
}

impl fmt::Debug for CmdRc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialEq for CmdRc {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&*other.0)
    }
}

command!(ChangeMode(Mode), |s: &ChangeMode, editor: &mut Editor| {
    editor.mode = s.0.clone();
});

command!(Quit, |_: &Quit, editor: &mut Editor| {
    editor.is_running = false;
});
