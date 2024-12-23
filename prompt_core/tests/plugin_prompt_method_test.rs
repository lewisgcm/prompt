use prompt_core::config::ModelConfigSettingType;
use prompt_core::javascript_engine::{modules, JavascriptEngineModule};
use prompt_core::plugin::{
    plugin_from_module, plugin_model_configure, plugin_model_prompt, ModelPrompt,
};
use prompt_core::{eval_module, javascript_engine};
use std::collections::HashMap;

#[tokio::test]
async fn test_simple_prompt_method() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("tests/model-plugin/prompt-method/simple.js").unwrap();

    let engine_result = javascript_engine::new(
        vec![JavascriptEngineModule {
            name: String::from("test"),
            code: String::from(js.as_str()),
        }],
        &printer,
    )
    .await;

    assert!(
        engine_result.is_ok(),
        "unexpected engine result: {:#?}",
        engine_result.err()
    );

    if let Ok(engine) = engine_result {
        let _ = eval_module!(&engine, "test", |ctx, value| {
            let plugin = plugin_from_module(&ctx, value);
            assert!(
                plugin.is_ok(),
                "unexpected error getting plugin: {:#?}",
                plugin.err()
            );

            if let Ok(plugin) = plugin {
                let configure = plugin_model_configure(
                    &ctx,
                    &plugin,
                    HashMap::from([(
                        String::from("deep"),
                        Some(ModelConfigSettingType::String(String::from("derp"))),
                    )]),
                )
                .await;
                assert!(
                    configure.is_ok(),
                    "unexpected error from plugin: {:#?}",
                    configure
                );

                let result =
                    plugin_model_prompt(&ctx, &plugin, ModelPrompt::Text("prompt".to_string()))
                        .await;

                assert!(
                    result.is_ok(),
                    "expecting model prompt to be ok: {:#?}",
                    result.err()
                );
                if let Ok(model_response) = result {
                    assert_eq!(
                        ModelPrompt::Text(
                            "I was configured with: derp, and was sent: prompt".to_string()
                        ),
                        model_response[0]
                    );
                }
            }

            return Ok(());
        });

        engine.idle().await;
    }
}
