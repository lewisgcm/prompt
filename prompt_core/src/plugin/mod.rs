mod util;

use crate::config::ModelConfigSettingType;
use crate::javascript_engine::util::CaughtResultExt;
use crate::plugin::util::JSTypeExt;
use anyhow::format_err;
use rquickjs::prelude::This;
use rquickjs::promise::MaybePromise;
use rquickjs::{Array, CatchResultExt, Ctx, Function, Object, Value};
use std::collections::HashMap;

const PLUGIN_CONFIGURATION_METHOD_NAME: &str = "configuration";
const PLUGIN_CONFIGURE_METHOD_NAME: &str = "configure";
const CONFIGURATION_INPUT_FIELD_NAME: &str = "input";
const PLUGIN_PROMPT_METHOD_NAME: &str = "prompt";

pub fn plugin_from_module<'js>(
    _ctx: &Ctx<'js>,
    module: Value<'js>,
) -> Result<Object<'js>, anyhow::Error> {
    let object = module
        .as_object()
        .ok_or(format_err!("no exported members found in module."))?;

    let plugin: Value = object
        .clone()
        .get("plugin")
        .map_err(|_| format_err!("member does not exist in module"))?;

    if let Some(plugin_object) = plugin.as_object() {
        return Ok(plugin_object.clone());
    }

    Err(format_err!("The 'plugin' export should be an object"))
}

pub async fn plugin_test<'js>(ctx: &'_ Ctx<'js>, plugin: Object<'js>) -> Result<(), anyhow::Error> {
    let test_method: Function = plugin
        .get("test")
        .map_err(|_| format_err!("Method 'test' not found in plugin."))?;

    let test_method_invocation_promise: MaybePromise =
        test_method.call(()).catch(ctx).to_result()?;

    let _: () = test_method_invocation_promise
        .into_future()
        .await
        .catch(ctx)
        .to_result()?;

    Ok(())
}

pub struct ModelInput {
    pub name: String,
    pub display_name: String,
    pub input_type: String,
    pub required: bool,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModelPromptBinaryPayload {
    pub value: Vec<u8>,
    pub format: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModelPrompt {
    Text(String),
    Image(ModelPromptBinaryPayload),
    Document(ModelPromptBinaryPayload),
}

pub async fn plugin_model_prompt<'js>(
    ctx: &'_ Ctx<'js>,
    plugin: &'_ Object<'js>,
    model_prompt: ModelPrompt,
) -> Result<Vec<ModelPrompt>, anyhow::Error> {
    let method: Function = plugin.get(PLUGIN_PROMPT_METHOD_NAME).map_err(|_| {
        format_err!(
            "Method '{}' not found in model plugin.",
            PLUGIN_PROMPT_METHOD_NAME
        )
    })?;

    let prompt_value: Value<'js> = model_prompt.to_value(ctx.clone())?;
    let method_invocation_promise: MaybePromise = method
        .call((This(plugin.clone()), prompt_value))
        .catch(ctx)
        .to_result()?;
    let responses: Array = method_invocation_promise
        .into_future()
        .await
        .catch(ctx)
        .to_result()?;

    let mut model_responses: Vec<ModelPrompt> = Vec::new();
    for response in responses {
        if let Ok(response) = response {
            model_responses.push(ModelPrompt::from_value(response)?);
        }
    }

    Ok(model_responses)
}

pub async fn plugin_model_configure<'js>(
    ctx: &'_ Ctx<'js>,
    plugin: &'_ Object<'js>,
    settings: Option<HashMap<String, Option<ModelConfigSettingType>>>,
) -> Result<(), anyhow::Error> {
    let method: Function = plugin.get(PLUGIN_CONFIGURE_METHOD_NAME).map_err(|_| {
        format_err!(
            "Method '{}' not found in model plugin.",
            PLUGIN_CONFIGURE_METHOD_NAME
        )
    })?;

    let object = Object::new(ctx.clone())?;
    if let Some(settings) = settings {
        for setting in settings {
            if let Some(setting_value) = setting.1 {
                object.set(setting.0, setting_value.to_value(ctx.clone())?)?;
            }
        }
    }

    let method_invocation_promise: MaybePromise = method
        .call((This(plugin.clone()), object))
        .catch(ctx)
        .to_result()?;

    let _: () = method_invocation_promise
        .into_future()
        .await
        .catch(ctx)
        .to_result()?;

    Ok(())
}

pub async fn plugin_model_configuration<'js, F>(
    ctx: &'_ Ctx<'js>,
    plugin: Object<'js>,
    get_input: F,
) -> Result<HashMap<String, Option<ModelConfigSettingType>>, anyhow::Error>
where
    F: FnOnce(ModelInput) -> Result<Value<'js>, anyhow::Error> + Clone,
{
    let method: Function = plugin.get(PLUGIN_CONFIGURATION_METHOD_NAME).map_err(|_| {
        format_err!(
            "Method '{}' not found in model plugin.",
            PLUGIN_CONFIGURATION_METHOD_NAME
        )
    })?;

    let method_invocation_promise: MaybePromise = method.call(()).catch(ctx).to_result()?;

    let method_invocation_result: Value = method_invocation_promise
        .into_future()
        .await
        .catch(ctx)
        .to_result()?;

    let configuration = method_invocation_result.as_array().ok_or(format_err!(
        "Method '{}' should return an array.",
        PLUGIN_CONFIGURATION_METHOD_NAME
    ))?;

    let input_context = Object::new(ctx.clone())?;

    for value in configuration.iter::<Value>() {
        if let Ok(value) = value {
            let object = value.as_object().ok_or(format_err!(
                "Method 'configuration' should return array or configuration steps."
            ))?;

            let input: Value = object.get(CONFIGURATION_INPUT_FIELD_NAME)?;
            let input_function: &Function = input.as_function().ok_or(format_err!(
                "Configuration step '{}' field should be a function",
                CONFIGURATION_INPUT_FIELD_NAME
            ))?;

            let result_promise: MaybePromise = input_function
                .call((input_context.clone(),))
                .catch(ctx)
                .to_result()?;

            let result: Value = result_promise.into_future().await.catch(ctx).to_result()?;
            let result_object = result
                .as_object()
                .ok_or(format_err!("Configuration step should return an object."))?;

            for prop in result_object.props::<String, Object>() {
                if let Ok(prop) = prop {
                    let display_name: String = prop.1.get("displayName")?;
                    let input_type: String = prop.1.get("type")?;
                    let required: bool = prop.1.get("required")?;
                    let options: Option<Vec<String>> = prop.1.get("options")?;

                    let input = get_input.clone()(ModelInput {
                        name: prop.0.clone(),
                        display_name,
                        input_type,
                        required,
                        options,
                    })?;

                    input_context.set(prop.0, input.clone())?;
                }
            }
        }
    }

    let mut settings: HashMap<String, Option<ModelConfigSettingType>> = HashMap::new();
    for setting in input_context.props::<String, Value>() {
        if let Ok(setting) = setting {
            let value = ModelConfigSettingType::from_value(setting.1.clone())?;
            settings.insert(setting.0, Some(value.clone()));
        }
    }

    Ok(settings)
}
