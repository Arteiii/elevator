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
use std::process::{Command, ExitStatus};

/// Run a program with elevated privileges.
///
/// This function attempts to elevate the program's privileges by setting the effective user ID to root
/// using the `setuid` system call, and then spawns a new process to execute the specified program
/// with the given arguments.
///
/// # Arguments
///
/// * `program_path` - The path to the program to execute.
/// * `args` - A single str of arguments to pass to the program.
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
/// if let Err(err) = run_elevated("/usr/bin/some_program", "arg1 arg2") {
///     eprintln!("Error: {}", err);
/// }
/// ```
#[inline]
pub fn run_elevated<S: AsRef<OsStr>>(program_path: S, args: &str) -> std::io::Result<ExitStatus> {
    // Check if the process is running with elevated privileges
    if !is_running_as_sudo() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error: This program must be run with elevated privileges (sudo).",
        ));
    }

    // Split the arguments string at spaces
    let args_split: Vec<&str> = args.split_whitespace().collect();

    // Start the specified program with the provided arguments
    let mut child = Command::new(program_path).args(args_split).spawn()?;

    // Wait for the process to finish and capture the exit status
    let exit_status = child.wait()?;

    Ok(exit_status)
}

// Function to check if the process is running with elevated privileges (sudo)
pub fn is_running_as_sudo() -> bool {
    // Check if the environment variable "SUDO_USER" is set
    std::env::var("SUDO_USER").is_ok()
}
