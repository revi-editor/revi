//
// pub trait Model {
//     fn set_mode<T>(&mut self, mode: T)
//         where T: std::fmt::Debug + Copy + Clone;
// }
//
// pub trait Window {
//     type Contents;
//     fn get_contents(&self) -> Self::Contents;
//     fn is_read_only(&self) -> bool;
// }
//
// pub struct CommandBar(pub String);
// impl Cursor for CommandBar {
//     fn move_cursor_up(&mut self) {
//
//     }
//
//     fn move_cursor_down(&mut self) {
//
//     }
//
//     fn move_cursor_left(&mut self) {
//
//     }
//
//     fn move_cursor_right(&mut self) {
//
//     }
// }
// impl Window for CommandBar {
//     type Contents = String;
//     fn get_contents(&self) -> Self::Contents {
//         self.0.clone()
//     }
//     fn is_read_only(&self) -> bool {
//         false
//     }
// }
