# Colory
A simple library to make your command line application more colorful!


## Warning!
This library may not work with terminals that do not support ansi escape codes.

## Getting Started
To use colory in your project, add it to the dependencies section in you Cargo.toml

```toml
[dependencies]
colory = "0.2.2"
```
This will allow Cargo to download, build, and cache the colory support as a package directly from crates.io.

```rust
use colory::ForegroundColor as fg;

fn main() {
    println!("{}Hello, world!{}", fg::Green, fg::Normal);
}
```
For more complex examples, see the [examples](https://github.com/CoolDeveloper101/colory/tree/master/examples) directory.
