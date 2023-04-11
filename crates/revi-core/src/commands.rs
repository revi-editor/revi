use std::rc::Rc;
use crate::mode::Mode;
use crate::revi::ReVi;
use std::fmt;

#[macro_export]
macro_rules! commands {
    ( $( $x:ident $(($($args:expr),*))? ),* ) => {
        vec![$(BoxedCommand { command: std::rc::Rc::new($x $(($($args),*))?) }),*]
    }

}

macro_rules! build_command {
    ($name:ident, $counter:expr $(, $ty:ty)?; $caller:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $name $((pub $ty))?;
        impl Command for $name {
            fn call(&self, revi: &mut ReVi, count: usize) {
                $caller(&self, revi, count);
            }
            fn id(&self) -> usize {
                $counter
            }
        }
        impl From<$name> for BoxedCommand {
            fn from(value: $name) -> Self {
                Self {
                    command: std::rc::Rc::new(value),
                }
            }
        }
    };
}

pub trait Command: fmt::Debug {
    fn call(&self, revi: &mut ReVi, count: usize);
    fn id(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct BoxedCommand {
    pub command: Rc<dyn Command>,
}

impl PartialEq for BoxedCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command.id() == other.command.id()
    }
}

build_command!(
    CursorUp,
    0;
    |_: &CursorUp, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().move_cursor_up(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    CursorDown,
    1;
    |_: &CursorDown, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().move_cursor_down(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    CursorLeft,
    2;
    |_: &CursorLeft, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().move_cursor_left(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    CursorRight,
    3;
    |_: &CursorRight, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().move_cursor_right(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    ScrollUp,
    4;
    |_: &ScrollUp, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().scroll_up(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    ScrollDown,
    5;
    |_: &ScrollDown, revi: &mut ReVi, count: usize| {
        revi.focused_window_mut().scroll_up(count);
        revi.queue.push(revi.focused);
    }
);
build_command!(
    Home,
    6;
    |_: &Home, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().home();
        revi.queue.push(revi.focused);
    }
);
build_command!(
    End,
    7;
    |_: &End, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().end();
        revi.queue.push(revi.focused);
    }
);
build_command!(
    MoveForwardByWord,
    8;
    |_: &MoveForwardByWord, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().move_forward_by_word();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    MoveBackwardByWord,
    9;
    |_: &MoveBackwardByWord, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().move_backward_by_word();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    JumpToFirstLineBuffer,
    10;
    |_: &JumpToFirstLineBuffer, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().jump_to_first_line_buffer();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    JumpToLastLineBuffer,
    11;
    |_: &JumpToLastLineBuffer, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().jump_to_last_line_buffer();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    Backspace,
    12;
    |_: &Backspace, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().backspace();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    NewLine,
    13;
    |_: &NewLine, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().insert_newline();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    FirstCharInLine,
    14;
    |_: &FirstCharInLine, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().first_char_in_line();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    DeleteChar,
    15;
    |_: &DeleteChar, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().delete();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    DeleteLine,
    16;
    |_: &DeleteLine, revi: &mut ReVi, _: usize| {
        let line = revi.focused_window_mut().delete_line();
        revi.queue.push(revi.focused);
        revi.clipboard.clear();
        revi.clipboard.push_str(line.as_str());
    }
);

build_command!(
    YankLine,
    17;
    |_: &YankLine, revi: &mut ReVi, _: usize| {
        let yanked_line;
        {
            let cursor = revi.focused_window().cursor_file();
            let line = cursor.as_usize_y();
            let buffer = revi.focused_window().buffer();
            yanked_line = buffer.line(line);
        }
        revi.clipboard.clear();
        revi.clipboard.push_str(yanked_line.as_str());
        revi.queue.push(revi.focused);
    }
);

build_command!(
    Paste,
    18;
    |_: &Paste, revi: &mut ReVi, _: usize| {
        revi.queue.push(revi.focused);
        // TODO: Fix this cloning.
        let clipboard = revi.clipboard.clone();
        {
            let window = revi.focused_window_mut();
            let line_idx = window.cursor_file().as_usize_y();
            let mut buffer = window.buffer_mut();
            buffer.insert_line(line_idx + 1, &clipboard);
        }
        revi.focused_window_mut().move_cursor_down(1);
    }
);

build_command!(
    PasteBack,
    19;
    |_: &PasteBack, revi: &mut ReVi, _: usize| {
        revi.queue.push(revi.focused);
        // TODO: Fix this cloning.
        let clipboard = revi.clipboard.clone();
        {
            let window = revi.focused_window_mut();
            let line_idx = window.cursor_file().as_usize_y();
            let mut buffer = window.buffer_mut();
            buffer.insert_line(line_idx + 1, &clipboard);
        }
    }
);

build_command!(
    InsertChar,
    20,
    char;
    |insert_char: &InsertChar, revi: &mut ReVi, _: usize| {
        revi.focused_window_mut().insert_char(insert_char.0);
        revi.queue.push(revi.focused);
    }
);

build_command!(
    ChangeMode,
    21,
    Mode;
    |change_mode: &ChangeMode, revi: &mut ReVi, _: usize| {
        revi.change_modes(change_mode.0);
        revi.queue.push(revi.focused);
    }
);

build_command!(
    EnterCommandMode,
    22;
    |_: &EnterCommandMode, revi: &mut ReVi, _: usize| {
        revi.enter_command_mode();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    ExitCommandMode,
    23;
    |_: &ExitCommandMode, revi: &mut ReVi, _: usize| {
        if revi.focused == 0 {
            revi.exit_command_mode();
            revi.queue.push(revi.focused);
        }
    }
);

build_command!(
    ExcuteCommandLine,
    24;
    |_: &ExcuteCommandLine, revi: &mut ReVi, _: usize| {
        if revi.focused == 0 {
            eprintln!("executing command");
            revi.execute_command_line();
        }
    }
);

build_command!(
    NextWindow,
    25;
    |_: &NextWindow, revi: &mut ReVi, _: usize| {
        revi.next_window();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    Print,
    26,
    String;
    |p: &Print, revi: &mut ReVi, _: usize| {
        revi.print(&p.0);
        revi.queue.push(0);
    }
);

build_command!(
    Save,
    27;
    |_: &Save, revi: &mut ReVi, _: usize| {
        revi.focused_window().save();
        revi.queue.push(revi.focused);
    }
);

build_command!(
    Quit,
    28;
    |_: &Quit, revi: &mut ReVi, _: usize| {
        revi.exit();
    }
);

build_command!(
    CloseWindow,
    29;
    |_: &CloseWindow, revi: &mut ReVi, _: usize| {
        revi.close_current_window();
    }
);

build_command!(
    ListBuffers,
    30;
    |_: &ListBuffers, revi: &mut ReVi, _: usize| {
        revi.list_buffers();
    }
);

build_command!(
    InsertTab,
    31;
    |_: &InsertTab, revi: &mut ReVi, count: usize| {
        for _ in 0..revi.tab_width+count {
            revi.focused_window_mut().insert_char(' ');
        }
    }
);

build_command!(
    JumpListBack,
    32;
    |_: &JumpListBack, _revi: &mut ReVi, _: usize| {
        unimplemented!("JumpListBack");
    }
);

build_command!(
    JumpListForward,
    33;
    |_: &JumpListForward, _revi: &mut ReVi, _: usize| {
        unimplemented!("JumpListForward");
    }
);

build_command!(
    Undo,
    34;
    |_: &Undo, _revi: &mut ReVi, _: usize| {
        unimplemented!("Undo");
    }
);

