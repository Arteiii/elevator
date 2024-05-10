//! A cross-platform Rust library for running programs with elevated privileges.
//!
//! This library provides functions to run programs with elevated privileges on both Windows and Unix-like systems.
//!
//! # Examples
//!
//! ## Windows
//!
//! ```
//! use elevator_lib::run_elevated;
//!
//! // Run a program with elevated privileges on Windows
//! if let Err(err) = run_elevated("C:\\Windows\\System32\\notepad.exe", &["C:\\example.txt"]) {
//!     eprintln!("Error: {}", err);
//! }
//! ```
//!
//! ## Unix-like
//!
//! ```
//! use elevator_lib::run_elevated;
//!
//! // Run a program with elevated privileges on Unix-like systems
//! if let Err(err) = run_elevated("/usr/bin/some_program", &["arg1", "arg2"]) {
//!     eprintln!("Error: {}", err);
//! }
//! ```

// windows:
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

// unix:
#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;
