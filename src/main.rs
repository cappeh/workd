use std::fs;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct App {
    prog: String,
    cmd: String,
    args: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    app: Vec<App>,
}
fn main() {
    let config_file = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();
    for app in config.app {
        Command::new(app.cmd).args([app.args, app.prog]).spawn().unwrap();
    }
}
