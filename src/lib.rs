use std::{fs, io};
use std::process::Command;
use serde::Deserialize;
use toml::de;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct App {
    pub prog: String,
    pub cmd: String,
    pub args: String,
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

pub fn spawn_apps() -> Result<(), Box<dyn Error>>{
    let config_file = load_config_toml_file()?;
    let config = read_toml_to_config(config_file)?;

    for app in config.app {
        Command::new(app.cmd).args([app.args, app.prog]).spawn()?;
    }

    Ok(())
}