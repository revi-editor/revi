use crate::commands::Command;
use crate::key_parser::string_to_key;
use crate::keymapper::Mapper;
use crate::mode::*;
use crate::revi::ReVi;
use mlua::prelude::*;
use mlua::{Result, UserData};
use std::convert::TryFrom;

impl LuaUserData for ReVi {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("cursor_pos", |_, revi, ()| Ok(revi.cursor_position_u16()));
        methods.add_method_mut("cursor", |_, revi, (x, y): (u16, u16)| {
            revi.set_cursor_position(x, y);
            Ok(())
        });
        methods.add_method_mut("print", |_, _revi, thing: String| {
            eprintln!("Rect: {}", thing);
            Ok(())
        });
    }
}

impl LuaUserData for Mapper {
    // -- mapper:nmap("g", {"yy", "p", "P"})
    // -- mapper:map("Insert", "<C-a>", {"DeleteChar", "YankLine", "Paste"})
    // -- mapper:map({mode="Insert", keys="<C-a>", command="DeleteChar"})
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "nmap",
            |_, mapper, (keys, commands_keys): (String, Vec<String>)| {
                let commands = commands_keys
                    .iter()
                    .filter_map(|c| {
                        mapper
                            .get(Mode::Normal, &string_to_key(c))
                            .map(Clone::clone)
                    })
                    .flatten()
                    .collect::<Vec<Command>>();
                mapper.insert(Mode::Normal, &keys, commands);
                Ok(())
            },
        );
        // methods.add_method_mut(
        //     "map",
        //     |_, _mapper, (mode, keys, command): (String, String, String)| {
        //         eprintln!("Mode: {:?}, Keys: {:?}, Command: {:?}", mode, keys, command);
        //         Ok(())
        //     },
        // )
        methods.add_method_mut(
            "map",
            |_, mapper, (mode, keys, commands): (Mode, String, Vec<Command>)| {
                eprintln!(
                    "Mode: {:?}, Keys: {:?}, Command: {:?}",
                    mode, keys, commands
                );
                mapper.insert(mode, keys.as_str(), commands);
                Ok(())
            },
        )
    }
}

impl UserData for Command {}
impl UserData for Mode {}

pub fn initialize_lua_api(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let mode = lua.create_function(|_, mode: String| match Mode::try_from(mode.as_str()) {
        Ok(v) => Ok(v),
        Err(_) => Err(mlua::Error::UserDataTypeMismatch),
    })?;
    globals.set("mode", mode)?;
    let command =
        lua.create_function(
            |_, command: String| match Command::try_from(command.as_str()) {
                Ok(v) => Ok(v),
                Err(_) => Err(mlua::Error::UserDataTypeMismatch),
            },
        )?;
    globals.set("command", command)?;
    Ok(())
}
