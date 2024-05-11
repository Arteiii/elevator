use elevator_lib::run_elevated;
use std::io::Write;
use std::{env, io};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that at least one argument is provided (program path)
    if args.len() < 2 {
        println!("Usage: {} <program_path> [args...]", args[0]);

        exit();
    }

    // Extract the program path and arguments
    let program_path = &args[1];
    let program_args = if args.len() > 2 {
        args[2..].join(" ")
    } else {
        String::new()
    };

    // Require admin privileges
    if let Err(err) = run_elevated(program_path, program_args) {
        println!("Error: {}", err);
        exit();
    }
}

fn exit() {
    let _ = io::stdout().flush();
    println!("Press Enter to exit...");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input); // Wait for Enter key press
    std::process::exit(1);
}
