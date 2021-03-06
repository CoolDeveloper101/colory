# Colory

[![build](https://github.com/CoolDeveloper101/colory/workflows/build/badge.svg)](https://github.com/CoolDeveloper101/colory/actions/)

A simple library to make your command line application more colorful!


## Getting Started
To use colory in your project, add it to the dependencies section in you Cargo.toml

```toml
[dependencies]
colory = "0.5.0"
```
This will allow Cargo to download, build, and cache the colory as a package directly from crates.io.

```rust
use colory::{ForegroundColor as fg, colory_init};

fn main() {
    colory_init(); // required to call this on windows. It will do nothing on *nix systems.
    println!("{}Hello, world!{}", fg::Green, fg::Normal);
}
```
For more complex examples, see the [examples](https://github.com/CoolDeveloper101/colory/tree/master/examples) directory.


## Platform Support
Here is a list of prominent terminal emulators that support ansi escape codes(this list is not complete, contribute by opening an issue or a pull request) -


**Unix Like**
* Linux console
* GNU Screen
* Minicorn
* tmux

**macOs**
* Terminal
* iTerm2
* xterm

**Microsoft Windows**
* AlphaCom
* ConEmu
* mintty
* PuTTy
* Windows Terminal
* Windows Console (aka cmd)
* Powershell

If you terminal emulator is not in the list, download this library and run some of the examples.
If they do not work, then your terminal probably does not support it.

**Note**: This library may not work with terminals that do not support ansi escape codes.