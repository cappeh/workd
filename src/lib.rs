use std::{fs, io};
use serde::Deserialize;
use toml::de::Error;

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

pub fn read_toml_to_config(file: String) -> Result<Config, Error> {
   let config = toml::from_str(&file)?;
   Ok(config)
}