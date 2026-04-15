//! Configuration loader for Scissor

use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub map_file: String,
    pub world_name: String,
    // Add more config fields as needed
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
