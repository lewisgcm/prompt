use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(rename = "settings")]
    pub settings: Option<HashMap<String, String>>,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "plugins")]
    pub plugins: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "default-model")]
    pub default_model: Option<String>,

    #[serde(rename = "models")]
    pub models: Option<HashMap<String, ModelConfig>>,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::File::open(filename)?;
    let config: Config = serde_yml::from_reader(contents)?;

    Ok(config)
}
