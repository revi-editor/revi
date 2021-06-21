<h1 align="center"> ReVi </h1>
<p align="center">
  <a><img alt="MAINTAINED" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg"></a>
  <a><img alt="Downloads" src="https://img.shields.io/crates/d/revi"></a>
  <a href="https://crates.io/crates/revi"><img alt="crates.io" src="https://img.shields.io/crates/v/revi.svg"></a>
  <a><img alt="lastupdated" src="https://img.shields.io/github/last-commit/revi-editor/revi?style=plastic"></a>
  <a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/revi-editor/revi?style=plastic"></a>
  <a><img alt="issuse" src="https://img.shields.io/github/issues/revi-editor/revi?style=plastic"></a>
  <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
  <a><img alt="RUST" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a><img alt="LUA" src="https://img.shields.io/badge/Lua-2C2D72?style=for-the-badge&logo=lua&logoColor=white"></a>
</p>

ReVi is a cross-platform terminal based Vim inspired text editor.
Currently ReVi is in heavy development and is not really at a point for
every day usage.  If you find a bug please feel free to open a issues for it.

# Usage

**Crates.io**
```sh
cargo install revi --version="0.0.0-beta-0.4"
revi <filename>
```
**GitHub**
```sh
git clone https://github.com/revi-editor/revi
cd revi
cargo install --path .
revi <filename>
```

# Development Use
```sh
git clone https://github.com/revi-editor/revi
cd revi
cargo run --release -- <filename>
```

# Road Map

- [ ] **Plugin API** `0.1%`:
  - [ ] **Custom KeyBindings**
- [ ] **WebSite** `0%`
- [ ] **Package Manager** `0%`
- [ ] **LSP** `0%`
