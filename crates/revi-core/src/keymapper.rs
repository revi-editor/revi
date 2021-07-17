use crate::mode::Mode;
use crate::revi_command::ReViCommand::{
    self, Backspace, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
    NextWindow, Quit, Save, ScrollDown, ScrollUp,
};

use revi_ui::Key;
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<ReViCommand>>;

pub struct Mapper {
    nmaps: KeyMap,
    imaps: KeyMap,
    cmaps: KeyMap,
}

impl Default for Mapper {
    fn default() -> Self {
        Self::new().build_normal().build_insert().build_command()
    }
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
        self.get_map(mode).get(event)
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
        self.insert_mapping(&Normal, vec![Key::Esc], vec![ChangeMode(Normal)])
            .insert_mapping(&Normal, vec![Key::UZ, Key::UZ], vec![Save, Quit])
            .insert_mapping(&Normal, vec![Key::UZ, Key::UQ], vec![Quit])
            .insert_mapping(&Normal, vec![Key::LJ], vec![CursorDown])
            .insert_mapping(&Normal, vec![Key::Down], vec![CursorDown])
            .insert_mapping(&Normal, vec![Key::LK], vec![CursorUp])
            .insert_mapping(&Normal, vec![Key::Up], vec![CursorUp])
            .insert_mapping(&Normal, vec![Key::LH], vec![CursorLeft])
            .insert_mapping(&Normal, vec![Key::Left], vec![CursorLeft])
            .insert_mapping(&Normal, vec![Key::LL], vec![CursorRight])
            .insert_mapping(&Normal, vec![Key::Right], vec![CursorRight])
            .insert_mapping(&Normal, vec![Key::Colon], vec![EnterCommandMode])
            .insert_mapping(&Normal, vec![Key::LI], vec![ChangeMode(Insert)])
            .insert_mapping(&Normal, vec![Key::LX], vec![DeleteChar])
            .insert_mapping(&Normal, vec![Key::Delete], vec![DeleteChar])
            .insert_mapping(&Normal, vec![Key::LD, Key::LD], vec![DeleteLine, CursorUp])
            .insert_mapping(&Normal, vec![Key::Home], vec![Home])
            .insert_mapping(&Normal, vec![Key::End], vec![End])
            .insert_mapping(&Normal, vec![Key::N0], vec![Home])
            .insert_mapping(&Normal, vec![Key::Char('$')], vec![End])
            .insert_mapping(
                &Normal,
                vec![Key::UA],
                vec![End, ChangeMode(Insert), CursorRight],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LY, Key::Ctrl],
                vec![ScrollUp, CursorDown],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LE, Key::Ctrl],
                vec![ScrollDown, CursorUp],
            )
            .insert_mapping(&Normal, vec![Key::LU, Key::Ctrl], vec![ScrollUp])
            .insert_mapping(&Normal, vec![Key::LD, Key::Ctrl], vec![ScrollDown])
            .insert_mapping(
                &Normal,
                vec![Key::LO],
                vec![End, ChangeMode(Insert), CursorRight, NewLine],
            )
            .insert_mapping(
                &Normal,
                vec![Key::UO],
                vec![Home, NewLine, ChangeMode(Insert), CursorUp],
            )
            .insert_mapping(&Normal, vec![Key::Caret], vec![FirstCharInLine])
            .insert_mapping(
                &Normal,
                vec![Key::UI],
                vec![FirstCharInLine, ChangeMode(Insert)],
            )
            .insert_mapping(&Normal, vec![Key::LW], vec![MoveForwardByWord])
            .insert_mapping(&Normal, vec![Key::LB], vec![MoveBackwardByWord])
            .insert_mapping(&Normal, vec![Key::LG, Key::LG], vec![JumpToFirstLineBuffer])
            .insert_mapping(&Normal, vec![Key::UG], vec![JumpToLastLineBuffer])
            .insert_mapping(
                &Normal,
                vec![Key::LW, Key::Ctrl, Key::LW, Key::Ctrl],
                vec![NextWindow],
            )
            .insert_mapping(
                &Normal,
                vec![Key::Enter],
                vec![ExcuteCommandLine, ExitCommandMode],
            )
    }

    fn build_insert(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Insert, vec![Key::Esc], vec![ChangeMode(Normal)])
            .insert_mapping(&Insert, vec![Key::Backspace], vec![Backspace])
            .insert_mapping(
                &Insert,
                vec![Key::Enter],
                vec![NewLine, ExcuteCommandLine, ExitCommandMode],
            )
            .insert_mapping(&Insert, vec![Key::Home], vec![Home])
            .insert_mapping(&Insert, vec![Key::End], vec![End])
            .insert_mapping(&Insert, vec![Key::Down], vec![CursorDown])
            .insert_mapping(&Insert, vec![Key::Up], vec![CursorUp])
            .insert_mapping(&Insert, vec![Key::Left], vec![CursorLeft])
            .insert_mapping(&Insert, vec![Key::Right], vec![CursorRight])
    }

    fn build_command(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Command, vec![Key::Esc], vec![ExitCommandMode])
            .insert_mapping(
                &Command,
                vec![Key::Enter],
                vec![ExcuteCommandLine, ExitCommandMode],
            )
    }
}
