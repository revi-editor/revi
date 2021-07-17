<h1 align="center"> ReVi </h1>
<p align="center">
<a><img alt="MAINTAINED" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg"></a>
<a><img alt="Downloads" src="https://img.shields.io/crates/d/revi"></a>
<a href="https://crates.io/crates/revi"><img alt="crates.io" src="https://img.shields.io/crates/v/revi.svg"></a>
<a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>
<p align="center">
<a><img alt="Stars" src="https://img.shields.io/github/stars/revi-editor/revi?style=social"></a>
<a><img alt="Forks" src="https://img.shields.io/github/forks/revi-editor/revi?style=social"></a>
<a><img alt="watchers" src="https://img.shields.io/github/watchers/revi-editor/revi?style=social"></a>
<a><img alt="contributors" src="https://img.shields.io/github/contributors/revi-editor/revi"></a>
</p>
<p align="center">
<a><img alt="issues" src="https://img.shields.io/github/issues/revi-editor/revi"></a>
<a><img alt="last commit" src="https://img.shields.io/github/last-commit/revi-editor/revi"></a>
<a><img alt="repo size" src="https://img.shields.io/github/repo-size/revi-editor/revi"></a> <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
<a><img alt="lines" src="https://img.shields.io/tokei/lines/github/revi-editor/revi"></a>
</p>
<p align="center">
<a><img alt="RUST" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
<a><img alt="LUA" src="https://img.shields.io/badge/Lua-2C2D72?style=for-the-badge&logo=lua&logoColor=white"></a>
</p>

# Table Of Contents:

  - [**About**](#about)
  - [**Usage**](#usage)
  - [**Install**](#cratesio)
  - [**Clone && Installing**](#github)
  - [**Development**](#development-use)
  - [**Q&A**](#questions-and-answers)
  - [**KeyBindings**](#keybindings)
  - [**Roadmap**](#road-map)
- [**Changelog**](./CHANGELOG.md)

# About

  ReVi is a cross-platform terminal based Vim inspired text editor.
  Currently ReVi is in heavy development and it's probably not good idea to use for every day use
  but I have been using ReVi to work on ReVi to help find bugs. Editor inception 😲!
  If you like what you see help the project out with a [github](https://github.com/revi-editor/revi) star.
  If you find a bug please feel free to open a issues for it.

  <p align="center">
  <a><img alt="Image" src="./snapshots/line_numbers.png"></a>
  </p>


# Usage

### **Crates.io**
  ```sh
  cargo install revi --version="0.0.0-beta-1"
  revi <filename>
  ```
### **GitHub**
  ```sh
  git clone https://github.com/revi-editor/revi
  cd revi
  cargo install --path .
  revi <filename>
  ```

### **Development Use**
  ```sh
  git clone https://github.com/revi-editor/revi
  cd revi
  cargo run --release -- <filename>
  ```

### **Questions and Answers**

  - *ReVi is locked up* => Press `Esc` and then do command to clear command chase.  WIP.
  - *ReVi doesn't even open* => for version's `0.0.0-beta-0.5` and below need to have a `init.lua` file in the directory.  FIXED on version `0.0.0-beta-0.6`

# KeyBindings

### **Normal Mode**

  |'Key'|*Command*|Note
  |:---|:---:|---:
  `Esc`|*NormalMode*|
  `ZZ`|*Save Quit*|
  `ZQ`|*Quit*|
  `<C-y>`|*ScrollUp Cursor Keeps Line Number*|Not working 100% correct
  `<C-e>`|*ScrollDown Cursor Keeps Line Number*|Not working 100% correct
  `<C-u>`|*ScrollUp*|Not working 100% correct
  `<C-d>`|*ScrollDown*|Not working 100% correct
  `j`|*CursorDown*|
  `Down`|*CursorDown*|
  `k`|*CursorUp*|
  `Up`|*CursorUp*|
  `h`|*CursorLeft*|
  `Left`|*CursorLeft*|
  `l`|*CursorRight*|
  `Right`|*CursorRight*|
  `w`|*Move Forwards by a Word*|
  `b`|*Move Backwards by a Word*|
  `:`|*CommandMode*|
  `i`|*InsertMode*|
  `x`|*DeleteChar*|
  `Delete`|*DeleteChar*|
  `d`|*DeleteLine*|
  `Home`|*Home*|
  `End`|*End*|
  `0`|*Home*|
  `$`|*End*|
  `A`|*End InsertMode CursorLeft*|
  'gg'|*JumpToFirstLine*
  'G'|*JumpToLastLine*

### **Insert Mode**

  |'Key'|*Command*|Note
  |:---|:---:|---:
  `Esc`|*Normal*|
  `Backspace`|*Backspace*|
  `Enter`|*NewLine*|
  `Home`|*Home*|
  `End`|*End*|
  `Down`|*CursorDown*|
  `Up`|*CursorUp*|
  `Left`|*CursorLeft*|
  `Right`|*CursorRight*|

### **Command Mode**

  |'Key'|*Command*|Note
  |:---|:---:|---:
  `Esc`|*Normal*|
  `Enter`|*Normal*|

### **Commands**
  *All Commands will change in further versions*
  |'Command'|*Action*|Note
  |:---|:---:|---:
  `q`|*QUIT*|
  `quit`|*QUIT*|
  `exit`|*QUIT*|
  `b[buffer number]`|*Sets Buffer*|
  `set number`|*Sets line numbers to AbsoluteNumber*|
  `set relativenumber`|*Sets line numbers to RelativeNumber*|
  `set nonumber`|*Removes any line number type*|

# Road Map

  - [ ] **Added Modes**:
  - [X] **Normal**
  - [X] **Insert**
  - [ ] **Command**
  - [ ] **Visual**
  - [ ] **Visual Line**
  - [ ] **Visual Block**
  - [X] **Basic KeyBindings**
  - [ ] **Basic Unicode Support**
  - [ ] **Plugin API**:
  - [ ] **Custom KeyBindings**
  - [ ] **Help Docs**
  - [ ] **WebSite**
  - [ ] **Package Manager**
  - [ ] **LSP**
