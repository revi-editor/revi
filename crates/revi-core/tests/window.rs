extern crate revi_core;
use revi_core::buffer::Buffer;
use revi_core::line_number::LineNumbers;
use revi_core::window::Window;
use std::{cell::RefCell, rc::Rc};

const TEST_FILE: &'static str = "./tests/test.txt";

#[test]
fn test_window_width() {
    let buffer = Rc::new(RefCell::new(Buffer::new()));
    let window = Window::new(100, 20, buffer);
    assert_eq!(window.width(), 100);
}

#[test]
fn test_window_height() {
    let buffer = Rc::new(RefCell::new(Buffer::new()));
    let window = Window::new(100, 20, buffer);
    assert_eq!(window.height(), 20);
}

#[test]
fn test_window_text_width() {
    let buffer = Rc::new(RefCell::new(Buffer::new()));
    let window = Window::new(100, 20, buffer).with_line_numbers(LineNumbers::RelativeNumber);
    assert_eq!(window.text_width(), 95);
}

#[test]
fn test_window_() {
    let buffer = Rc::new(RefCell::new(Buffer::new()));
    let window = Window::new(100, 20, buffer);
    assert_eq!(window.text_width(), 100);
}

#[test]
fn test_move_cursor_down() {
    let buffer = Rc::new(RefCell::new(Buffer::from_path(TEST_FILE)));
    let mut window = Window::new(100, 20, buffer);
    assert_eq!(window.cursor_screen().as_u16(), (0, 0));
    window.move_cursor_down(1);
    assert_eq!(window.cursor_screen().as_u16(), (0, 1));
}

#[test]
fn test_can_not_move_cursor_passed_end_of_last_line() {
    let buffer = Rc::new(RefCell::new(Buffer::from_path(TEST_FILE)));
    let mut window = Window::new(100, 20, buffer);
    assert_eq!(window.cursor_screen().as_u16(), (0, 0));
    window.jump_to_last_line_buffer();
    assert_eq!(window.cursor_screen().as_u16(), (0, 8));
}
