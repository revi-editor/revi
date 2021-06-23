# v0.0.0-beta-0.5

### Enhancements Add

- Added Keybinding 'A' Move cursor to end of line and place editor in insert mode.
- Added Keybinding '$' to move cursor to end of line.
- Added Keybinding '<C-d>' to scroll down by one line.
- Added Keybinding '<C-u>' to scroll up by one line.
- Added Keybinding '<C-y> to scroll up by one line and maintain cursor line.
- Added Keybinding '<C-e> to scroll down by one line and maintain cursor line.

### Bug Fix

- `#22` Some times cursor would go out of text bounds
- `#23` Backspace up a line would not remove line number
- General Code Clean up