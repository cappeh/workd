use serde::Deserialize;
use std::error::Error;
use std::process::Command;
use std::{fs, io};
use std::path::PathBuf;
use toml::de;

#[derive(Debug, Deserialize)]
pub struct App {
    pub prog: PathBuf,
    pub cmd: PathBuf,
    pub args: String,
    pub workspace: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: Vec<App>,
}

pub fn load_config_toml_file() -> Result<String, io::Error> {
    fs::read_to_string("config.toml")
}

pub fn read_toml_to_config(file: String) -> Result<Config, de::Error> {
    let config = toml::from_str(&file)?;
    Ok(config)
}

pub fn spawn_apps() -> Result<(), Box<dyn Error>> {
    let config_file = load_config_toml_file()?;
    let config = read_toml_to_config(config_file)?;

    for app in config.app {
        move_to_workspace(&app)?;
        Command::new(&app.cmd)
            .arg(&app.args)
            .arg(&app.prog)
            .spawn()?;
    }

    Ok(())
}

pub fn move_to_workspace(app: &App) -> Result<(), Box<dyn Error>> {
    // move to workspace
    Command::new("aerospace")
        .args(["workspace", &app.workspace])
        .spawn()?;
    Ok(())
}
