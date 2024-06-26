# elevator

This library provides functions to run programs with elevated privileges on Windows systems.

## Usage

To use this library in your Rust project, add it to your Cargo.toml:

```shell
cargo add elevator
```

Then you can use it in your Rust code:

```rust
use elevator_lib::run_elevated;

// Example: Run a program with elevated privileges
fn main() {
    if let Err(err) = run_elevated(r#"C:\Windows\System32\notepad.exe"#, r#"C:\example.txt"#) {
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
elevator <program_path> [args...]
```

For example:

```shell
elevator "C:\\Windows\\System32\\notepad.exe" arg1 arg2
```

## License

This project is licensed under the [MIT](https://opensource.org/license/MIT) License -
see the [LICENSE](LICENSE-MIT) file for details.
