mod input_prompt;
mod validators;

use clap::ArgMatches;
use homedir::my_home;
use prompt_core::config;
use prompt_core::config::{Plugin, PluginType};
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

const PROMPT_DEFAULT_DIRECTORY: &str = ".prompt";
const SETUP_MODEL_DISPLAY: &'static str = "Add Model";
const SETUP_ADD_MODEL_PLUGIN_DISPLAY: &'static str = "Add Model Plugin";
const SETUP_ADD_TOOL_PLUGIN_DISPLAY: &'static str = "Add Tool Plugin";
const SETUP_EXIT_DISPLAY: &'static str = "Exit";

#[derive(Debug, Eq, PartialEq)]
enum Setup {
    AddModel,
    AddModelPlugin,
    AddToolPlugin,
}
pub async fn run_command(sub_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let user_home_directory = my_home()?.map(|home_dir| {
        PathBuf::from(home_dir.to_str().unwrap().to_string()).join(PROMPT_DEFAULT_DIRECTORY)
    });

    let home_directory = sub_matches
        .get_one::<String>("DIR")
        .map(PathBuf::from)
        .or_else(|| user_home_directory)
        .ok_or_else(|| "could not resolve home directory")?;

    let config = config::PromptConfig::from_prompt_home(home_directory)?;
    let installed_model_plugins = config.list_plugins(PluginType::Model)?;
    let installed_tool_plugins = config.list_plugins(PluginType::Tool)?;

    if sub_matches.get_flag("LIST_MODEL_PLUGINS") {
        print_plugins(&installed_model_plugins);
        return Ok(());
    } else if sub_matches.get_flag("LIST_TOOL_PLUGINS") {
        print_plugins(&installed_tool_plugins);
        return Ok(());
    }

    let start_command = if installed_model_plugins.is_empty() {
        println!("You don't have any plugins installed. Lets install one now.");
        Some(Setup::AddModelPlugin)
    } else {
        input_prompt::prompt_for_next_command()?
    };

    let mut command_queue = VecDeque::from(start_command.map(|c| vec![c]).unwrap_or(vec![]));
    while let Some(next_command) = command_queue.pop_front() {
        match next_command {
            Setup::AddModel => {
                let model_plugins = config.list_plugins(PluginType::Model)?;
                let tool_plugins = config.list_plugins(PluginType::Tool)?;

                input_prompt::prompt_for_add_model_config(
                    &model_plugins,
                    &tool_plugins,
                )?;
                // 1.Get list of available model plugins
                // 2. Get model config setting
                //  2.1. Call a 'config' option in the model module, which returns a list of config inputs
                // 3. Prompt based on the inputs
            }
            Setup::AddModelPlugin | Setup::AddToolPlugin => {
                let plugin_type = if next_command == Setup::AddModelPlugin {
                    PluginType::Model
                } else {
                    PluginType::Tool
                };
                let plugin = input_prompt::prompt_for_plugin_location()?;
                config.install_plugin(Plugin {
                    name: plugin.name,
                    location: plugin.location,
                    plugin_type,
                })?;
            }
        }
    }

    config.write()?;

    Ok(())
}

fn print_plugins(plugins: &Vec<Plugin>) {
    if plugins.is_empty() {
        println!("You have no installed plugins.");
        return;
    }

    println!("Installed plugins:");
    for plugin in plugins {
        println!("\t {}: {}", plugin.name, plugin.location.display());
    }
}
