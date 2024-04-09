use super::{
    command::{ChangeMode, CmdRc, Quit},
    Mode,
};
use crate::key::{string_to_keys, Keys};

#[derive(Debug)]
enum MapNode {
    Map(Keys, KeyMap),
    Middle(Keys, KeyMap, CmdRc),
    End(Keys, CmdRc),
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

    fn is_possible_command(&self, keys: &[Keys]) -> bool {
        if keys.is_empty() {
            return false;
        }
        self.mappings.iter().any(|node| match node {
            MapNode::Map(key, _) if key == &keys[0] && keys.len() == 1 => true,
            MapNode::Map(key, keymap) if key == &keys[0] => keymap.is_command(&keys[1..]),
            MapNode::Middle(key, _, _) if key == &keys[0] && keys.len() == 1 => true,
            MapNode::Middle(key, keymap, _) if key == &keys[0] => keymap.is_command(&keys[1..]),
            MapNode::End(key, _) if key == &keys[0] => true,
            _ => false,
        })
    }

    fn is_command(&self, keys: &[Keys]) -> bool {
        if keys.is_empty() {
            return false;
        }
        self.mappings.iter().any(|node| match node {
            MapNode::Map(key, keymap) if key == &keys[0] => keymap.is_command(&keys[1..]),
            MapNode::Middle(key, _, _) if key == &keys[0] && keys.len() == 1 => true,
            MapNode::Middle(key, keymap, _) if key == &keys[0] => keymap.is_command(&keys[1..]),
            MapNode::End(key, _) if key == &keys[0] => true,
            _ => false,
        })
    }

    // fn _is_last_branch_command(&self, _: &[Keys]) -> bool {
    //     todo!()
    // }

    fn get(&self, key: &[Keys]) -> Option<CmdRc> {
        if key.is_empty() {
            return None;
        }
        for node in self.mappings.iter() {
            match node {
                MapNode::Map(k, map) if k == &key[0] => return map.get(&key[1..]),
                MapNode::Middle(k, _, cmd) if k == &key[0] && key.len() == 1 => {
                    return Some(cmd.clone())
                }
                MapNode::Middle(k, map, _) if k == &key[0] => return map.get(&key[1..]),
                MapNode::End(k, command) if k == &key[0] => return Some(command.clone()),
                _ => {}
            };
        }
        None
    }

    fn insert(&mut self, keys: &[Keys], command: impl Into<CmdRc>) {
        let Some(key) = keys.first() else {
            return;
        };
        for node in self.mappings.iter_mut() {
            match node {
                MapNode::Map(k, map) if k == key => return map.insert(&keys[1..], command),
                MapNode::Middle(k, map, _) if k == key => return map.insert(&keys[1..], command),
                MapNode::End(k, cmd) if k == key && keys.len() == 1 => {
                    *cmd = command.into();
                    return;
                }
                MapNode::End(k, cmd) if k == key && keys.len() > 1 => {
                    let mut map = KeyMap::new();
                    map.insert(&keys[1..], command);
                    *node = MapNode::Middle(*k, map, cmd.clone());
                    return;
                }
                _ => {}
            }
        }
        self.insert_new(keys, command);
    }

    // Blindly inserts new mapping
    fn insert_new(&mut self, keys: &[Keys], command: impl Into<CmdRc>) {
        let mut key_iter = keys.iter().rev();
        let Some(key) = key_iter.next() else {
            return;
        };
        let start_node = MapNode::End(*key, command.into());
        let mapnode = key_iter.fold(start_node, |acc, key| {
            MapNode::Map(*key, KeyMap::new_mapping(acc))
        });
        self.mappings.push(mapnode);
    }
}

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
    pub fn _is_mapping(&self, mode: &Mode, keys: &[Keys]) -> bool {
        self.get_map(mode).is_command(keys)
    }

    #[must_use]
    pub fn is_possible_mapping(&self, mode: &Mode, keys: &[Keys]) -> bool {
        self.get_map(mode).is_possible_command(keys)
    }

    #[must_use]
    pub fn get_mapping(&self, mode: &Mode, keys: &[Keys]) -> Option<CmdRc> {
        self.get_map(mode).get(keys)
    }

    #[must_use]
    pub fn with_mapping(mut self, mode: Mode, keys: &str, message: impl Into<CmdRc>) -> Self {
        self.get_map_mut(mode)
            .insert(&string_to_keys(keys), message);
        self
    }

    pub fn _nmap(&mut self, keys: &str, message: impl Into<CmdRc>) {
        self.nmaps.insert(&string_to_keys(keys), message);
    }

    // pub fn nmap_from_str(&mut self, keys: &str, command: &str) {
    //     let keys = string_to_keys(keys);
    //     let command = string_to_keys(command);
    //     let mut cmds = Vec::new();
    //     let mut combo = Vec::new();
    //     for key in command.iter() {
    //         match self.get_mapping(&Mode::Normal, &[*key]) {
    //             Some(cmd) => cmds.push(cmd),
    //             None => match self.get_mapping(&Mode::Normal, &combo) {
    //                 Some(cmd) => {
    //                     cmds.push(cmd);
    //                     combo.clear();
    //                 }
    //                 _ => combo.push(*key),
    //             },
    //         }
    //     }
    //     if cmds.is_empty() {
    //         return;
    //     }
    //     self.nmaps.insert(&keys, ExeCommandList(cmds).into());
    // }

    fn build_normal(self) -> Self {
        self.with_mapping(Mode::Normal, "ZQ", Quit)
            // .with_mapping(Mode::Normal, "j", CursorDown)
            // .with_mapping(Mode::Normal, "<down>", CursorDown)
            // .with_mapping(Mode::Normal, "k", CursorUp)
            // .with_mapping(Mode::Normal, "up", CursorUp)
            // .with_mapping(Mode::Normal, "h", CursorLeft)
            // .with_mapping(Mode::Normal, "<left>", CursorLeft)
            // .with_mapping(Mode::Normal, "l", CursorRight)
            // .with_mapping(Mode::Normal, "<right>", CursorRight)
            .with_mapping(Mode::Normal, ":", ChangeMode(Mode::Command))
            .with_mapping(Mode::Normal, "i", ChangeMode(Mode::Insert))
        // .with_mapping(Mode::Normal, "x", Delete)
        // .with_mapping(Mode::Normal, "<delete>", Delete)
        // .with_mapping(Mode::Normal, "dd", DeleteLine)
        // .with_mapping(Mode::Normal, "<home>", CursorHome)
        // .with_mapping(Mode::Normal, "0", CursorHome)
        // .with_mapping(Mode::Normal, "gg", CursorTopOfBuffer)
        // .with_mapping(Mode::Normal, "G", CursorToBottomOfBuffer)
        // .with_mapping(Mode::Normal, "<end>", CursorEnd)
        // .with_mapping(Mode::Normal, "$", CursorEnd)
        // .with_mapping(Mode::Normal, "A", InsertAtEnd)
        // .with_mapping(Mode::Normal, "<C-y>", ScrollUp, CursorDown)
        // .with_mapping(Mode::Normal, "<C-e>", ScrollDown, CursorUp)
        // .with_mapping(Mode::Normal, "<C-u>", ScrollUp)
        // .with_mapping(Mode::Normal, "<C-d>", ScrollDown)
        //     .with_mapping(
        //         Mode::Normal,
        //         "o",
        //         End, ChangeMode(Mode::Insert), CursorRight, NewLine,
        //     )
        //     .with_mapping(
        //         Mode::Normal,
        //         "O",
        //         CursorHome, NewLine, ChangeMode(Mode::Insert), CursorUp,
        //     )
        //     .with_mapping(Mode::Normal, "^", FirstCharInLine)
        //     .with_mapping(
        //         Mode::Normal,
        //         "I",
        //         FirstCharInLine, ChangeMode(Mode::Insert),
        //     )
        //     .with_mapping(Mode::Normal, "w", MoveForwardByWord)
        //     .with_mapping(Mode::Normal, "b", MoveBackwardByWord)
        //     .with_mapping(Mode::Normal, "<C-w><C-w>", NextWindow)
        //     .with_mapping(
        //         Mode::Normal,
        //         "<enter>",
        //         ExecuteCommandLine, ExitCommandMode,
        //     )
        //     .with_mapping(Mode::Normal, "yy", YankLine)
        //     .with_mapping(Mode::Normal, "p", Paste)
        //     .with_mapping(Mode::Normal, "P", PasteBack)
        //     .with_mapping(Mode::Normal, "u", Undo)
        //     .with_mapping(Mode::Normal, "<space>a", CursorRight)
    }

    fn build_insert(self) -> Self {
        self.with_mapping(Mode::Insert, "<esc>", ChangeMode(Mode::Normal))
        // .with_mapping(Mode::Insert, "<backspace>", Message::BackSpace)
        // .with_mapping(Mode::Insert, "<up>", Message::CursorUp)
        // .with_mapping(Mode::Insert, "<down>", Message::CursorDown)
        // .with_mapping(Mode::Insert, "<left>", Message::CursorLeft)
        // .with_mapping(Mode::Insert, "<right>", Message::CursorRight)
        // .with_mapping(Mode::Insert, "<home>", Message::CursorHome)
        // .with_mapping(Mode::Insert, "<end>", Message::CursorEnd)
        //     .with_mapping(Mode::Insert, "<tab>", InsertTab)
    }

    fn build_command(self) -> Self {
        self.with_mapping(Mode::Command, "<esc>", ChangeMode(Mode::Normal))
        // .with_mapping(Mode::Command, "<enter>", Message::ExecuteCommand)
        // .with_mapping(Mode::Command, "<backspace>", Message::BackSpace)
        // .with_mapping(Mode::Command, "<tab>", Message::NextAvailableCommand)
        // .with_mapping(Mode::Command, "<c-h>", CursorLeft)
        // .with_mapping(Mode::Command, "<c-l>", CursorRight)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_normal_keymapper() {
//         let km = Mapper::default();
//         assert_eq!(
//             km.get_mapping(&Mode::Normal, &string_to_keys("k")).unwrap(),
//             CursorUp.into()
//         );
//         assert_eq!(
//             km.get_mapping(&Mode::Normal, &string_to_keys("j")).unwrap(),
//             CursorDown.into()
//         );
//     }
//
//     #[test]
//     fn test_command_insert_char() {
//         let km = Mapper::default();
//         assert!(km.is_mapping(&Mode::Command, &string_to_keys("<esc>")));
//     }
//
//     #[test]
//     fn test_multi_key_bindings() {
//         let km = Mapper::default();
//         let keys = string_to_keys("gg");
//         let left = km.get_mapping(&Mode::Normal, &keys).unwrap();
//         let right = CursorTopOfBuffer.into();
//         assert_eq!(left, right);
//     }
// }
