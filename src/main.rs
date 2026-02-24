use std::process;
use workd::spawn_apps;

fn main() {
    if let Err(e) = spawn_apps() {
        eprintln!("Error starting applications: {e}");
        process::exit(1);
    }
}
