use crate::revi::ReVi;
use mlua::prelude::*;

impl LuaUserData for ReVi {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("cursor_pos", |_, revi, ()| Ok(revi.cursor_position_u16()));
        methods.add_method_mut("cursor", |_, revi, (x, y): (u16, u16)| {
            revi.set_cursor_position(x, y);
            Ok(())
        });
    }
}
