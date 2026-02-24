use std::process::Command;

fn main() {
    Command::new("open").arg("-a").arg("Wezterm").spawn().unwrap();
}
