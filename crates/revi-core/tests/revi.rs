extern crate revi_core;
use revi_core::line_number::LineNumberKind;
use revi_core::mode::Mode;
use revi_core::{ReVi, Settings};

#[test]
fn test_cursor_position_with_lines() {
    let settings = Settings {
        line_number_kind: LineNumberKind::None,
        tab_width: 4,
    };
    let files = vec![String::from("test.txt")];
    let revi = ReVi::new(settings, &files);
    assert_eq!(revi.borrow().cursor_position_u16(), (5, 0));
}

#[test]
fn test_set_cursor_position_with_lines() {
    let settings = Settings {
        line_number_kind: LineNumberKind::None,
        tab_width: 4,
    };
    let files = vec![String::from("test.txt")];
    let revi = ReVi::new(settings, &files);
    revi.borrow_mut().set_cursor_position(20, 20);
    assert_eq!(revi.borrow().cursor_position_u16(), (25, 20));
}

#[test]
fn test_getting_mode_and_setting_mode() {
    let settings = Settings {
        line_number_kind: LineNumberKind::None,
        tab_width: 4,
    };
    let files = vec![String::from("test.txt")];
    let revi = ReVi::new(settings, &files);
    assert_eq!(revi.borrow().mode(), &Mode::Normal);
    *revi.borrow_mut().mode_mut() = Mode::Insert;
    assert_eq!(revi.borrow().mode(), &Mode::Insert);
    *revi.borrow_mut().mode_mut() = Mode::Command;
    assert_eq!(revi.borrow().mode(), &Mode::Command);
}

#[test]
fn test_start_queued() {
    let settings = Settings {
        line_number_kind: LineNumberKind::None,
        tab_width: 4,
    };
    let files = vec![String::from("test.txt")];
    let revi = ReVi::new(settings, &files);
    assert_eq!(revi.borrow_mut().queued().len(), 2);
}

#[test]
fn test_is_running_and_exit() {
    let settings = Settings {
        line_number_kind: LineNumberKind::None,
        tab_width: 4,
    };
    let files = vec![String::from("test.txt")];
    let revi = ReVi::new(settings, &files);
    assert!(revi.borrow().is_running);
    revi.borrow_mut().exit();
    assert!(!revi.borrow().is_running);
}
