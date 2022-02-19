use crate::commands;
use crate::commands::{
    Backspace, BoxedCommand, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
    NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp, YankLine,
};
use crate::key_parser::string_to_key;
use crate::mode::Mode;
use revi_ui::Key;
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
    pub fn with_mapping(mut self, mode: Mode, keys: &str, commands: Vec<BoxedCommand>) -> Self {
        self.get_map_mut(mode).insert(string_to_key(keys), commands);
        self
    }

    fn build_normal(self) -> Self {
        self.with_mapping(Mode::Normal, "<esc>", commands![ChangeMode(Mode::Normal)])
            .with_mapping(Mode::Normal, "<C-s>", commands![Save])
            .with_mapping(Mode::Normal, "zz", commands![Save, Quit])
            .with_mapping(Mode::Normal, "zq", commands![Quit])
            .with_mapping(Mode::Normal, "j", commands![CursorDown])
            .with_mapping(Mode::Normal, "down", commands![CursorDown])
            .with_mapping(Mode::Normal, "k", commands![CursorUp])
            .with_mapping(Mode::Normal, "up", commands![CursorUp])
            .with_mapping(Mode::Normal, "h", commands![CursorLeft])
            .with_mapping(Mode::Normal, "left", commands![CursorLeft])
            .with_mapping(Mode::Normal, "l", commands![CursorRight])
            .with_mapping(Mode::Normal, "right", commands![CursorRight])
            .with_mapping(Mode::Normal, ":", commands![EnterCommandMode])
            .with_mapping(Mode::Normal, "i", commands![ChangeMode(Mode::Insert)])
            .with_mapping(Mode::Normal, "x", commands![DeleteChar])
            .with_mapping(Mode::Normal, "delete", commands![DeleteChar])
            .with_mapping(Mode::Normal, "dd", commands![DeleteLine, CursorUp])
            .with_mapping(Mode::Normal, "home", commands![Home])
            .with_mapping(Mode::Normal, "end", commands![End])
            .with_mapping(Mode::Normal, "0", commands![Home])
            .with_mapping(Mode::Normal, "$", commands![End])
            .with_mapping(
                Mode::Normal,
                "A",
                commands![End, ChangeMode(Mode::Insert), CursorRight],
            )
            .with_mapping(Mode::Normal, "<C-y>", commands![ScrollUp, CursorDown])
            .with_mapping(Mode::Normal, "<C-e>", commands![ScrollDown, CursorUp])
            .with_mapping(Mode::Normal, "<C-u>", commands![ScrollUp])
            .with_mapping(Mode::Normal, "<C-d>", commands![ScrollDown])
            .with_mapping(
                Mode::Normal,
                "o",
                commands![End, ChangeMode(Mode::Insert), CursorRight, NewLine],
            )
            .with_mapping(
                Mode::Normal,
                "O",
                commands![Home, NewLine, ChangeMode(Mode::Insert), CursorUp],
            )
            .with_mapping(Mode::Normal, "^", commands![FirstCharInLine])
            .with_mapping(
                Mode::Normal,
                "I",
                commands![FirstCharInLine, ChangeMode(Mode::Insert)],
            )
            .with_mapping(Mode::Normal, "w", commands![MoveForwardByWord])
            .with_mapping(Mode::Normal, "b", commands![MoveBackwardByWord])
            .with_mapping(Mode::Normal, "gg", commands![JumpToFirstLineBuffer])
            .with_mapping(Mode::Normal, "G", commands![JumpToLastLineBuffer])
            .with_mapping(Mode::Normal, "<C-w><C-w>", commands![NextWindow])
            .with_mapping(
                Mode::Normal,
                "<enter>",
                commands![ExcuteCommandLine, ExitCommandMode],
            )
            .with_mapping(Mode::Normal, "yy", commands![YankLine])
            .with_mapping(Mode::Normal, "p", commands![Paste])
            .with_mapping(Mode::Normal, "P", commands![PasteBack])
    }

    fn build_insert(self) -> Self {
        self.with_mapping(Mode::Insert, "<esc>", commands![ChangeMode(Mode::Normal)])
            .with_mapping(Mode::Insert, "<backspace>", commands![Backspace])
            .with_mapping(
                Mode::Insert,
                "<enter>",
                commands![NewLine, ExcuteCommandLine, ExitCommandMode],
            )
            .with_mapping(Mode::Insert, "<home>", commands![Home])
            .with_mapping(Mode::Insert, "<end>", commands![End])
            .with_mapping(Mode::Insert, "<down>", commands![CursorDown])
            .with_mapping(Mode::Insert, "<up>", commands![CursorUp])
            .with_mapping(Mode::Insert, "<left>", commands![CursorLeft])
            .with_mapping(Mode::Insert, "<right>", commands![CursorRight])
    }

    fn build_command(self) -> Self {
        self.with_mapping(Mode::Command, "<esc>", commands![ExitCommandMode])
            .with_mapping(
                Mode::Command,
                "enter",
                commands![ExcuteCommandLine, ExitCommandMode],
            )
    }
}
