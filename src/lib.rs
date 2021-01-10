/*
Copyright 2021 CoolDeveloper101

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

#[deprecated(since = "4.0.0", note = "Use Styles::Off instead.")]
struct Reset;

impl fmt::Display for Reset{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[0m")
    }
}


/// The initializer. It is necessary to call this on windows
/// so that ansi escape codes work correctly.
/// # Example
/// ```rust
/// # use colory::{Off, ForegroundColor, colory_init};
/// #
/// # fn main() {
/// colory_init().unwrap(); // Should be called on windows. 
/// println!("{}This is red.{}", ForegroundColor::Red, Off);
/// # }
/// ```
/// **Note**: The doctests currently fail due to this as colory is not able to get the current console mode.
/// It is advisable not to use the colory_init() function inside tests as it may cause the test to fail.
pub fn colory_init() -> Result<(), &'static str> {

    #[cfg(windows)]
    {
        use std::os::raw::{c_int, c_ulong, c_void};

        type HANDLE = *mut c_void;
        type DWORD = c_ulong;
        type BOOL = c_int;
        type LPDWORD = *mut DWORD;

        const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
        const TRUE: BOOL = 1;
        const STD_OUTPUT_HANDLE: DWORD = -11i32 as u32;
        const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

        extern "system" {
            fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
            fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
            fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
        }

        
        unsafe {
            let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
            if h_out == INVALID_HANDLE_VALUE {
                return Err("Invalid handle value!");
            }

            let mut dw_original_out_mode = 0;
            if GetConsoleMode(h_out, &mut dw_original_out_mode) != TRUE {
                return Err("Failed to get current console mode.");
            }

            let dw_requested_out_modes = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            let dw_out_mode = dw_original_out_mode | dw_requested_out_modes;
            if SetConsoleMode(h_out, dw_out_mode) != TRUE {
                // we failed to set both modes, try to step down mode gracefully.
                return Err("Failed to enable virtual terminal processing.");
            }
        }
    }

    Ok(())
}