use crate::{key::Key, mode::Mode, revi_command::ReViCommand};
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<ReViCommand>>;

pub struct Mapper {
    nmaps: KeyMap,
    imaps: KeyMap,
    cmaps: KeyMap,
}

impl Mapper {
    fn new() -> Self {
        Self {
            nmaps: KeyMap::new(),
            imaps: KeyMap::new(),
            cmaps: KeyMap::new(),
        }
    }

    fn get_map(&self, mode: &Mode) -> &KeyMap {
        use Mode::*;
        match mode {
            Normal => &self.nmaps,
            Insert => &self.imaps,
            Command => &self.cmaps,
        }
    }

    fn get_map_mut(&mut self, mode: &Mode) -> &mut KeyMap {
        use Mode::*;
        match mode {
            Normal => &mut self.nmaps,
            Insert => &mut self.imaps,
            Command => &mut self.cmaps,
        }
    }

    pub fn get_mapping(&self, mode: &Mode, event: &[Key]) -> Option<&Vec<ReViCommand>> {
        Some(self.get_map(mode).get(event)?)
    }

    pub fn insert_mapping(
        mut self,
        mode: &Mode,
        keys: Vec<Key>,
        commands: Vec<ReViCommand>,
    ) -> Self {
        self.get_map_mut(mode).insert(keys, commands);
        self
    }

    fn build_normal(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Normal, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
            .insert_mapping(
                &Normal,
                vec![Key::UZ, Key::Shift, Key::UZ, Key::Shift],
                vec![ReViCommand::Quit, ReViCommand::Save],
            ) // Save Command would be called too.
            .insert_mapping(
                &Normal,
                vec![Key::UZ, Key::Shift, Key::UQ, Key::Shift],
                vec![ReViCommand::Quit],
            ) // Save Command would be called too.
            .insert_mapping(&Normal, vec![Key::LJ], vec![ReViCommand::CursorDown])
            .insert_mapping(&Normal, vec![Key::LK], vec![ReViCommand::CursorUp])
            .insert_mapping(&Normal, vec![Key::LH], vec![ReViCommand::CursorLeft])
            .insert_mapping(&Normal, vec![Key::LL], vec![ReViCommand::CursorRight])
            .insert_mapping(&Normal, vec![Key::Colon], vec![ReViCommand::Mode(Command)])
            .insert_mapping(&Normal, vec![Key::LI], vec![ReViCommand::Mode(Insert)])
            .insert_mapping(&Normal, vec![Key::LX], vec![ReViCommand::DeleteChar])
            .insert_mapping(&Normal, vec![Key::Delete], vec![ReViCommand::DeleteChar])
    }

    fn build_insert(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Insert, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
            .insert_mapping(&Insert, vec![Key::Backspace], vec![ReViCommand::Backspace])
            .insert_mapping(&Insert, vec![Key::Enter], vec![ReViCommand::NewLine])
    }

    fn build_command(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Command, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
    }
}

pub fn key_builder() -> Mapper {
    Mapper::new().build_normal().build_insert().build_command()
}
