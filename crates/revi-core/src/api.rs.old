use crate::{
    line_number::LineNumberKind,
    revi::{ReVi, Settings},
    window::Window,
    Buffer, Mapper,
};
use mlua::{prelude::*, MetaMethod};
use std::cell::RefCell;
use std::rc::Rc;
pub fn create_api(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let init_buffer = lua.create_function(|_, ()| Ok(Buffer::new()))?;
    globals.set("init_buffer", init_buffer)?;
    Ok(())
}

impl LuaUserData for Window {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Call,
            |_, (width, height, buffer): (u16, u16, Buffer)| {
                Ok(Self::new(width, height, Rc::new(RefCell::new(buffer))))
            },
        );
    }
}
impl LuaUserData for Buffer {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(Self::new()));
        methods.add_method_mut("insert", |_, buffer, (idx, c): (usize, String)| {
            buffer.insert(idx, &c);
            Ok(())
        });
    }
}

impl LuaUserData for Mapper {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("normal", |lua, mapper, keys: LuaTable| {
            let name: String = keys.get("name")?;
            let key_combo: String = keys.get("keys")?;
            let command: LuaFunction = keys.get("command")?;
            let globals = lua.globals();
            globals
                .set(name.as_str(), command)
                .expect("failed to set global function for commands");

            mapper.normal_insert(
                &key_combo,
                vec![crate::commands::LuaCommand(name.clone()).into()],
            );
            Ok(())
        });
    }
}

impl LuaUserData for LineNumberKind {}

impl LuaUserData for Settings {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("line_number_kind", |_, settings| {
            Ok(format!("{:?}", settings.line_number_kind))
        });
        fields.add_field_method_get("tab_width", |_, settings| Ok(settings.tab_width));
        fields.add_field_method_get("LineNumberKindNone", |_, _| {
            Ok(format!("{:?}", LineNumberKind::None))
        });
        fields.add_field_method_get("LineNumberKindBoth", |_, _| {
            Ok(format!("{:?}", LineNumberKind::Both))
        });
        fields.add_field_method_get("LineNumberKindAbsolute", |_, _| {
            Ok(format!("{:?}", LineNumberKind::AbsoluteNumber))
        });
        fields.add_field_method_get("LineNumberKindRelative", |_, _| {
            Ok(format!("{:?}", LineNumberKind::RelativeNumber))
        });
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_line_number", |_, settings, kind: String| {
            eprintln!("{:?}", kind);
            let line_number_kind = match kind.to_lowercase().as_str() {
                "none" => Ok(LineNumberKind::None),
                "both" => Ok(LineNumberKind::Both),
                "absolutenumber" | "absolute_number" => Ok(LineNumberKind::AbsoluteNumber),
                "relativenumber" | "relative_number" => Ok(LineNumberKind::RelativeNumber),
                _ => Err(mlua::Error::RuntimeError("not a line number option".into())),
            }?;
            settings.line_number_kind = line_number_kind;
            Ok(())
        });
    }
}

impl LuaUserData for ReVi {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("buffers", |_, revi| Ok(revi.buffers.clone()));
        fields.add_field_method_get("width", |_, revi| Ok(revi.width()));
        fields.add_field_method_get("height", |_, revi| Ok(revi.height()));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_buffer", |_, revi, buffer: Buffer| {
            let id = revi.buffers.len();
            revi.buffers.push(Rc::new(RefCell::new(buffer)));
            Ok(id)
        });
        methods.add_method_mut("add_window", |_, revi, window: Window| {
            revi.windows.push(window);
            Ok(())
        });
        methods.add_method_mut("create_window", |_, revi, keys: LuaTable| {
            let width: u16 = keys.get("width")?;
            let height: u16 = keys.get("height")?;
            let buffer: Buffer = keys.get("buffer")?; //.unwrap_or(Buffer::new());
            let buffer = Rc::new(RefCell::new(buffer));
            revi.buffers.push(buffer.clone());
            let id = revi.windows.len();
            revi.windows.push(Window::new(width, height, buffer));
            revi.queue.push(id);
            Ok(())
        });
        methods.add_method_mut("cursor_up", |_, revi, count: usize| {
            revi.focused_window_mut().move_cursor_up(count);
            Ok(())
        });
        methods.add_method_mut("cursor_down", |_, revi, count: usize| {
            revi.focused_window_mut().move_cursor_down(count);
            Ok(())
        });
        methods.add_method_mut("cursor_left", |_, revi, count: usize| {
            revi.focused_window_mut().move_cursor_left(count);
            Ok(())
        });
        methods.add_method_mut("cursor_right", |_, revi, count: usize| {
            revi.focused_window_mut().move_cursor_right(count);
            Ok(())
        });
    }
}
