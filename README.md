<h1 align="center"> ReVi </h1>
<p align="center">
  <a><img alt="MAINTAINED" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg"></a>
  <a><img alt="Downloads" src="https://img.shields.io/crates/d/revi"></a>
  <a href="https://crates.io/crates/revi"><img alt="crates.io" src="https://img.shields.io/crates/v/revi.svg"></a>
  <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>
<p align="center">
  <a><img alt="issues" src="https://img.shields.io/github/issues/revi-editor/revi"></a>
  <a><img alt="last commit" src="https://img.shields.io/github/last-commit/revi-editor/revi"></a>
  <a><img alt="repo size" src="https://img.shields.io/github/repo-size/revi-editor/revi"></a>
  <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>
<p align="center">
  <a><img alt="RUST" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a><img alt="LUA" src="https://img.shields.io/badge/Lua-2C2D72?style=for-the-badge&logo=lua&logoColor=white"></a>
</p>

# Table Of Contents:

  - [**About**](#about)
  - [**Usage**](#usage)
    - [**Install**](###crates.io)
    - [**Usage**](###github)
    - [**Usage**](###development-use)
  - [**KeyBindings**](#keybindings)
  - [**Road Map**](#road-map)

# About

> ReVi is a cross-platform terminal based Vim inspired text editor.
Currently ReVi is in heavy development and is not really at a point for
every day usage.  If you find a bug please feel free to open a issues for it.

<p align="center">
  <a><img alt="Image" src="./snapshots/line_numbers.png"></a>
</p>


# Usage

### **Crates.io**
```sh
cargo install revi --version="0.0.0-beta-0.4"
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

# KeyBindings

**Mode**|'Key'|*Command*
:---|:---:|---:
**Normal**|`Esc`|*Normal*
**Normal**|`ZZ`|*Save Quit*
**Normal**|`ZQ`|*Quit*
**Normal**|`j`|*CursorDown*
**Normal**|`Down`|*CursorDown*
**Normal**|`k`|*CursorUp*
**Normal**|`Up`|*CursorUp*
**Normal**|`h`|*CursorLeft*
**Normal**|`Left`|*CursorLeft*
**Normal**|`l`|*CursorRight*
**Normal**|`Right`|*CursorRight*
**Normal**|`:`|*Command*
**Normal**|`i`|*Insert*
**Normal**|`x`|*DeleteChar*
**Normal**|`Delete`|*DeleteChar*
**Normal**|`d`|*DeleteLine*
**Normal**|`Home`|*Home*
**Normal**|`End`|*End*
**Normal**|`0`|*Home*
**Insert**|`Esc`|*Normal*
**Insert**|`Backspace`|*Backspace*
**Insert**|`Enter`|*NewLine*
**Insert**|`Home`|*Home*
**Insert**|`End`|*End*
**Insert**|`Down`|*CursorDown*
**Insert**|`Up`|*CursorUp*
**Insert**|`Left`|*CursorLeft*
**Insert**|`Right`|*CursorRight*



# Road Map

- [X] **Basic KeyBindings**
- [ ] **Plugin API** `0.1%`:
  - [ ] **Custom KeyBindings**
- [ ] **Help Docs** `0%`
- [ ] **WebSite** `0%`
- [ ] **Package Manager** `0%`
- [ ] **LSP** `0%`
