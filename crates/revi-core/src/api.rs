use mlua::prelude::*;
use crate::{revi::{Settings, ReVi}, line_number::LineNumberKind};

impl LuaUserData for LineNumberKind { }

impl LuaUserData for Settings {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("line_number_kind", |_, settings| {
            Ok(format!("{:?}", settings.line_number_kind))
        });
        fields.add_field_method_get("tab_width", |_, settings| {
            Ok(settings.tab_width)
        });
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
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_line_number", |_, revi, _on: bool| {
            // TODO: Finish this thought
            revi.focused_window_mut().move_cursor_up(0);
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


