use crate::commands;
use crate::commands::BoxedCommand;
use crate::commands::{
    CursorDown,
    CursorLeft,
    CursorRight,
    CursorUp,
    ScrollDown,
    ScrollUp,
    ChangeMode,
    InsertChar,
    ExecuteCommandLine,
};

use crate::mode::Mode;
use revi_ui::{string_to_keys, Keys};

#[derive(Debug)]
enum MapNode {
    Map(Keys, KeyMap),
    Middle(Keys, KeyMap, Vec<BoxedCommand>),
    End(Keys, Vec<BoxedCommand>),
}

#[derive(Debug)]
struct KeyMap {
    mappings: Vec<MapNode>,
}

impl KeyMap {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    fn new_mapping(node: MapNode) -> Self {
        Self {
            mappings: vec![node],
        }
    }

    fn is_command(&self, keys: &[Keys]) -> bool {
        if keys.is_empty() {
            return false;
        }
        self.mappings
            .iter()
            .any(|node| match node {
                MapNode::Map(key, keymap) if key == &keys[0] => keymap.is_command(&keys[1..]),
                MapNode::Middle(key, keymap, _) if key == &keys[0] => keymap.is_command(&keys[1..]),
                MapNode::End(key, _) if key == &keys[0] => true,
                _ => false,
            })
    }

    // fn _is_last_branch_command(&self, _: &[Keys]) -> bool {
    //     todo!()
    // }

    fn get(&self, key: &[Keys]) -> Option<&Vec<BoxedCommand>> {
        if key.is_empty() {
            return None;
        }
        for node in self.mappings.iter() {
            match node {
                MapNode::Map(k, map) if k == &key[0] => return map.get(&key[1..]),
                MapNode::Middle(k, _, cmd) if k == &key[0] && key.len() == 1 => return Some(cmd),
                MapNode::Middle(k, map, _) if k == &key[0] => return map.get(&key[1..]),
                MapNode::End(k, command) if k == &key[0] => return Some(command),
                _ => {}
            };
        }
        None
    }

    fn insert(&mut self, keys: &[Keys], command: Vec<BoxedCommand>) {
        let Some(key) = keys.first() else {
            return;
        };
        for node in self.mappings.iter_mut() {
            match node {
                MapNode::Map(k, map) if k == key => return map.insert(&keys[1..], command),
                MapNode::Middle(k, map, _) if k == key => {
                    return map.insert(&keys[1..], command)
                }
                MapNode::End(k, cmd) if k == key && keys.len() == 1 => {
                    *cmd = command;
                    return;
                }
                MapNode::End(k, cmd) if k == key && keys.len() > 1 => {
                    let mut map = KeyMap::new();
                    map.insert(&keys[1..], command);
                    *node = MapNode::Middle(*k, map, cmd.clone());
                    return;
                }
                _ => {},
            }
        }
        self.insert_new(keys, command);
    }

    // Blindly inserts new mapping
    fn insert_new(&mut self, keys: &[Keys], command: Vec<BoxedCommand>) {
        let mut key_iter = keys.iter().rev();
        let Some(key) = key_iter.next() else {
            return;
        };
        let start_node = MapNode::End(*key, command);
        let mapnode = key_iter.fold(start_node, |acc, key| {
            MapNode::Map(*key, KeyMap::new_mapping(acc))
        });
        self.mappings.push(mapnode);
    }
}

//type KeyMap = HashMap<Vec<Key>, Vec<BoxedCommand>>;

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

    fn get_map(&self, mode: &Mode) -> &KeyMap {
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
    pub fn is_mapping(&self, mode: &Mode, keys: &[Keys]) -> bool {
        self.get_map(mode).is_command(keys)
    }

    #[must_use]
    pub fn get_mapping(&self, mode: &Mode, keys: &[Keys]) -> Option<&Vec<BoxedCommand>> {
        self.get_map(mode).get(keys)
    }

    #[must_use]
    pub fn with_mapping(mut self, mode: Mode, keys: &str, commands: Vec<BoxedCommand>) -> Self {
        self.get_map_mut(mode)
            .insert(&string_to_keys(keys), commands);
        self
    }

    fn build_normal(self) -> Self {
        self
            .with_mapping(Mode::Normal, "<esc>", commands![ChangeMode(Mode::Normal)])
            // .with_mapping(Mode::Normal, "<C-s>", commands![Save])
            // .with_mapping(Mode::Normal, "zz", commands![Save, Quit])
            // .with_mapping(Mode::Normal, "zq", commands![Quit])
            .with_mapping(Mode::Normal, "j", commands![CursorDown])
            .with_mapping(Mode::Normal, "<down>", commands![CursorDown])
            .with_mapping(Mode::Normal, "k", commands![CursorUp])
            .with_mapping(Mode::Normal, "up", commands![CursorUp])
            .with_mapping(Mode::Normal, "h", commands![CursorLeft])
            .with_mapping(Mode::Normal, "<left>", commands![CursorLeft])
            .with_mapping(Mode::Normal, "l", commands![CursorRight])
            .with_mapping(Mode::Normal, "<right>", commands![CursorRight])
            .with_mapping(Mode::Normal, ":", commands![ChangeMode(Mode::Command)])
            .with_mapping(Mode::Normal, "i", commands![ChangeMode(Mode::Insert)])
            //     .with_mapping(Mode::Normal, "x", commands![DeleteChar])
            //     .with_mapping(Mode::Normal, "delete", commands![DeleteChar])
            //     .with_mapping(Mode::Normal, "dd", commands![DeleteLine, CursorUp])
            //     .with_mapping(Mode::Normal, "home", commands![Home])
            //     .with_mapping(Mode::Normal, "end", commands![End])
            //     .with_mapping(Mode::Normal, "0", commands![Home])
            //     .with_mapping(Mode::Normal, "$", commands![End])
            //     .with_mapping(
            //         Mode::Normal,
            //         "A",
            //         commands![End, ChangeMode(Mode::Insert), CursorRight],
            //     )
            .with_mapping(Mode::Normal, "<C-y>", commands![ScrollUp, CursorDown])
            .with_mapping(Mode::Normal, "<C-e>", commands![ScrollDown, CursorUp])
            .with_mapping(Mode::Normal, "<C-u>", commands![ScrollUp])
            .with_mapping(Mode::Normal, "<C-d>", commands![ScrollDown])
        //     .with_mapping(
        //         Mode::Normal,
        //         "o",
        //         commands![End, ChangeMode(Mode::Insert), CursorRight, NewLine],
        //     )
        //     .with_mapping(
        //         Mode::Normal,
        //         "O",
        //         commands![Home, NewLine, ChangeMode(Mode::Insert), CursorUp],
        //     )
        //     .with_mapping(Mode::Normal, "^", commands![FirstCharInLine])
        //     .with_mapping(
        //         Mode::Normal,
        //         "I",
        //         commands![FirstCharInLine, ChangeMode(Mode::Insert)],
        //     )
        //     .with_mapping(Mode::Normal, "w", commands![MoveForwardByWord])
        //     .with_mapping(Mode::Normal, "b", commands![MoveBackwardByWord])
        //     .with_mapping(Mode::Normal, "gg", commands![JumpToFirstLineBuffer])
        //     .with_mapping(Mode::Normal, "G", commands![JumpToLastLineBuffer])
        //     .with_mapping(Mode::Normal, "<C-w><C-w>", commands![NextWindow])
        //     .with_mapping(
        //         Mode::Normal,
        //         "<enter>",
        //         commands![ExecuteCommandLine, ExitCommandMode],
        //     )
        //     .with_mapping(Mode::Normal, "yy", commands![YankLine])
        //     .with_mapping(Mode::Normal, "p", commands![Paste])
        //     .with_mapping(Mode::Normal, "P", commands![PasteBack])
        //     .with_mapping(Mode::Normal, "u", commands![Undo])
        //     .with_mapping(Mode::Normal, "<space>a", commands![CursorRight])
    }

    fn build_insert(self) -> Self {
        self.with_mapping(Mode::Insert, "<esc>", commands![ChangeMode(Mode::Normal)])
        //     .with_mapping(Mode::Insert, "<backspace>", commands![Backspace])
        //     .with_mapping(
        //         Mode::Insert,
        //         "<enter>",
        //         commands![NewLine, ExecuteCommandLine, ExitCommandMode],
        //     )
        //     .with_mapping(Mode::Insert, "<home>", commands![Home])
        //     .with_mapping(Mode::Insert, "<end>", commands![End])
        //     .with_mapping(Mode::Insert, "<down>", commands![CursorDown])
        //     .with_mapping(Mode::Insert, "<up>", commands![CursorUp])
        //     .with_mapping(Mode::Insert, "<left>", commands![CursorLeft])
        //     .with_mapping(Mode::Insert, "<right>", commands![CursorRight])
        //     .with_mapping(Mode::Insert, "<tab>", commands![InsertTab])
    }

    fn build_command(self) -> Self {
        self.with_mapping(Mode::Command, "<esc>", commands![ChangeMode(Mode::Normal)])
            .with_mapping(
                Mode::Command,
                "enter",
                commands![ExecuteCommandLine],
            )
    }
}

#[test]
fn test_normal_keymapper() {
    let km = Mapper::default();
    assert_eq!(
        km.get_mapping(&Mode::Normal, &string_to_keys("k"))
            .unwrap(),
        &commands!(CursorUp)
    );
    assert_eq!(
        km.get_mapping(&Mode::Normal, &string_to_keys("j"))
            .unwrap(),
        &commands!(CursorDown)
    );
}

#[test]
fn test_command_insert_char() {
    let km = Mapper::default();
    assert!(
        !km.is_mapping(&Mode::Command, &string_to_keys("<esc>"))
    );
}
