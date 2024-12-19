use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";
const MODEL_PLUGIN_DIRECTORY: &str = "model_plugins";
const TOOL_PLUGIN_DIRECTORY: &str = "tool_plugins";

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

#[derive(Clone, Copy)]
pub enum PluginType {
    Tool,
    Model,
}

pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub location: PathBuf,
}

impl Config {
    pub fn empty() -> Config {
        Config {
            default_model: Option::default(),
            models: Option::default(),
        }
    }

    pub fn from_directory(path: PathBuf) -> Result<Option<Config>, Box<dyn std::error::Error>> {
        let file_exists = Config::exists(path.clone())?;
        if file_exists {
            let contents = fs::File::open(path.join(CONFIG_FILE_NAME))?;
            let config: Config = serde_yml::from_reader(contents)?;

            return Ok(Some(config));
        }

        Ok(Option::default())
    }

    pub fn to_directory(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&path)?;
        let contents = serde_yml::to_string(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.join(CONFIG_FILE_NAME))?;

        file.write(contents.as_bytes())?;

        Ok(())
    }

    pub fn exists(path: PathBuf) -> std::io::Result<bool> {
        fs::exists(path.join(CONFIG_FILE_NAME))
    }

    pub fn copy_plugin_to_directory(prompt_home: PathBuf, plugin: Plugin) -> std::io::Result<()> {
        let sub_directory = match plugin.plugin_type {
            PluginType::Tool => TOOL_PLUGIN_DIRECTORY,
            PluginType::Model => MODEL_PLUGIN_DIRECTORY,
        };

        fs::create_dir_all(prompt_home.join(sub_directory))?;

        fs::copy(
            plugin.location,
            prompt_home
                .join(sub_directory)
                .join(plugin.name)
                .join(".js"),
        )?;

        Ok(())
    }

    pub fn list_plugins(
        prompt_home: PathBuf,
        plugin_type: PluginType,
    ) -> Result<Vec<Plugin>, Box<dyn std::error::Error>> {
        let sub_directory = match plugin_type {
            PluginType::Tool => TOOL_PLUGIN_DIRECTORY,
            PluginType::Model => MODEL_PLUGIN_DIRECTORY,
        };

        let plugin_path_exists = fs::exists(prompt_home.join(sub_directory))?;
        if !plugin_path_exists {
            return Ok(Vec::new());
        }

        let mut plugins: Vec<Plugin> = Vec::new();
        let plugin_files = fs::read_dir(prompt_home.join(sub_directory))?;
        for plugin_file in plugin_files {
            if let Ok(file) = plugin_file {
                if file.path().is_file() {
                    let path = PathBuf::from(file.path());
                    let name = path
                        .file_stem()
                        .ok_or(format!(
                            "Could not get plugin name for path: {}",
                            path.display()
                        ))?
                        .to_str()
                        .ok_or("Could not map plugin name to string.".to_string())?;

                    plugins.push(Plugin {
                        name: name.to_string(),
                        plugin_type,
                        location: path,
                    });
                }
            }
        }

        Ok(plugins)
    }
}
