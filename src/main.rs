use av_exclusion_lib::*;

fn main() {
    // Require admin privileges
    if let Err(err) = require_admin_privileges() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
