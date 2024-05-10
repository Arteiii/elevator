use elevator_lib::run_elevated;
use std::env;


fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that at least one argument is provided (program path)
    if args.len() < 2 {
        eprintln!("Usage: {} <program_path> [args...]", args[0]);
        std::process::exit(1);
    }

    // Extract the program path and arguments
    let program_path = &args[1];
    let program_args: Vec<&str> = args
        .get(2..)
        .map_or_else(Vec::new, |a| a.iter().map(|arg| arg.as_str()).collect());

    // Require admin privileges
    if let Err(err) = run_elevated(program_path, &program_args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}