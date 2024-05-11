use elevator_lib::run_elevated;

// Example: Run a program with elevated privileges
fn main() {
    // Run a program with elevated privileges
    if let Err(err) = run_elevated(r#"C:\Windows\System32\notepad.exe"#, r#"C:\test.txt"#) {
        eprintln!("Error: {}", err);
    }
}
