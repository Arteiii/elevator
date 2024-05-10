# elevator

This library provides functions to run programs with elevated privileges on both Windows and Unix-like systems.

## Usage

To use this library in your Rust project, add it to your Cargo.toml:

```shell
cargo add elevator_lib
```

Then you can use it in your Rust code:

```shell
use elevator_lib::run_elevated;

// Example: Run a program with elevated privileges
// Note: The usage for both Unix and Windows is similar
// For Unix-like systems, the library will use the `unix` module,
// while for Windows, it will use the `windows` module
fn main() {
    if let Err(err) = run_elevated("C:\\Windows\\System32\\notepad.exe", &["C:\\example.txt"]) {
      eprintln!("Error: {}", err);
    }
}
```

## Command-line Tool

Install it using Cargo:

```shell
cargo install elevator
```

After installation, you can use it like this:

```shell
elevator-cli <program_path> [args...]
```

For example:

```shell
elevator-cli "/usr/bin/some_program" arg1 arg2
```

## License

This project is licensed under the [MIT](https://opensource.org/license/MIT) License -
see the [LICENSE](LICENSE-MIT) file for details.
