use crate::commands::Command::{
    self, Backspace, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
    NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp, YankLine,
};
use crate::key_parser::string_to_key;
use crate::mode::Mode;
use revi_ui::Key;
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<Command>>;

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
    pub fn get(&self, mode: Mode, event: &[Key]) -> Option<&Vec<Command>> {
        self.get_map(mode).get(event)
    }
    #[must_use]
    pub fn with(mut self, mode: Mode, keys: &str, commands: Vec<Command>) -> Self {
        self.get_map_mut(mode).insert(string_to_key(keys), commands);
        self
    }

    pub fn insert(&mut self, mode: Mode, keys: &str, commands: Vec<Command>) {
        self.get_map_mut(mode).insert(string_to_key(keys), commands);
    }

    fn build_normal(self) -> Self {
        self.with(Mode::Normal, "<esc>", vec![ChangeMode(Mode::Normal)])
            .with(Mode::Normal, "<C-s>", vec![Save])
            .with(Mode::Normal, "ZZ", vec![Save, Quit])
            .with(Mode::Normal, "ZQ", vec![Quit])
            .with(Mode::Normal, "j", vec![CursorDown])
            .with(Mode::Normal, "<down>", vec![CursorDown])
            .with(Mode::Normal, "k", vec![CursorUp])
            .with(Mode::Normal, "<up>", vec![CursorUp])
            .with(Mode::Normal, "h", vec![CursorLeft])
            .with(Mode::Normal, "<left>", vec![CursorLeft])
            .with(Mode::Normal, "l", vec![CursorRight])
            .with(Mode::Normal, "<right>", vec![CursorRight])
            .with(Mode::Normal, ":", vec![EnterCommandMode])
            .with(Mode::Normal, "i", vec![ChangeMode(Mode::Insert)])
            .with(Mode::Normal, "x", vec![DeleteChar])
            .with(Mode::Normal, "<delete>", vec![DeleteChar])
            .with(Mode::Normal, "dd", vec![DeleteLine, CursorUp])
            .with(Mode::Normal, "<home>", vec![Home])
            .with(Mode::Normal, "<end>", vec![End])
            .with(Mode::Normal, "0", vec![Home])
            .with(Mode::Normal, "$", vec![End])
            .with(
                Mode::Normal,
                "A",
                vec![End, ChangeMode(Mode::Insert), CursorRight],
            )
            .with(Mode::Normal, "<C-y>", vec![ScrollUp, CursorDown])
            .with(Mode::Normal, "<C-e>", vec![ScrollDown, CursorUp])
            .with(Mode::Normal, "<C-u>", vec![ScrollUp])
            .with(Mode::Normal, "<C-d>", vec![ScrollDown])
            .with(
                Mode::Normal,
                "o",
                vec![End, ChangeMode(Mode::Insert), CursorRight, NewLine],
            )
            .with(
                Mode::Normal,
                "O",
                vec![Home, NewLine, ChangeMode(Mode::Insert), CursorUp],
            )
            .with(Mode::Normal, "^", vec![FirstCharInLine])
            .with(
                Mode::Normal,
                "I",
                vec![FirstCharInLine, ChangeMode(Mode::Insert)],
            )
            .with(Mode::Normal, "w", vec![MoveForwardByWord])
            .with(Mode::Normal, "b", vec![MoveBackwardByWord])
            .with(Mode::Normal, "gg", vec![JumpToFirstLineBuffer])
            .with(Mode::Normal, "G", vec![JumpToLastLineBuffer])
            .with(Mode::Normal, "<C-w><C-w>", vec![NextWindow])
            .with(
                Mode::Normal,
                "<enter>",
                vec![ExcuteCommandLine, ExitCommandMode],
            )
            .with(Mode::Normal, "yy", vec![YankLine])
            .with(Mode::Normal, "p", vec![Paste])
            .with(Mode::Normal, "P", vec![PasteBack])
    }

    fn build_insert(self) -> Self {
        self.with(Mode::Insert, "<esc>", vec![ChangeMode(Mode::Normal)])
            .with(Mode::Insert, "<backspace>", vec![Backspace])
            .with(
                Mode::Insert,
                "<enter>",
                vec![NewLine, ExcuteCommandLine, ExitCommandMode],
            )
            .with(Mode::Insert, "<home>", vec![Home])
            .with(Mode::Insert, "<end>", vec![End])
            .with(Mode::Insert, "<down>", vec![CursorDown])
            .with(Mode::Insert, "<up>", vec![CursorUp])
            .with(Mode::Insert, "<left>", vec![CursorLeft])
            .with(Mode::Insert, "<right>", vec![CursorRight])
    }

    fn build_command(self) -> Self {
        self.with(Mode::Command, "<esc>", vec![ExitCommandMode])
            .with(
                Mode::Command,
                "enter",
                vec![ExcuteCommandLine, ExitCommandMode],
            )
    }
}
