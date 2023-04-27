use crate::Context;
use crate::Mode;
use crate::commands::UserCommand;

use rhai::{CustomType, TypeBuilder, Dynamic, FuncArgs};

#[derive(Debug, Clone)]
pub struct ContextRhaiApi(pub Context);
impl ContextRhaiApi {
    fn get_mode(&mut self) -> rhai::ImmutableString {
        self.0.mode.borrow().to_string().into()
    }
    fn set_mode(&mut self, str_mode: rhai::ImmutableString) {
        let mode = match str_mode.to_lowercase().as_str() {
            "insert" => Mode::Insert,
            "command" => Mode::Command,
            "normal" | _ => Mode::Normal,
        };
        // *self.0.panes[self.0.focused_pane].borrow_mut().mode = mode;
        // BUG: This doesnt set the current window status bar to current mode
        *self.0.mode.borrow_mut() = mode;
    }

    fn nmap_from_str(
        &mut self,
        combo: rhai::ImmutableString,
        command: rhai::ImmutableString,
    ) {
        self.0.map_keys.borrow_mut().nmap_from_str(combo.as_str(), command.as_str());
    }

    fn nmap_function(
        &mut self,
        combo: rhai::ImmutableString,
        func: rhai::FnPtr,
    ) {
        let mut rhai_commands = self.0.rhai_commands.borrow_mut();
        let id = rhai_commands.len();
        rhai_commands.push(func);
        self.0.map_keys.borrow_mut().nmap(combo.as_str(), UserCommand(id));
    }

    fn set_cursor_row(&mut self, row: rhai::INT) {
        let id = self.0.focused_pane;
        let mut pane = self.0.panes[id].borrow_mut();
        pane.get_cursor_pos_mut().map(|c| {
            c.pos.y = row as u16;
            c
        });
    }

    fn set_cursor_col(&mut self, col: rhai::INT) {
        let id = self.0.focused_pane;
        let mut pane = self.0.panes[id].borrow_mut();
        pane.get_cursor_pos_mut().map(|c| {
            c.pos.x = col as u16;
            c
        });
    }
    fn set_scroll_row(&mut self, row: rhai::INT) {
        let id = self.0.focused_pane;
        let mut pane = self.0.panes[id].borrow_mut();
        pane.get_cursor_pos_mut().map(|c| {
            c.scroll.y = row as u16;
            c
        });
    }
}

impl CustomType for ContextRhaiApi {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("Context")
            .with_fn("nmap", Self::nmap_from_str)
            .with_fn("nmap", Self::nmap_function)
            .with_fn("set_cursor_row", Self::set_cursor_row)
            .with_fn("set_cursor_col", Self::set_cursor_col)
            .with_fn("set_scroll_row", Self::set_scroll_row)
            .with_get_set("mode", Self::get_mode, Self::set_mode);
    }
}