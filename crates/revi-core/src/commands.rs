use revi_ui::tui::layout::{Pos, Size};

use crate::context::{Context, Id};
use crate::mode::Mode;
use crate::{panes::MessageBox, Buffer, Event};
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::process;
use std::rc::Rc;

pub trait Command: fmt::Debug {
    fn call(&self, ctx: Context);
    fn equal(&self, other: &dyn Command) -> bool;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! build_command {
    ($([doc:  $doc:expr])? $name:ident$(($($ty:ty $(,)?)*))?; $caller:expr) => {
        $(#[doc=$doc])?
        #[derive(Debug, PartialEq)]
        pub struct $name $(($(pub $ty, )*))?;
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
    UserCommand(Id);
    |Self(id): &UserCommand, ctx: Context| {
        let command_list = ctx.rhai_commands.borrow();
        let Some(fnptr) = &command_list.get(id) else {
            Message("no command with this name is found".into(), format!("{id:?}")).call(ctx.clone());
            return;
        };
        let rhai = ctx.rhai.borrow_mut();
        let engine = &rhai.engine;
        let ast = &rhai.ast;
            // let name = fnptr.fn_name();
            if let Err(err_message) = fnptr.call::<()>(engine, ast, ()) {
            Message(err_message.to_string(), "".into()).call(ctx.clone());
        }
            // .expect(&format!("failed to execute user command '{name}'"));
    }
);

build_command!(
    CursorUp;
    |_: &CursorUp, ctx: Context| {
        ctx.focused_pane().borrow_mut().move_cursor_up();
    }
);
build_command!(
    CursorDown;
    |_: &CursorDown, ctx: Context| {
        ctx.focused_pane().borrow_mut().move_cursor_down();
    }
);
build_command!(
    CursorLeft;
    |_: &CursorLeft, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Command => {
                ctx.command_bar.borrow_mut().move_cursor_left();
            }
            _ => {
                ctx.focused_pane().borrow_mut().move_cursor_left();
            }
        }
    }
);

build_command!(
    CursorRight;
    |_: &CursorRight, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Command => {
                ctx.command_bar.borrow_mut().move_cursor_right();
            }
            _ => {
                ctx.focused_pane().borrow_mut().move_cursor_right();
            }
        }
    }
);

build_command!(
    ExeCommandList(Vec<CmdRc>);
    |ecl: &ExeCommandList, ctx: Context| {
        for cmd in ecl.0.iter() {
            cmd.call(ctx.clone());
        }
    }
);

build_command!(
    ScrollUp;
    |_: &ScrollUp, ctx: Context| {
        ctx.focused_pane().borrow_mut().scroll_up();
    }
);

build_command!(
    ScrollDown;
    |_: &ScrollDown, ctx: Context| {
        ctx.focused_pane().borrow_mut().scroll_down();
    }
);

build_command!(
    BackSpace;
    |_: &BackSpace, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Command => {
                ctx.command_bar.borrow_mut().backspace();
            }
            _ => {
                ctx.focused_pane().borrow_mut().backspace();
            }
        }
    }
);

build_command!(
    InsertChar(char);
    |InsertChar(c): &InsertChar, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Insert => {
                let pane = ctx.focused_pane();
                let mut pane = pane.borrow_mut();
                pane.insert_char(*c);
                if c == &'\n' {
                    pane.move_cursor_down();
                    pane.move_cursor_home();
                    return;
                }
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
    ChangeMode(Mode);
    |Self(mode): &ChangeMode, ctx: Context| {
        let (cmd_focused, pane_focused) = match &mode {
            Mode::Command => (true, false),
            Mode::Normal => (false, true),
            Mode::Insert => (false, true),
        };
        let mut bar = ctx.command_bar.borrow_mut();
        bar.set_focused(cmd_focused);
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.set_focused(pane_focused);
        *ctx.mode.borrow_mut() = *mode;

    }
);

build_command!(
    ExecuteCommandLine;
    |_: &ExecuteCommandLine, ctx: Context| {
        ChangeMode(crate::mode::Mode::Normal).call(ctx.clone());
        let mut bar = ctx.command_bar.borrow_mut();
        bar.get_cursor_pos_mut().map(|mut c| {
            c.pos.x = 0;
            c
        });
        let command = bar.get_buffer_contents().trim().to_string();
        bar.clear_buffer();

        let (cmd, _) = command.split_once(' ').unwrap_or((command.as_str(), ""));
        match cmd {
            c if c.starts_with('!')=> ExecuteTerminalCommand(command[1..].trim().into()).call(ctx.clone()),
            "exit" | "quit" | "q" => Quit.call(ctx.clone()),
            "write" | "w" => SaveFile.call(ctx.clone()),
            "edit" | "e" => EditFile(command).call(ctx.clone()),
            "buffer" | "b" => JumpToBuffer(command).call(ctx.clone()),
            "ls" => ListBuffers.call(ctx.clone()),
            name => {
                let user_command_list = ctx.rhai_commands.borrow();
                let Some(fnptr) = user_command_list.get(&Id::Name(name.into())) else {
                    Message("no command with this name".into(), name.to_string()).call(ctx.clone());
                    return;
                };
                let rhai = ctx.rhai.borrow_mut();
                let engine = &rhai.engine;
                let ast = &rhai.ast;
                    // let name = fnptr.fn_name();
                    if let Err(err_message) = fnptr.call::<()>(engine, ast, ()) {
                    Message(err_message.to_string(), "".into()).call(ctx.clone());
                }

            },
        }
    }
);

build_command!(
    ExecuteTerminalCommand(String);
    |Self(command): &ExecuteTerminalCommand, ctx: Context| {
        let Some((head, args)) = command.split_once(' ').or(Some((command, ""))) else {
            return;
        };
        let args = args.split(' ').collect::<Vec<_>>();
        let mut cmd =  process::Command::new(head);
        if !args.first().cloned().unwrap_or_default().is_empty() {
            cmd.args(&args);
        }

        let message = cmd.output().map(|output| {
            let stderr = String::from_utf8(output.stderr).ok().unwrap_or_default();
            let stdout = String::from_utf8(output.stdout).ok().unwrap_or_default();
            format!("{stderr}\n{stdout}")
        }).unwrap_or_default();
        Message(message.trim().to_string(), command.into()).call(ctx);
    }
);

build_command!(
    Quit;
    |_: &Quit, ctx: Context| {
        *ctx.is_running.borrow_mut() = false;
    }
);

build_command!(
    Delete;
    |_: &Delete, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Command => {
                ctx.command_bar.borrow_mut().delete();
            }
            _ => {
                ctx.focused_pane().borrow_mut().delete();
            }
        }
    }
);

build_command!(
    DeleteLine;
    |_: &DeleteLine, ctx: Context| {
        let mode = *ctx.mode.borrow();
        match mode {
            Mode::Command => {
                ctx.command_bar.borrow_mut().delete_line();
            }
            _ => {
                ctx.focused_pane().borrow_mut().delete_line();
            }
        }
    }
);

build_command!(
    Message(String, String);
    |Self(message, footer): &Message, ctx: Context| {
        let id = ctx.panes.borrow().len();
        *ctx.focused_pane.borrow_mut() = id;
        // let Size { width, height } = ctx.window_size();
        // let pos = Pos { x: (width/2)/2, y: (height/2)/2};
        let width = ctx.window_size().width;
        let height = message.lines().count() as u16;
        let pos = Pos { x: 0, y: 0 };
        let size = Size { width, height };
        let buffer = Rc::new(RefCell::new(Buffer::new_str("", message)));


        let message_box = Rc::new(RefCell::new(MessageBox::new(pos, size, buffer).with_footer(footer)));
        let id = ctx.panes.borrow().len();
        *ctx.focused_pane.borrow_mut() = id;
        ctx.panes.borrow_mut().push(message_box);
        *ctx.event.borrow_mut() = Event::Message;
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
build_command!(
    SaveFile;
    |_: &SaveFile, ctx: Context| {
        use std::fs::File;
        use std::io::BufWriter;
        let id = *ctx.focused_pane.borrow();
        let buf = ctx.buffers.borrow();
        let buf = buf[id].borrow();
        let name = &buf.name;
        File::create(name)
            .map(BufWriter::new)
            .and_then(|b|buf.get_rope().write_to(b))
            .map_err(|err|Message(
                        err.to_string(),
                        String::new()
                    ).call(ctx.clone()))
            .ok();
    }
);

build_command!(
    [doc: "loads a file from disk if it exsists and if not just create a empty file
    and place it in buffer list and put the current file buffer into the current window."]
    EditFile(String);
    |Self(command): &EditFile, ctx: Context| {
        let Some((_, name)) = command.split_once(' ') else {
            let msg = "edit command requires a file path";
            Message(msg.into(), command.into()).call(ctx);
            return;
        };
        let text = std::fs::read_to_string(name).unwrap_or("\n".into());
        let rope = ropey::Rope::from_str(&text);
        let new_buf = Rc::new(RefCell::new(Buffer::new(name, rope)));
        let mut buf_list = ctx.buffers.borrow_mut();
        buf_list.push(new_buf.clone());
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.set_buffer(new_buf);
    }
);

build_command!(
    [doc: "command `buffer | b` `JumpToBuffer` swaps Pane's buffer with selected buffer
    which could be a file path or a index of buffer in list of buffers open."]
    JumpToBuffer(String);
    |Self(command): &JumpToBuffer, ctx: Context| {
        let Some((_, name)) = command.split_once(' ') else {
            Message(
                "expected a arguement".into(),
                command.into())
                .call(ctx);
            return;
        };
        if let Ok(idx) = name.parse::<usize>() {
            let bufs = ctx.buffers.borrow();
            let Some(buf) =  bufs.get(idx) else {
                Message(
                    "buffer id does not exsist".into(),
                    command.into())
                    .call(ctx.clone());
                return;
            };
            let pane = ctx.focused_pane();
            let mut pane = pane.borrow_mut();
            pane.set_buffer(buf.clone());
            return;
        }
        let buffers = ctx.buffers.borrow();
        let Some(buf) = buffers.iter().find(|b| b.borrow().name == name) else {
            Message(
                "name buffer open with that name or path".into(),
                command.into())
                .call(ctx.clone());
            return;
        };
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.set_buffer(buf.clone());
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

build_command!(
    ListBuffers;
    |_: &ListBuffers, ctx: Context| {
        let buf = ctx.buffers.borrow().clone();
        let paths = buf.iter()
             .enumerate()
             .map(|(i, b)|format!("{} {}", i, b.borrow().name))
             .collect::<Vec<String>>();
        let msg = paths.join("\n");
        Message(msg, "ls List Buffers".into()).call(ctx);
    }
);
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

build_command!(
    CursorHome;
    |_: &CursorHome, ctx: Context| {
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.move_cursor_home();
    }
);

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

build_command!(
    CursorTopOfBuffer;
    |_: &CursorTopOfBuffer, ctx: Context| {
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.move_cursor_top_of_buffer();
    }
);

build_command!(
    CursorToBottomOfBuffer;
    |_: &CursorToBottomOfBuffer, ctx: Context| {
        let pane = ctx.focused_pane();
        let mut pane = pane.borrow_mut();
        pane.move_cursor_bottom_of_buffer();

    }
);

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

#[test]
fn test_parsual_eq() {
    assert_eq!(Into::<CmdRc>::into(Quit), Quit.into());
}
