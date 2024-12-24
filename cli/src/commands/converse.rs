use crate::util::get_home_dir;
use anyhow::format_err;
use clap::ArgMatches;
use prompt_core::javascript_engine::JavascriptEngineModule;
use prompt_core::plugin::{ModelPrompt, ModelPromptBinaryPayload};
use prompt_core::{config, eval_module, plugin};
use std::fs;
use std::path::PathBuf;

pub async fn run_command(sub_matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let home_directory = get_home_dir(sub_matches)?;
    let prompt = get_prompt_from_input(sub_matches)?;
    let config = config::PromptConfig::from_prompt_home(home_directory)?;

    let model_name = sub_matches
        .get_one::<String>("MODEL")
        .map(|x| x.clone())
        .or_else(|| config.config.default_model.clone())
        .ok_or(format_err!("no model specified"))?;

    let (model_config, model_plugin) = config.get_model_plugin(model_name.clone())?;
    let logger = prompt_core::javascript_engine::modules::console::ConsoleLogger::new();
    let data = fs::read_to_string(model_plugin.location)?;
    let js_engine = prompt_core::javascript_engine::new(
        vec![JavascriptEngineModule {
            name: model_name.clone(),
            code: data,
        }],
        &logger,
    )
    .await?;

    let result = eval_module!(&js_engine, model_name.clone(), |ctx, value| {
        let plugin = plugin::plugin_from_module(&ctx, value)?;
        plugin::plugin_model_configure(&ctx, &plugin, model_config.settings.clone()).await?;
        Ok::<Vec<ModelPrompt>, anyhow::Error>(
            plugin::plugin_model_prompt(&ctx, &plugin, prompt).await?,
        )
    })?;

    println!("{:#?}", result);

    js_engine.idle().await;

    Ok(())
}

fn get_prompt_from_input(sub_matches: &ArgMatches) -> Result<ModelPrompt, anyhow::Error> {
    let input_text: Option<&String> = sub_matches.get_one("INPUT_TEXT");
    let input_image: Option<&String> = sub_matches.get_one("INPUT_IMAGE");
    let input_document: Option<&String> = sub_matches.get_one("INPUT_DOCUMENT");

    if let Some(input_text) = input_text {
        return Ok(ModelPrompt::Text(input_text.to_string()));
    } else if let Some(input_image) = input_image {
        let data = fs::read(input_image)?;
        let path = PathBuf::from(input_image);
        let extension = path.extension();

        return match extension {
            Some(extension) => Ok(ModelPrompt::Image(ModelPromptBinaryPayload {
                value: data,
                format: extension
                    .to_str()
                    .ok_or(format_err!("error mapping file extension"))?
                    .to_string(),
            })),
            None => Err(format_err!("could not resolve file extension")),
        };
    } else if let Some(input_document) = input_document {
        let data = fs::read(input_document)?;
        let path = PathBuf::from(input_document);
        let extension = path.extension();

        return match extension {
            Some(extension) => Ok(ModelPrompt::Document(ModelPromptBinaryPayload {
                value: data,
                format: extension
                    .to_str()
                    .ok_or(format_err!("error mapping file extension"))?
                    .to_string(),
            })),
            None => Err(format_err!("could not resolve file extension")),
        };
    }

    Err(format_err!(
        "text, image, or document are required for input prompt"
    ))
}
