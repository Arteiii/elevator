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

use std::ffi::OsStr;
use std::io;
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
    // Set the effective user ID to root
    unsafe {
        if libc::setuid(0) != 0 {
            return Err(io::Error::last_os_error());
        }

        // Execute the program with elevated privileges
        Command::new(program_path)
            .args(args.iter().map(|arg| OsStr::new(*arg)))
            .spawn()?;
    }

    Ok(())
}
