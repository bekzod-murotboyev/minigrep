use std::process;

fn main() {
    if let Err(e) = minigrep::run() {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
