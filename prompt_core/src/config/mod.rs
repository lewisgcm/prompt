use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";
const MODEL_PLUGIN_DIRECTORY: &str = "model_plugins";
const TOOL_PLUGIN_DIRECTORY: &str = "tool_plugins";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ModelConfigSettingType {
    String(String),
    Integer(i32),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(rename = "settings")]
    pub settings: Option<HashMap<String, Option<ModelConfigSettingType>>>,
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

impl Config {
    pub fn add_model(&mut self, model_name: String, model_config: ModelConfig) {
        match &mut self.models {
            Some(ref mut models) => {
                models.insert(model_name, model_config);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(model_name, model_config);
                self.models = Some(map);
            }
        }
    }
}

#[derive(Debug)]
pub struct PromptConfig {
    pub prompt_home: PathBuf,
    pub config: Config,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginType {
    Tool,
    Model,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub location: PathBuf,
}

impl Clone for Plugin {
    fn clone(&self) -> Plugin {
        Plugin {
            name: self.name.clone(),
            plugin_type: self.plugin_type.clone(),
            location: self.location.clone(),
        }
    }
}

impl Display for Plugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PromptConfig {
    pub fn from_prompt_home(path: PathBuf) -> Result<PromptConfig, Box<dyn std::error::Error>> {
        fs::create_dir_all(&path)?;
        fs::create_dir_all(path.join(TOOL_PLUGIN_DIRECTORY))?;
        fs::create_dir_all(path.join(MODEL_PLUGIN_DIRECTORY))?;

        let file_exists = fs::exists(path.join(CONFIG_FILE_NAME))?;
        if file_exists {
            let contents = fs::File::open(path.join(CONFIG_FILE_NAME))?;
            let config: Config = serde_yml::from_reader(contents)?;

            return Ok(PromptConfig {
                config,
                prompt_home: path,
            });
        }

        Ok(PromptConfig {
            prompt_home: path,
            config: Config {
                default_model: Option::default(),
                models: Option::default(),
            },
        })
    }

    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_yml::to_string(&self.config)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.prompt_home.join(CONFIG_FILE_NAME))?;

        file.write(contents.as_bytes())?;

        Ok(())
    }

    pub fn install_plugin(&self, plugin: Plugin) -> std::io::Result<()> {
        let sub_directory = match plugin.plugin_type {
            PluginType::Tool => TOOL_PLUGIN_DIRECTORY,
            PluginType::Model => MODEL_PLUGIN_DIRECTORY,
        };

        let install_location = self
            .prompt_home
            .join(sub_directory)
            .join(plugin.name.clone() + ".js");

        fs::copy(plugin.location, install_location.clone())?;

        println!(
            "Plugin '{}' installed to '{}'.",
            plugin.name.clone(),
            install_location.display()
        );

        Ok(())
    }

    pub fn list_plugins(
        &self,
        plugin_type: PluginType,
    ) -> Result<Vec<Plugin>, Box<dyn std::error::Error>> {
        let sub_directory = match plugin_type {
            PluginType::Tool => TOOL_PLUGIN_DIRECTORY,
            PluginType::Model => MODEL_PLUGIN_DIRECTORY,
        };

        let plugin_path_exists = fs::exists(self.prompt_home.join(sub_directory))?;
        if !plugin_path_exists {
            return Ok(Vec::new());
        }

        let mut plugins: Vec<Plugin> = Vec::new();
        let plugin_files = fs::read_dir(self.prompt_home.join(sub_directory))?;
        for plugin_file in plugin_files {
            if let Ok(file) = plugin_file {
                if file.path().is_file()
                    && file.path().extension() == Some(std::ffi::OsStr::new("js"))
                {
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
