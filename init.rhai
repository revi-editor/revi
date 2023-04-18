if revi.buffer_count() == 0 {
  let width = revi.width;
  let height = revi.height;
  let msg = "welcome to revi text editor\nnewline";
  let buffer = new_buffer();
  for i in 0..height/2 {
    buffer.insert(i, "\n");
  }
  let offset = ((width/2) - (msg.len()/2));
  for i in 0..offset {
    buffer.insert(height/2, " ");
  }
  buffer.insert(height/2 + offset, msg);
  let window = new_window(width, height, buffer);
  revi.create_window(window)
}
