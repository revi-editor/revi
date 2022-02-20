// noun (adjective) verb
// w    d
// word delete
//
//    w
//  -------
// ↓       ↓
// commands can be terminator type or extender
//
//
// verb (adjective) noun
// d     i          w
#![allow(unused)]
// use revi_core::commands::{
//     Backspace, BoxedCommand, ChangeMode, Command, CursorDown, CursorLeft, CursorRight, CursorUp,
//     DeleteChar, DeleteLine, End, EnterCommandMode, ExcuteCommandLine, ExitCommandMode,
//     FirstCharInLine, Home, JumpToFirstLineBuffer, JumpToLastLineBuffer, MoveBackwardByWord,
//     MoveForwardByWord, NewLine, NextWindow, Paste, PasteBack, Quit, Save, ScrollDown, ScrollUp,
//     YankLine,
// };
// use revi_core::{Mapper, Mode};
use revi_ui::{Key, Keys};

// pub fn keys_parser<'a>(map: &'a Mapper, mode: Mode, keys: &[Key]) -> Option<&'a Vec<BoxedCommand>> {
//     map.get_mapping(mode, keys)
// }

use std::{iter::Peekable, str::Chars};
fn specal_keys<'a>(stream: &mut Peekable<Chars<'a>>) -> Vec<Key> {
    let mut string = String::new();
    while let Some(c) = stream.next_if(|c| c != &'>') {
        string.push(c);
    }
    let _ = stream.next();
    if string.contains("-") {
        let lr = string.split("-").collect::<Vec<&str>>();
        let modifier = lr[0].to_lowercase();
        let modifier = match modifier.as_str() {
            "c" => "ctrl",
            "a" => "alt",
            m => m,
        };
        return vec![
            Key::from(lr[1].chars().collect::<Vec<char>>()[0]),
            Key::from(modifier),
        ];
    }
    vec![Key::from(string.as_str())]
}

pub fn string_to_key(keys_string: &str) -> Vec<Key> {
    let mut stream = keys_string.chars().peekable();
    let mut keys = Vec::new();
    while let Some(c) = stream.next() {
        match c {
            '<' => keys.append(&mut specal_keys(&mut stream)),
            _ => keys.push(Key::from(c)),
        }
    }
    keys
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn specal_keys_parser() {
        let string = "esc>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Esc], specal_keys(&mut string.chars().peekable()));
        let string = "space>";
        eprintln!("{:?}", string);
        assert_eq!(
            vec![Key::Space],
            specal_keys(&mut string.chars().peekable())
        );
        let string = "C-c>";
        eprintln!("{:?}", string);
        assert_eq!(
            vec![Key::Ctrl, Key::LC],
            specal_keys(&mut string.chars().peekable())
        );
    }

    #[test]
    fn parse_ctrl_a() {
        let string = "<C-a>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Ctrl, Key::LA], string_to_key(string));
    }

    #[test]
    fn parse_esc() {
        let string = "<esc>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Esc], string_to_key(string));
    }

    #[test]
    fn parse_ctrl_h() {
        let string = "<C-h>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Ctrl, Key::LH], string_to_key(string));
    }

    #[test]
    fn parse_i() {
        let string = "i";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::LI], string_to_key(string));
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_I() {
        let string = "I";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::UI], string_to_key(string));
    }
}
