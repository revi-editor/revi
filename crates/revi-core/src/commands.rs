use crate::context::Context;
use crate::mode::Mode;
use std::fmt;
use std::rc::Rc;
use std::any::Any;

pub trait Command: fmt::Debug {
    fn call(&self, ctx: Context);
    fn equal(&self, other: &dyn Command) -> bool;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! build_command {
    ($name:ident $(, $ty:ty)?; $caller:expr) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name $((pub $ty))?;
        impl Command for $name {
            fn call(&self, ctx: Context) {
                $caller(&self, ctx);
            }
            fn equal(&self, other: &dyn Command) -> bool {
                other.as_any().downcast_ref::<Self>().map_or(false, |i| self==i)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl From<$name> for CmdRc {
            fn from(value: $name) -> Self {
                Self(Rc::new(value))
            }
        }
    };
}

#[derive(Clone)]
pub struct CmdRc(Rc<dyn Command>);

impl CmdRc {
    pub fn call(&self, ctx: Context) {
        self.0.call(ctx);
    }
}

impl std::fmt::Debug for CmdRc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl CmdRc {
    pub fn new(command: impl Command + 'static) -> Self {
        Self(Rc::new(command))
    }
}

impl PartialEq for CmdRc {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&*other.0)
    }
}

build_command!(
    CursorUp;
    |_: &CursorUp, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().move_cursor_up();
    }
);
build_command!(
    CursorDown;
    |_: &CursorDown, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().move_cursor_down();
    }
);
build_command!(
    CursorLeft;
    |_: &CursorLeft, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().move_cursor_left();
    }
);
build_command!(
    CursorRight;
    |_: &CursorRight, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().move_cursor_right();
    }
);
build_command!(
    ScrollUp;
    |_: &ScrollUp, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().scroll_up();
    }
);
build_command!(
    ScrollDown;
    |_: &ScrollDown, ctx: Context| {
        ctx.panes[ctx.focused_pane].borrow_mut().scroll_down();
    }
);

// build_command!(
//     Home,
//     6;
//     |_: &Home, _ctx: Context| {
//     }
// );

// build_command!(
//     End,
//     7;
//     |_: &End, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().end();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );

// build_command!(
//     MoveForwardByWord,
//     8;
//     |_: &MoveForwardByWord, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().move_forward_by_word();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );

// build_command!(
//     MoveBackwardByWord,
//     9;
//     |_: &MoveBackwardByWord, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().move_backward_by_word();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     JumpToFirstLineBuffer,
//     10;
//     |_: &JumpToFirstLineBuffer, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().jump_to_first_line_buffer();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     JumpToLastLineBuffer,
//     11;
//     |_: &JumpToLastLineBuffer, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().jump_to_last_line_buffer();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     Backspace,
//     12;
//     |_: &Backspace, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().backspace();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     NewLine,
//     13;
//     |_: &NewLine, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().insert_newline();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     FirstCharInLine,
//     14;
//     |_: &FirstCharInLine, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().first_char_in_line();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     DeleteChar,
//     15;
//     |_: &DeleteChar, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window_mut().delete();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     DeleteLine,
//     16;
//     |_: &DeleteLine, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         let line = revi.focused_window_mut().delete_line();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//         revi.clipboard.clear();
//         revi.clipboard.push_str(line.as_str());
//     }
// );
//
// build_command!(
//     YankLine,
//     17;
//     |_: &YankLine, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         let yanked_line;
//         {
//             let cursor = revi.focused_window().cursor_file();
//             let line = cursor.as_usize_y();
//             let buffer = revi.focused_window().buffer();
//             yanked_line = buffer.line(line);
//         }
//         revi.clipboard.clear();
//         revi.clipboard.push_str(yanked_line.as_str());
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     Paste,
//     18;
//     |_: &Paste, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//         // TODO: Fix this cloning.
//         let clipboard = revi.clipboard.clone();
//         {
//             let window = revi.focused_window_mut();
//             let line_idx = window.cursor_file().as_usize_y();
//             let mut buffer = window.buffer_mut();
//             buffer.insert_line(line_idx + 1, &clipboard);
//         }
//         revi.focused_window_mut().move_cursor_down(1);
//     }
// );
//
// build_command!(
//     PasteBack,
//     19;
//     |_: &PasteBack, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//         // TODO: Fix this cloning.
//         let clipboard = revi.clipboard.clone();
//         {
//             let window = revi.focused_window_mut();
//             let line_idx = window.cursor_file().as_usize_y();
//             let mut buffer = window.buffer_mut();
//             buffer.insert_line(line_idx + 1, &clipboard);
//         }
//     }
// );
//
build_command!(
    InsertChar,
    char;
    |InsertChar(c): &InsertChar, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Insert => {
                let id = ctx.focused_pane;
                let mut pane = ctx.panes[id].borrow_mut();
                pane.insert_char(*c);
                pane.move_cursor_right();
            }
            Mode::Command => {
                let mut bar = ctx.command_bar.borrow_mut();
                bar.insert_char(*c);
                bar.move_cursor_right();
            }
            _ => {},
        }
    }
);

build_command!(
    ChangeMode,
    Mode;
    |Self(mode): &ChangeMode, ctx: Context| {
        let (cmd_focused, pane_focused) = match &mode {
            Mode::Command => (true, false),
            Mode::Normal => (false, true),
            Mode::Insert => (false, true),
        };
        let mut bar = ctx.command_bar.borrow_mut();
        bar.set_focused(cmd_focused);
        let id = ctx.focused_pane;
        let mut pane = ctx.panes[id].borrow_mut();
        pane.set_focused(pane_focused);
        *ctx.mode.borrow_mut() = *mode;

    }
);

// build_command!(
//     FocusedPane,
//     22,
//     PaneId;
//     |fpane: &FocusedPane, ctx: Context| {
//         match fpane.0 {
//             PaneId::CommandBar => {
//                 let id = ctx.focused_pane;
//             }
//             PaneId::Number(id) =>
//         }
//     }
// );
// build_command!(
//     EnterCommandMode,
//     22;
//     |_: &EnterCommandMode, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.enter_command_mode();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );

// build_command!(
//     ExitCommandMode,
//     23;
//     |_: &ExitCommandMode, ctx: Context| {
//     }
// );

build_command!(
    ExecuteCommandLine;
    |_: &ExecuteCommandLine, ctx: Context| {
        ChangeMode(crate::mode::Mode::Normal).call(ctx.clone());
        let mut bar = ctx.command_bar.borrow_mut();
        if let Some(cursor) = bar.get_cursor_pos_mut() {
            cursor.pos.x = 0;
        }
        let command = bar.get_buffer_contents();
        bar.clear_buffer();
        match command.as_str() {
            "exit" | "quit" | "q" => Quit.call(ctx.clone()),
            _ => {},
        }
    }
);

// build_command!(
//     NextWindow,
//     25;
//     |_: &NextWindow, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.next_window();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );
//
// build_command!(
//     Save,
//     26;
//     |_: &Save, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.focused_window().save();
//         let focused_window = revi.focused;
//         revi.queue.push(focused_window);
//     }
// );

build_command!(
    Quit;
    |_: &Quit, ctx: Context| {
        *ctx.is_running.borrow_mut() = false;
    }
);

// build_command!(
//     CloseWindow,
//     28;
//     |_: &CloseWindow, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.close_current_window();
//     }
// );
//
// build_command!(
//     ListBuffers,
//     29;
//     |_: &ListBuffers, revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         revi.list_buffers();
//     }
// );
//
// build_command!(
//     InsertTab,
//     30;
//     |_: &InsertTab, revi_rc: Rc<RefCell<ReVi>>, count: usize| {
//         let mut revi = revi_rc.borrow_mut();
//         for _ in 0..revi.settings.tab_width+count {
//             revi.focused_window_mut().insert_char(' ');
//         }
//     }
// );
//
// build_command!(
//     JumpListBack,
//     31;
//     |_: &JumpListBack, _revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         unimplemented!("JumpListBack");
//     }
// );
//
// build_command!(
//     JumpListForward,
//     32;
//     |_: &JumpListForward, _revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         unimplemented!("JumpListForward");
//     }
// );
//
// build_command!(
//     Undo,
//     33;
//     |_: &Undo, _revi_rc: Rc<RefCell<ReVi>>, _: usize| {
//         unimplemented!("Undo");
//     }
// );

#[test]
fn test_parsual_eq() {
    assert_eq!(Into::<CmdRc>::into(Quit), Quit.into());
}
