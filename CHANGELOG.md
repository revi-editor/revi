# Changelog

## ------------- **Version 0.0.0-beta-1 (Jul 2021)** -------------------------------------------

### Added

  - [Multiple Windows](46)
  - [Multiple Buffers so more then one file can be worked on.](18)
  - [Added a basic Command Mode](7)
  - [updated mlua crate from `1.2` to `1.3.1`](b337e8846e5fb1e74ac668c21af6f90b42a732fa)
  - [keybinding `gg` to jump to top of file](38)
  - [keybinding `G` to jump to bottom of file](38)
- [Added a Test for ever Method/Function in ReVi](35)

### Fixed
  - Cursor moving out of max file's line count.

## ------------- **Version 0.0.0-beta-0.6 (Jun 28 2021)** -------------------------------------------
  Mon 28 Jun 2021 06:42:48 PM CDT


  In these update brought a lot of structural changes to ReVi in the regards to breaking up some
  of it into there own packages.  Now ReVi has [revi-core]() and [revi-ui]() so that we can
  abstract away the drawing of the screen and the core part of this program. In doing so I do not
  see way we could not support a GUI state as well.  I just wanted to say that making the `w` and `b`
  commands was extremely harder then was expected.  The implementation of it is certainly not the best
  but it gets the job done for now.  Getting this working and worrying about how it is implemented on
  the first go around is ok.  Its helped me see how to make such an algorithm for this and things I
  can do to make it better.

### Added

  - [updated ropey crate from `1.2` to `1.3.1`](b337e8846e5fb1e74ac668c21af6f90b42a732fa)
  - [keybinding `o` to insert new line below cursor](33)
  - [keybinding `O` to insert new line above cursor](33)
  - [keybinding `^` place cursor at the first char on current line](27)
  - [keybinding `I` place cursor at the first char in line and place into insert mode](28)
  - [keybinding `b` to move backwards by word](10)
  - [keybinding `w` to move forward by word](11)
- [keybinding `Enter` in command mode for exiting it for now]()

### Fixed

  - [Fixed behaver of 'dd' command add CursorUp command]()
- [Fixed Crash when no local init.lua file is found](36)

## ------------- **Version 0.0.0-beta-0.5 (Jun 22 2021)** -------------------------------------------

### Added

  - [Added Keybinding `A` Move cursor to end of line and place editor in insert mode](14)
  - [Added Keybinding `$` to move cursor to end of line](26)
  - [Added Keybinding `<C-d>` to scroll down by one line](31)
  - [Added Keybinding `<C-u>` to scroll up by one line](31)
  - [Added Keybinding `<C-y>` to scroll up by one line and maintain cursor line](31)
- [Added Keybinding `<C-e>` to scroll down by one line and maintain cursor line](31)

### Fixed

  - [Some times cursor would go out of text bounds](22)
  - [Backspace up a line would not remove line number](23)
- [Something I did with Fix #23 made it possible to go to far right letting you back space just the new line.](24)
  - [When at bottom of screen pressing enter in insert mode does not scroll or move cursor down after inserting '\n'](30)
  - [Line number does not update correctly when scrolling off screen then backspacing back up.](31)
- [General Code Clean up](29)
