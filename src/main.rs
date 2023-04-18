#![warn(clippy::all)]

const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

const LINUX_CONFIG_PATH: &str = "/.config/revi/init.rhai";

mod commandline;
use revi_core::{
    commands::{BoxedCommand, InsertChar},
    Mapper, Mode, ReVi, Settings,
};
use revi_ui::{Key, Tui};

use std::cell::RefCell;
use std::rc::Rc;
fn execute(revi: Rc<RefCell<ReVi>>, count: usize, commands: &[BoxedCommand]) {
    for boxed in commands {
        boxed.command.call(revi.clone(), count);
    }
}

fn insert_chars(tui: &mut Tui, input: &mut Input, revi: Rc<RefCell<ReVi>>) {
    let input_chars = input
        .as_chars()
        .iter()
        .filter(|c| **c != '\0')
        .map(|c| InsertChar(*c).into())
        .collect::<Vec<BoxedCommand>>();
    execute(revi.clone(), input.number_usize(), &input_chars);
    input.clear();
    tui.update(&mut *revi.borrow_mut());
}

fn main() {
    let home_path = env!("HOME").to_string();
    let config_file_path = format!("{home_path}{LINUX_CONFIG_PATH}");
    // let _config_file = std::fs::read_to_string()
    //     .expect(&format!(
    //         "failed to read in config file at path '{config_file_path}{LINUX_CONFIG_PATH}'"
    //     ));
    let files = commandline::args();

    let settings = Settings::default();
    let keymapper = Mapper::default();
    let revi = ReVi::new(settings, &files);
    let (engine, mut scope) = revi_core::api::init_api(revi.clone()).expect("failed to init api");
    engine.eval_file_with_scope::<()>(&mut scope, config_file_path.into()).expect("failed to eval init file");

    let mut tui = Tui::default();
    let mut input = Input::default();

    input.clear();
    tui.update(&mut *revi.borrow_mut());

    while revi.borrow().is_running {
        let mode = *revi.borrow().mode();
        if tui.poll_read(std::time::Duration::from_millis(50)) {
            let keys = tui.get_key_press();
            input.input(mode, keys);
            let commands = keymapper.get_mapping(mode, input.keys());
            match (mode, commands) {
                (_, Some(cmd)) => {
                    execute(revi.clone(), input.number_usize(), &cmd);
                    tui.update(&mut *revi.borrow_mut());
                    input.clear();
                }
                (Mode::Insert, None) => insert_chars(&mut tui, &mut input, revi.clone()),
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Number {
    inner: Vec<u16>,
}

#[allow(dead_code)]
impl Number {
    pub fn push(&mut self, num: usize) {
        self.inner.push(num as u16);
    }

    pub fn as_u16(&self) -> u16 {
        let mut number = 0;
        for (i, n) in self.inner.iter().rev().enumerate() {
            number += 10_u16.pow(i as u32) * n;
        }
        if number == 0 {
            1
        } else {
            number
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn as_usize(&self) -> usize {
        self.as_u16() as usize
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Input {
    number: Number,
    input_keys: Vec<Key>,
    chars: Vec<char>,
}

impl Input {
    fn insert_number(&mut self, (k1, _): (Key, Key)) {
        if self.input_keys.is_empty() && k1.try_digit().is_some() {
            self.number.push(k1.try_digit().unwrap());
        }
    }

    fn insert_key(&mut self, mode: Mode, (k1, k2): (Key, Key)) {
        match k1 {
            Key::Esc | Key::LH | Key::LJ | Key::LK | Key::LL if mode == Mode::Normal => {
                self.input_keys.clear();
                self.input_keys.push(k1);
                if k2 != Key::Null {
                    self.input_keys.push(k2);
                }
            }
            _ if mode == Mode::Insert || mode == Mode::Command => {
                let c = k1.as_char();
                if c == '\0' {
                    self.input_keys.clear();
                    self.input_keys.push(k1);
                    if k2 != Key::Null {
                        self.input_keys.push(k2);
                    }
                    return;
                }
                self.chars.push(c);
            }
            _ if mode == Mode::Normal && k1.try_digit().filter(|c| *c != 0).is_none() => {
                if k1.try_digit().is_some() && self.number.is_empty() {
                    self.input_keys.push(k1);
                    return;
                }
                self.input_keys.push(k1);
                if k2 != Key::Null {
                    self.input_keys.push(k2);
                }
            }
            _ => {}
        }
    }

    pub fn input(&mut self, mode: Mode, (k1, k2): (Key, Key)) {
        if k1 == Key::Null {
            return;
        }
        self.insert_number((k1, k2));
        self.insert_key(mode, (k1, k2));
    }

    pub fn keys(&mut self) -> &[Key] {
        // self.chars.clear();
        &self.input_keys
    }

    pub fn number_u16(&mut self) -> u16 {
        let n = self.number.as_u16();
        self.number.clear();
        n
    }

    pub fn number_usize(&mut self) -> usize {
        let n = self.number.as_usize();
        self.number.clear();
        n
    }

    pub fn clear(&mut self) {
        self.number.clear();
        self.input_keys.clear();
        self.chars.clear();
    }

    pub fn as_chars(&mut self) -> Vec<char> {
        let c = self.chars.clone();
        self.chars.clear();
        c
    }
}
