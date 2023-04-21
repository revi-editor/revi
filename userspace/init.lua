-- settings:set_line_number(settings.LineNumberKindBoth)
settings:set_line_number("both")

local buffer_count = revi.buffers
local width = revi.width
local height = revi.height
if #buffer_count == 0 then
  local welcome_msg = "welcome to revi text editor"
  local w = (width / 2) - (welcome_msg:len() / 2)
  local msg = string.format("%".. w .. "s%s", " ", welcome_msg)

  local buffer = init_buffer()
  for line=0, height/2 do
    buffer:insert(line, "\n")
  end
  local offset = height/2;
  for idx=1,msg:len() do
    buffer:insert(offset + idx - 1, msg:sub(idx, idx))
  end
  -- buffer:insert_center("welcome to revi text editor")
  revi:create_window({
    width = width,
    height = height,
    buffer = buffer,
  })
end
