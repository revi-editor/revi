use crate::commands;
use crate::commands::{
    Backspace, BoxedCommand, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
    NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp, YankLine,
};
use crate::mode::Mode;
use revi_ui::{keys, Key};
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<BoxedCommand>>;

#[derive(Debug)]
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

    fn get_map(&self, mode: Mode) -> &KeyMap {
        match mode {
            Mode::Normal => &self.nmaps,
            Mode::Insert => &self.imaps,
            Mode::Command => &self.cmaps,
        }
    }

    fn get_map_mut(&mut self, mode: Mode) -> &mut KeyMap {
        match mode {
            Mode::Normal => &mut self.nmaps,
            Mode::Insert => &mut self.imaps,
            Mode::Command => &mut self.cmaps,
        }
    }

    #[must_use]
    pub fn get_mapping(&self, mode: Mode, event: &[Key]) -> Option<&Vec<BoxedCommand>> {
        self.get_map(mode).get(event)
    }
    #[must_use]
    pub fn with_mapping(mut self, mode: Mode, keys: Vec<Key>, commands: Vec<BoxedCommand>) -> Self {
        self.get_map_mut(mode).insert(keys, commands);
        self
    }

    fn build_normal(self) -> Self {
        self.with_mapping(
            Mode::Normal,
            vec![Key::Esc],
            commands![ChangeMode Mode::Normal],
        )
        .with_mapping(Mode::Normal, keys![LS, Ctrl], commands![Save])
        .with_mapping(Mode::Normal, keys![UZ, UZ], commands![Save, Quit])
        .with_mapping(Mode::Normal, keys![UZ, UQ], commands![Quit])
        .with_mapping(Mode::Normal, keys![LJ], commands![CursorDown])
        .with_mapping(Mode::Normal, keys![Down], commands![CursorDown])
        .with_mapping(Mode::Normal, keys![LK], commands![CursorUp])
        .with_mapping(Mode::Normal, keys![Up], commands![CursorUp])
        .with_mapping(Mode::Normal, keys![LH], commands![CursorLeft])
        .with_mapping(Mode::Normal, keys![Left], commands![CursorLeft])
        .with_mapping(Mode::Normal, keys![LL], commands![CursorRight])
        .with_mapping(Mode::Normal, keys![Right], commands![CursorRight])
        .with_mapping(Mode::Normal, keys![Colon], commands![EnterCommandMode])
        .with_mapping(Mode::Normal, keys![LI], commands![ChangeMode Mode::Insert])
        .with_mapping(Mode::Normal, keys![LX], commands![DeleteChar])
        .with_mapping(Mode::Normal, keys![Delete], commands![DeleteChar])
        .with_mapping(
            Mode::Normal,
            vec![Key::LD, Key::LD],
            commands![DeleteLine, CursorUp],
        )
        .with_mapping(Mode::Normal, keys![Home], commands![Home])
        .with_mapping(Mode::Normal, keys![End], commands![End])
        .with_mapping(Mode::Normal, keys![N0], commands![Home])
        .with_mapping(Mode::Normal, keys![Char('$')], commands![End])
        .with_mapping(
            Mode::Normal,
            keys![UA],
            commands![
                End,
                ChangeMode Mode::Insert,
                CursorRight
            ],
        )
        .with_mapping(
            Mode::Normal,
            keys![LY, Ctrl],
            commands![ScrollUp, CursorDown],
        )
        .with_mapping(
            Mode::Normal,
            keys![LE, Ctrl],
            commands![ScrollDown, CursorUp],
        )
        .with_mapping(Mode::Normal, keys![LU, Ctrl], commands![ScrollUp])
        .with_mapping(Mode::Normal, keys![LD, Ctrl], commands![ScrollDown])
        .with_mapping(
            Mode::Normal,
            keys![LO],
            commands![
                End,
                ChangeMode Mode::Insert,
                CursorRight,
                NewLine
            ],
        )
        .with_mapping(
            Mode::Normal,
            keys![UO],
            commands![
                Home,
                NewLine,
                ChangeMode Mode::Insert,
                CursorUp
            ],
        )
        .with_mapping(Mode::Normal, keys![Caret], commands![FirstCharInLine])
        .with_mapping(
            Mode::Normal,
            keys![UI],
            commands![
                FirstCharInLine,
                ChangeMode Mode::Insert
            ],
        )
        .with_mapping(Mode::Normal, keys![LW], commands![MoveForwardByWord])
        .with_mapping(Mode::Normal, keys![LB], commands![MoveBackwardByWord])
        .with_mapping(
            Mode::Normal,
            keys![LG, LG],
            commands![JumpToFirstLineBuffer],
        )
        .with_mapping(Mode::Normal, keys![UG], commands![JumpToLastLineBuffer])
        .with_mapping(
            Mode::Normal,
            keys![LW, Ctrl, LW, Ctrl],
            commands![NextWindow],
        )
        .with_mapping(
            Mode::Normal,
            keys![Enter],
            commands![ExcuteCommandLine, ExitCommandMode],
        )
        .with_mapping(Mode::Normal, keys![LY, LY], commands![YankLine])
        .with_mapping(Mode::Normal, keys![LP], commands![Paste])
        .with_mapping(Mode::Normal, keys![UP], commands![PasteBack])
    }

    fn build_insert(self) -> Self {
        self.with_mapping(Mode::Insert, keys![Esc], commands![ChangeMode Mode::Normal])
            .with_mapping(Mode::Insert, keys![Backspace], commands![Backspace])
            .with_mapping(
                Mode::Insert,
                keys![Enter],
                commands![NewLine, ExcuteCommandLine, ExitCommandMode],
            )
            .with_mapping(Mode::Insert, keys![Home], commands![Home])
            .with_mapping(Mode::Insert, keys![End], commands![End])
            .with_mapping(Mode::Insert, keys![Down], commands![CursorDown])
            .with_mapping(Mode::Insert, keys![Up], commands![CursorUp])
            .with_mapping(Mode::Insert, keys![Left], commands![CursorLeft])
            .with_mapping(Mode::Insert, keys![Right], commands![CursorRight])
    }

    fn build_command(self) -> Self {
        self.with_mapping(Mode::Command, keys![Esc], commands![ExitCommandMode])
            .with_mapping(
                Mode::Command,
                keys![Enter],
                commands![ExcuteCommandLine, ExitCommandMode],
            )
    }
}
