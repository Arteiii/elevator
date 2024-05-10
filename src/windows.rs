
//! Module for running programs with elevated privileges on Windows systems.
//!
//! # Examples
//!
//! ```
//! use elevator_lib::run_elevated;
//!
//! // Run a program with elevated privileges
//! if let Err(err) = run_elevated("C:\\Windows\\System32\\notepad.exe", &["C:\\example.txt"]) {
//!     eprintln!("Error: {}", err);
//! }
//! ```
//! 
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use winapi::shared::minwindef::{DWORD, HINSTANCE};
use winapi::um::shellapi::{ShellExecuteW, SEE_MASK_NOASYNC};
use winapi::um::winuser::SW_SHOWNORMAL;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::FormatMessageW;

/// Run a program with elevated privileges on Windows.
///
/// This function attempts to run the specified program with elevated privileges using the `ShellExecuteW`
/// function from the Windows API with the "runas" verb.
///
/// # Arguments
///
/// * `program_path` - The path to the program to execute.
/// * `args` - A slice of arguments to pass to the program.
///
/// # Errors
///
/// Returns a `String` with an error message if running the program with elevated privileges fails.
///
/// # Examples
///
/// ```
/// use elevator_lib::run_elevated;
///
/// // Run a program with elevated privileges
/// if let Err(err) = run_elevated("C:\\Windows\\System32\\notepad.exe", &["C:\\example.txt"]) {
///     eprintln!("Error: {}", err);
/// }
/// ```
pub fn run_elevated(program_path: &str, args: &[&str]) -> Result<(), String> {
    unsafe {
        let verb = OsStr::new("runas")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let program = OsStr::new(program_path)
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let parameters = args
            .iter()
            .flat_map(|arg| {
                let arg_wide: Vec<u16> = OsStr::new(arg).encode_wide().chain(Some(0)).collect();
                arg_wide
            })
            .collect::<Vec<_>>();

        let result = ShellExecuteW(
            ptr::null_mut(),
            verb.as_ptr(),
            program.as_ptr(),
            parameters.as_ptr() as *const _,
            ptr::null_mut(),
            SW_SHOWNORMAL | SEE_MASK_NOASYNC as i32,
        );

        if result > 32 as HINSTANCE {
            Ok(())
        } else {
            let error_code = GetLastError();
            let mut buffer = [0; 256];
            let len = FormatMessageW(
                winapi::um::winbase::FORMAT_MESSAGE_FROM_SYSTEM,
                ptr::null_mut(),
                error_code,
                0,
                buffer.as_mut_ptr(),
                buffer.len() as DWORD,
                ptr::null_mut(),
            );

            if len != 0 {
                let message = String::from_utf16_lossy(&buffer[..len as usize]);
                Err(format!("Failed to run {} as administrator: {}", program_path, message))
            } else {
                Err(format!("Failed to run {} as administrator.", program_path))
            }
        }
    }
}
