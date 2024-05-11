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
use std::os::windows::process::ExitStatusExt;
use std::process::ExitStatus;

use windows::core::{w, HSTRING, PCWSTR};
use windows::Win32::System::Threading::{GetExitCodeProcess, WaitForSingleObject, INFINITE};
use windows::Win32::UI::Shell::{
    ShellExecuteExW, SEE_MASK_NOASYNC, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW,
};

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
/// Returns an `std::io::Result`
/// containing the exit status of the process if successful, or an `std::io::Error`
/// if running the program with elevated privileges fails.
///
/// # Examples
///
/// ```
/// use elevator_lib::run_elevated;
///
/// // Run a program with elevated privileges
/// if let Err(err) = run_elevated(r#"C:\Windows\System32\notepad.exe"#, r#"C:\example.txt"#) {
///     eprintln!("Error: {}", err);
/// }
/// ```
///
#[inline]
pub fn run_elevated<S: AsRef<OsStr>, T: AsRef<OsStr>>(
    program_path: S,
    args: T,
) -> std::io::Result<ExitStatus> {
    let mut code = 1;
    let file = HSTRING::from(program_path.as_ref());
    let par = HSTRING::from(args.as_ref());

    let mut sei = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOASYNC | SEE_MASK_NOCLOSEPROCESS,
        lpVerb: w!("runas"),
        lpFile: PCWSTR(file.as_ptr()),
        lpParameters: PCWSTR(par.as_ptr()),
        nShow: 1,
        ..Default::default()
    };
    unsafe {
        ShellExecuteExW(&mut sei)?;
        let process = { sei.hProcess };
        if process.is_invalid() {
            return Err(std::io::Error::last_os_error());
        };
        WaitForSingleObject(process, INFINITE);
        GetExitCodeProcess(process, &mut code)?;
    };
    Ok(ExitStatus::from_raw(code))
}
