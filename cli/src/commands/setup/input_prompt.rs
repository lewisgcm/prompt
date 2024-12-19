use crate::commands::setup::{
    validators, Setup, SETUP_ADD_MODEL_PLUGIN_DISPLAY, SETUP_ADD_TOOL_PLUGIN_DISPLAY,
    SETUP_EXIT_DISPLAY, SETUP_MODEL_DISPLAY,
};
use inquire::Text;
use std::error::Error;
use std::path::PathBuf;

pub struct InputPluginLocation {
    pub name: String,
    pub location: PathBuf,
}

pub fn prompt_for_plugin_location() -> Result<InputPluginLocation, Box<dyn Error>> {
    let location = Text::new("Where is the bundled plugin js file located?")
        .with_validator(validators::plugin_location_validator)
        .prompt()
        .map(|l| PathBuf::from(l))?;

    let name = Text::new("What will the plugin be named (letters, numbers and '_' only)?")
        .with_validator(validators::plugin_name_validator)
        .prompt()?;

    Ok(InputPluginLocation { name, location })
}

pub fn prompt_for_next_command() -> Result<Option<Setup>, Box<dyn Error>> {
    let selected = inquire::Select::new(
        "What would you like to configure?",
        vec![
            SETUP_MODEL_DISPLAY,
            SETUP_ADD_MODEL_PLUGIN_DISPLAY,
            SETUP_ADD_TOOL_PLUGIN_DISPLAY,
            SETUP_EXIT_DISPLAY,
        ],
    )
    .prompt()?;

    Ok(match selected {
        SETUP_MODEL_DISPLAY => Some(Setup::AddModel),
        SETUP_ADD_MODEL_PLUGIN_DISPLAY => Some(Setup::AddModelPlugin),
        SETUP_ADD_TOOL_PLUGIN_DISPLAY => Some(Setup::AddToolPlugin),
        _ => None,
    })
}
