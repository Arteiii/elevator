//! A cross-platform Rust library for running programs with elevated privileges.
//!
//! This library provides functions to run programs with elevated privileges on Windows systems.
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

// windows:
#[cfg(windows)]
#[allow(internal_features)]
mod windows;

#[cfg(windows)]
pub use windows::*;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;
