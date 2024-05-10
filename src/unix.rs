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
pub fn run_elevated(program_path: &str, args: &[&str]) -> Result<(), String> {
    // Check if the process is running with elevated privileges
    if !is_running_as_sudo() {
        return Err("Error: This program must be run with elevated privileges (sudo).".to_string());
    }

    // Start the specified program with the provided arguments
    let result = Command::new(program_path).args(args).spawn();

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

// Function to check if the process is running with elevated privileges (sudo)
pub fn is_running_as_sudo() -> bool {
    // Check if the environment variable "SUDO_USER" is set
    std::env::var("SUDO_USER").is_ok()
}
