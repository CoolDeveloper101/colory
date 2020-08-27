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

mod foreground;
pub use foreground::ForegroundColor;

mod background;
pub use background::BackgroundColor;

mod styles;
pub use styles::Styles;
pub use styles::Styles::Off;

#[deprecated(since = "4.0", note = "Use Styles::Off instead.")]
struct Reset;

impl fmt::Display for Reset{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[0m")
    }
}