mod input_prompt;
mod validators;

use clap::ArgMatches;
use homedir::my_home;
use prompt_core::config;
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

const PROMPT_DEFAULT_DIRECTORY: &str = ".prompt";
const SETUP_MODEL_DISPLAY: &'static str = "Add Model";
const SETUP_ADD_MODEL_PLUGIN_DISPLAY: &'static str = "Add Model Plugin";
const SETUP_ADD_TOOL_PLUGIN_DISPLAY: &'static str = "Add Tool Plugin";
const SETUP_EXIT_DISPLAY: &'static str = "Exit";

#[derive(Debug)]
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

    let has_setup_previously = config::Config::exists(home_directory.clone())?;

    let config =
        config::Config::from_directory(home_directory.clone())?.unwrap_or(config::Config::empty());

    let start_command = if !has_setup_previously {
        Some(Setup::AddModelPlugin)
    } else {
        input_prompt::prompt_for_next_command()?
    };

    let mut command_queue = VecDeque::from(start_command.map(|c| vec![c]).unwrap_or(vec![]));
    while let Some(next_command) = command_queue.pop_front() {
        match next_command {
            Setup::AddModel => {
                // 1.Get list of available model plugins
                // 2. Get model config setting
                //  2.1. Call a 'config' option in the model module, which returns a list of config inputs
                // 3. Prompt based on the inputs
            }
            Setup::AddModelPlugin | Setup::AddToolPlugin => {
                let _ = input_prompt::prompt_for_plugin_location();
                //
            }
        }
    }

    config.to_directory(home_directory)?;

    Ok(())
}
