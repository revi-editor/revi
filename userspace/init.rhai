let go_to_top_of_file = || {
  revi.set_cursor_row(0);
  revi.set_scroll_row(0);
};

revi.nmap("gg", go_to_top_of_file);

let go_to_home = || {
  revi.set_cursor_col(0);
};
revi.nmap("0", go_to_home);
revi.nmap("tt", "jjjj");
revi.message("init file loaded");


let UserCommand = || {
  revi.message("hello");
};

revi.export_command(UserCommand);

// revi.nmap("er", ":UserCommand<CR>");




























// revi.nmap("<up>", "k");
// revi.nmap("<down>", "j");
// revi.nmap("<right>", "l");
// revi.nmap("<left>", "h");
// revi.mode = "command";






// if revi.buffer_count() > 0 {
//   return;
// }
// let width = revi.width;
// let height = revi.height;
// let msg = "welcome to revi text editor";
// let buffer = new_buffer();
// for i in 0..height/2 {
//   buffer.insert(i, "\n");
// }
// let offset = ((width/2) - (msg.len()/2));
// for i in 0..offset {
//   buffer.insert(height/2, " ");
// }
// buffer.insert(height/2 + offset, msg);
// buffer.read_only = true;
// let window = new_window(width, height, buffer);
// revi.create_window(window)
