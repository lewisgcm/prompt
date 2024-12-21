use prompt_core::javascript_engine::{modules, JavascriptEngineModule};
use prompt_core::plugin::{plugin_from_module, plugin_model_configure};
use prompt_core::{eval_module, javascript_engine};

#[tokio::test]
async fn test_simple_configure_method() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("tests/model-plugin/configure-method/simple.js").unwrap();

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
                let plugin_result = plugin_model_configure(&ctx, plugin, |x| {
                    let opt = x.options.unwrap().get(0).unwrap().clone();
                    let s = rquickjs::String::from_str(ctx.clone(), opt.as_str())?;
                    return Ok(s.as_value().clone());
                })
                .await;
                assert!(
                    plugin_result.is_ok(),
                    "unexpected plugin result: {:?}",
                    plugin_result
                );
            }

            return Ok(());
        });

        engine.idle().await;
    }
}
