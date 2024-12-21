use prompt_core::config::{ModelConfigSettingType, Plugin, PluginType, PromptConfig};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_load_config_no_file() {
    let home = PathBuf::from("tests/config/no-config-file");
    let result = PromptConfig::from_prompt_home(PathBuf::from(home.clone()));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().prompt_home, home);
}

#[test]
fn test_load_config_optionals() {
    let home = PathBuf::from("tests/config/empty");
    let result = PromptConfig::from_prompt_home(home);

    assert!(result.is_ok());

    let config_result = result.unwrap();
    assert!(!config_result.config.default_model.is_some());
    assert!(!config_result.config.models.is_some());
}

#[test]
fn test_load_config() {
    let home = PathBuf::from("tests/config/populated");
    let result = PromptConfig::from_prompt_home(home);
    assert!(result.is_ok());

    let config_result = result.unwrap();
    assert_eq!("test", config_result.config.default_model.unwrap());

    let binding = config_result.config.models.unwrap();
    let model_config = binding.get("claude").unwrap();

    assert_eq!("bedrock", model_config.provider.clone());
    assert_eq!(
        HashMap::from([
            (
                String::from("model-id"),
                Some(ModelConfigSettingType::String(String::from(
                    "anthropic.claude-3-haiku-20240307-v1:0"
                )))
            ),
            (
                String::from("region"),
                Some(ModelConfigSettingType::String(String::from("us-east-1")))
            ),
            (
                String::from("test"),
                None
            ),
            (
                String::from("integer"),
                Some(ModelConfigSettingType::Integer(5))
            ),
            (
                String::from("bool"),
                Some(ModelConfigSettingType::Bool(true))
            ),
            (
                String::from("float"),
                Some(ModelConfigSettingType::Float(5.20240307))
            )
        ]),
        *model_config.settings.as_ref().unwrap()
    );
}

#[test]
fn test_config_model_plugins() {
    let home = PathBuf::from("tests/config/plugins");
    let result = PromptConfig::from_prompt_home(home);
    assert!(result.is_ok());

    let config_result = result.unwrap();

    let model_plugins_result = config_result.list_plugins(PluginType::Model);
    assert!(model_plugins_result.is_ok());

    let model_plugins = model_plugins_result.unwrap();
    assert_eq!(model_plugins.len(), 2);
    assert_eq!(
        model_plugins,
        vec![
            Plugin {
                name: String::from("anthropic.js"),
                plugin_type: PluginType::Model,
                location: PathBuf::from("tests/config/plugins/model_plugins/anthropic.js.js")
            },
            Plugin {
                name: String::from("bedrock"),
                plugin_type: PluginType::Model,
                location: PathBuf::from("tests/config/plugins/model_plugins/bedrock.js")
            },
        ]
    );
}

#[test]
fn test_config_tool_plugins() {
    let home = PathBuf::from("tests/config/plugins");
    let result = PromptConfig::from_prompt_home(home);
    assert!(result.is_ok());

    let config_result = result.unwrap();

    let model_plugins_result = config_result.list_plugins(PluginType::Tool);
    assert!(model_plugins_result.is_ok());

    let model_plugins = model_plugins_result.unwrap();
    assert_eq!(model_plugins.len(), 2);
    assert_eq!(
        model_plugins,
        vec![
            Plugin {
                name: String::from("weather"),
                plugin_type: PluginType::Tool,
                location: PathBuf::from("tests/config/plugins/tool_plugins/weather.js")
            },
            Plugin {
                name: String::from("google.js"),
                plugin_type: PluginType::Tool,
                location: PathBuf::from("tests/config/plugins/tool_plugins/google.js.js")
            },
        ]
    );
}
