//! Module for running programs with elevated privileges on Unix-like systems.
//!
//! # Examples
//!
//! ```
//! use elevator_lib::run_elevated;
//!
//! // Run a program with elevated privileges
//! if let Err(err) = run_elevated("/usr/bin/some_program", &["arg1", "arg2"]) {
//!     eprintln!("Error: {}", err);
//! }
//! ```

use std::ffi::CString;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::process::Command;

/// Run a program with elevated privileges.
///
/// This function attempts to elevate the program's privileges by setting the effective user ID to root
/// using the `setuid` system call, and then spawns a new process to execute the specified program
/// with the given arguments.
///
/// # Arguments
///
/// * `program_path` - The path to the program to execute.
/// * `args` - A slice of arguments to pass to the program.
///
/// # Errors
///
/// Returns an `io::Error` if setting the effective user ID to root fails, or if spawning the process fails.
///
/// # Examples
///
/// ```
/// use elevator_lib::run_elevated;
///
/// // Run a program with elevated privileges
/// if let Err(err) = run_elevated("/usr/bin/some_program", &["arg1", "arg2"]) {
///     eprintln!("Error: {}", err);
/// }
/// ```
pub fn run_elevated(program_path: &str, args: &[&str]) -> io::Result<()> {
    // Convert program path and arguments to CString
    let program_cstr = CString::new(program_path)?;
    let args_cstr: Vec<CString> = args.iter().map(|arg| CString::new(*arg).unwrap()).collect();

    // Set the effective user ID to root
    unsafe {
        if libc::setuid(0) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Execute the program with elevated privileges
        Command::new(program_cstr.as_ptr())
            .args(
                args_cstr
                    .iter()
                    .map(|arg| arg.as_ptr())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .spawn()?;
    }

    Ok(())
}
