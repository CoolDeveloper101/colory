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


//! `colory` is a utility to make your command line output more colorful!
//! See the [getting started](https://github.com/CoolDeveloper101/colory#getting-started) for more details.

use std::fmt;


/// Sometimes you want to turn off all properties and switching to default for every property is quite unreadable.
/// By using this struct, you can turn off all ansi customizations.
/// # Example
/// ```
/// # use colory::{ForegroundColor as fg, BackgroundColor as bg, Reset};
/// #
/// # fn main() {
/// // Instead of doing this
/// println!("{}{}This is green text with a blue background.{}{} This is plain text.", bg::Blue, fg::Green, bg::Normal, fg::Normal);
/// // You can do
/// println!("{}{}This is green text with a blue background.{} This is plain text.", bg::Blue, fg::Green, Reset);
/// # }
/// ```
pub struct Reset;
impl fmt::Display for Reset {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[0m")
	}
}

mod foreground;
pub use foreground::ForegroundColor;

mod background;
pub use background::BackgroundColor;

mod styles;
pub use styles::Styles;
