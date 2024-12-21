use crate::commands::setup::{
    validators, Setup, SETUP_ADD_MODEL_PLUGIN_DISPLAY, SETUP_ADD_TOOL_PLUGIN_DISPLAY,
    SETUP_EXIT_DISPLAY, SETUP_MODEL_DISPLAY,
};
use inquire::{Confirm, CustomType, MultiSelect, Select, Text};
use prompt_core::config::{ModelConfig, Plugin};
use prompt_core::javascript_engine::{modules, JavascriptEngineModule};
use prompt_core::plugin::{plugin_from_module, plugin_model_configure};
use prompt_core::{eval_module, javascript_engine};
use std::error::Error;
use std::path::PathBuf;

pub struct InputPluginLocation {
    pub name: String,
    pub location: PathBuf,
}

pub async fn prompt_for_add_model_config(
    model_plugins: &Vec<Plugin>,
    tool_plugins: &Vec<Plugin>,
) -> Result<ModelConfig, Box<dyn Error>> {
    if model_plugins.is_empty() {
        return Err("No model plugins found.".into());
    }

    let selected_model = Select::new("Select model plugin", model_plugins.clone()).prompt()?;
    let js = std::fs::read_to_string(selected_model.location)?;
    let printer = modules::console::ConsoleLogger::new();
    let engine = javascript_engine::new(
        vec![JavascriptEngineModule {
            name: String::from(selected_model.name.clone()),
            code: String::from(js.as_str()),
        }],
        &printer,
    )
    .await?;

    let config = eval_module!(&engine, selected_model.name.clone(), |ctx, value| {
        let plugin = plugin_from_module(&ctx, value)?;
        let plugin_result = plugin_model_configure(&ctx, plugin, |x| {
            return match x.input_type.as_str() {
                "select" => {
                    let options = x
                        .options
                        .ok_or(format_err!("No options available for {}", x.name))?;
                    let input = Select::new(x.display_name.as_str(), options).prompt()?;

                    Ok(rquickjs::String::from_str(ctx.clone(), input.as_str())?
                        .as_value()
                        .clone())
                }
                "string" => {
                    let input = Text::new(x.display_name.as_str()).prompt()?;

                    Ok(rquickjs::String::from_str(ctx.clone(), input.as_str())?
                        .as_value()
                        .clone())
                }
                "bool" => {
                    let input = Confirm::new(x.display_name.as_str()).prompt()?;

                    Ok(Value::new_bool(ctx.clone(), input))
                }
                "float" => {
                    let input = inquire::prompt_f64(x.display_name.as_str())?;

                    Ok(Value::new_float(ctx.clone(), input))
                }
                "number" => {
                    let input = CustomType::<i32>::new(x.display_name.as_str()).prompt()?;

                    Ok(Value::new_int(ctx.clone(), input))
                }
                _ => Err(format_err!("unknown input type")),
            };
        })
        .await?;

        return Ok(plugin_result);
    })?;

    let selected_tools = if tool_plugins.is_empty() {
        vec![]
    } else {
        MultiSelect::new("Select tool plugin", tool_plugins.clone()).prompt()?
    };

    let loading = loading::Loading::default();
    loading.text("Cleaning up plugin engine");

    engine.idle().await;

    loading.end();

    Ok(ModelConfig {
        provider: "bedrock".to_string(),
        settings: Some(config),
        plugins: Some(
            selected_tools
                .iter()
                .map(|tool| tool.name.clone())
                .collect(),
        ),
    })
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
    let selected = Select::new(
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
