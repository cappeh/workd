use std::process::Command;

fn main() {
    let program = "Wezterm";
    Command::new("open").args(["-a", program]).spawn().unwrap();
}
