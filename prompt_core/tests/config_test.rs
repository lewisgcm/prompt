use std::collections::HashMap;

#[test]
fn test_load_config_no_file() {
    let result = prompt_core::config::load_config("tests/config/unknown.yml");
    assert!(!result.is_ok());
}

#[test]
fn test_load_config_optionals() {
    let result = prompt_core::config::load_config("tests/config/test-config-no-optionals.yml");
    assert!(result.is_ok());

    let config_result = result.unwrap();
    assert!(!config_result.default_model.is_some());
    assert!(!config_result.models.is_some());
}

#[test]
fn test_load_config() {
    let result = prompt_core::config::load_config("tests/config/test-config.yml");
    assert!(result.is_ok());

    let config_result = result.unwrap();
    assert_eq!("test", config_result.default_model.unwrap());

    let binding = config_result.models.unwrap();
    let model_config = binding.get("claude").unwrap();

    assert_eq!("bedrock", model_config.provider);
    assert_eq!(
        HashMap::from([
            (
                String::from("model-id"),
                String::from("anthropic.claude-3-haiku-20240307-v1:0")
            ),
            (String::from("region"), String::from("us-east-1"))
        ]),
        model_config.settings.clone().unwrap()
    );
}
