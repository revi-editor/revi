use crate::commands::{
    Backspace, ChangeMode, Command, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
    DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
    JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
    NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp, YankLine,
};
use crate::mode::Mode;
// use crate::revi_command::ReViCommand::{
//     self, Backspace, ChangeMode, CursorDown, CursorLeft, CursorRight, CursorUp, DeleteChar,
//     DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode, FirstCharInLine, Home,
//     JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord, MoveForwardByWord, NewLine,
//     NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp, YankLine,
// };

use revi_ui::Key;
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<Box<dyn Command>>>;

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
    pub fn get_mapping(&self, mode: Mode, event: &[Key]) -> Option<&Vec<Box<dyn Command>>> {
        self.get_map(mode).get(event)
    }
    #[must_use]
    pub fn insert_mapping(
        mut self,
        mode: Mode,
        keys: Vec<Key>,
        commands: Vec<Box<dyn Command>>,
    ) -> Self {
        self.get_map_mut(mode).insert(keys, commands);
        self
    }

    fn build_normal(self) -> Self {
        self.insert_mapping(
            Mode::Normal,
            vec![Key::Esc],
            vec![ChangeMode::new(Mode::Normal)],
        )
        .insert_mapping(Mode::Normal, vec![Key::LS, Key::Ctrl], vec![Save::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::UZ, Key::UZ],
            vec![Save::new(), Quit::new()],
        )
        .insert_mapping(Mode::Normal, vec![Key::UZ, Key::UQ], vec![Quit::new()])
        .insert_mapping(Mode::Normal, vec![Key::LJ], vec![CursorDown::new()])
        .insert_mapping(Mode::Normal, vec![Key::Down], vec![CursorDown::new()])
        .insert_mapping(Mode::Normal, vec![Key::LK], vec![CursorUp::new()])
        .insert_mapping(Mode::Normal, vec![Key::Up], vec![CursorUp::new()])
        .insert_mapping(Mode::Normal, vec![Key::LH], vec![CursorLeft::new()])
        .insert_mapping(Mode::Normal, vec![Key::Left], vec![CursorLeft::new()])
        .insert_mapping(Mode::Normal, vec![Key::LL], vec![CursorRight::new()])
        .insert_mapping(Mode::Normal, vec![Key::Right], vec![CursorRight::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::Colon],
            vec![EnterCommandMode::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LI],
            vec![ChangeMode::new(Mode::Insert)],
        )
        .insert_mapping(Mode::Normal, vec![Key::LX], vec![DeleteChar::new()])
        .insert_mapping(Mode::Normal, vec![Key::Delete], vec![DeleteChar::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::LD, Key::LD],
            vec![DeleteLine::new(), CursorUp::new()],
        )
        .insert_mapping(Mode::Normal, vec![Key::Home], vec![Home::new()])
        .insert_mapping(Mode::Normal, vec![Key::End], vec![End::new()])
        .insert_mapping(Mode::Normal, vec![Key::N0], vec![Home::new()])
        .insert_mapping(Mode::Normal, vec![Key::Char('$')], vec![End::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::UA],
            vec![
                End::new(),
                ChangeMode::new(Mode::Insert),
                CursorRight::new(),
            ],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LY, Key::Ctrl],
            vec![ScrollUp::new(), CursorDown::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LE, Key::Ctrl],
            vec![ScrollDown::new(), CursorUp::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LU, Key::Ctrl],
            vec![ScrollUp::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LD, Key::Ctrl],
            vec![ScrollDown::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LO],
            vec![
                End::new(),
                ChangeMode::new(Mode::Insert),
                CursorRight::new(),
                NewLine::new(),
            ],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::UO],
            vec![
                Home::new(),
                NewLine::new(),
                ChangeMode::new(Mode::Insert),
                CursorUp::new(),
            ],
        )
        .insert_mapping(Mode::Normal, vec![Key::Caret], vec![FirstCharInLine::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::UI],
            vec![FirstCharInLine::new(), ChangeMode::new(Mode::Insert)],
        )
        .insert_mapping(Mode::Normal, vec![Key::LW], vec![MoveForwardByWord::new()])
        .insert_mapping(Mode::Normal, vec![Key::LB], vec![MoveBackwardByWord::new()])
        .insert_mapping(
            Mode::Normal,
            vec![Key::LG, Key::LG],
            vec![JumpToFirstLineBuffer::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::UG],
            vec![JumpToLastLineBuffer::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::LW, Key::Ctrl, Key::LW, Key::Ctrl],
            vec![NextWindow::new()],
        )
        .insert_mapping(
            Mode::Normal,
            vec![Key::Enter],
            vec![ExcuteCommandLine::new(), ExitCommandMode::new()],
        )
        .insert_mapping(Mode::Normal, vec![Key::LY, Key::LY], vec![YankLine::new()])
        .insert_mapping(Mode::Normal, vec![Key::LP], vec![Paste::new()])
        .insert_mapping(Mode::Normal, vec![Key::UP], vec![PasteBack::new()])
    }

    fn build_insert(self) -> Self {
        self.insert_mapping(
            Mode::Insert,
            vec![Key::Esc],
            vec![ChangeMode::new(Mode::Normal)],
        )
        .insert_mapping(Mode::Insert, vec![Key::Backspace], vec![Backspace::new()])
        .insert_mapping(
            Mode::Insert,
            vec![Key::Enter],
            vec![
                NewLine::new(),
                ExcuteCommandLine::new(),
                ExitCommandMode::new(),
            ],
        )
        .insert_mapping(Mode::Insert, vec![Key::Home], vec![Home::new()])
        .insert_mapping(Mode::Insert, vec![Key::End], vec![End::new()])
        .insert_mapping(Mode::Insert, vec![Key::Down], vec![CursorDown::new()])
        .insert_mapping(Mode::Insert, vec![Key::Up], vec![CursorUp::new()])
        .insert_mapping(Mode::Insert, vec![Key::Left], vec![CursorLeft::new()])
        .insert_mapping(Mode::Insert, vec![Key::Right], vec![CursorRight::new()])
    }

    fn build_command(self) -> Self {
        self.insert_mapping(Mode::Command, vec![Key::Esc], vec![ExitCommandMode::new()])
            .insert_mapping(
                Mode::Command,
                vec![Key::Enter],
                vec![ExcuteCommandLine::new(), ExitCommandMode::new()],
            )
    }
}
