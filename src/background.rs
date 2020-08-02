/*
Copyright 2020 CoolDeveloper101

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/


use std::fmt;

/// An enum to manipulate background colors.
/// # Example
/// ```
/// # use colory::{Reset, BackgroundColor};
/// #
/// # fn main() {
/// println!("{}The background should be blue.{}", BackgroundColor::Red, Reset);
/// # }
/// ```
/// See the [examples](https://github.com/CoolDeveloper101/colory/tree/master/examples) directory for more examples.
pub enum BackgroundColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
  White,
  /// Somethimes you want more than 8 colors. Colory also has options to choose from a 8 bit color pallete which contains 256 colors.
  /// You can check out the color pallete at [Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
  EightBit(u8),
  /// Heck! You want even more colors. Colory also has an RGB pallete which allows you to have 16777216 colors!
  RGB(u8, u8, u8),
  /// Reset the background color to whatever the default background color is.
	Normal,
}

impl fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self{
        BackgroundColor::Black => write!(f, "\x1b[40m"),
        BackgroundColor::Red => write!(f, "\x1b[41m"),
        BackgroundColor::Green => write!(f, "\x1b[42m"),
        BackgroundColor::Yellow => write!(f, "\x1b[43m"),
        BackgroundColor::Blue => write!(f, "\x1b[44m"),
        BackgroundColor::Magenta => write!(f, "\x1b[45m"),
        BackgroundColor::Cyan => write!(f, "\x1b[46m"),
        BackgroundColor::White => write!(f, "\x1b[47m"),
        BackgroundColor::EightBit(c) => write!(f, "\x1b[48;5;{}m", c),
        BackgroundColor::RGB(r, g, b) => write!(f, "\x1b[48;2;{};{};{}m", r, g, b),
        BackgroundColor::Normal => write!(f, "\x1b[49m"),
      }
    }
}