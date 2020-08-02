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


/// An enum to manipulate foreground colors.
/// # Examples
/// ```
/// # use colory::{Reset, BackgroundColor};
/// #
/// # fn main() {
/// println!("{}This is red.{}", ForegroundColor::Red, Reset);
/// # }
/// ```
pub enum ForegroundColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	EightBit(u8),
	RGB(u8, u8, u8),
	Normal,
}

impl fmt::Display for ForegroundColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self{
        ForegroundColor::Black => write!(f, "\x1b[30m"),
        ForegroundColor::Red => write!(f, "\x1b[31m"),
        ForegroundColor::Green => write!(f, "\x1b[32m"),
        ForegroundColor::Yellow => write!(f, "\x1b[33m"),
        ForegroundColor::Blue => write!(f, "\x1b[34m"),
        ForegroundColor::Magenta => write!(f, "\x1b[35m"),
        ForegroundColor::Cyan => write!(f, "\x1b[36m"),
        ForegroundColor::White => write!(f, "\x1b[37m"),
        ForegroundColor::EightBit(c) => write!(f, "\x1b[38;5;{}m", c),
        ForegroundColor::RGB(r, g, b) => write!(f, "\x1b[38;2;{};{};{}m", r, g, b),
        ForegroundColor::Normal => write!(f, "\x1b[39m"),
      }
    }
}