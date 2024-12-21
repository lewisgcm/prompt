use prompt_core::javascript_engine::{modules, JavascriptEngineModule};
use prompt_core::plugin::{plugin_from_module, plugin_model_configure, plugin_test};
use prompt_core::{eval_module, javascript_engine};

#[tokio::test]
async fn test_invalid_js() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("tests/model-plugin/test-method/invalid-js.js").unwrap();
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
        let result = eval_module!(&engine, "test", |ctx, _value| { Ok(()) });
        assert!(result.is_err(), "expected module error");
    }
}

#[tokio::test]
async fn test_no_test_method() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("tests/model-plugin/test-method/no-test-method.js").unwrap();

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
                let plugin_result = plugin_test(&ctx, plugin).await;
                assert!(plugin_result.is_err(), "expected plugin error");
                assert_eq!(
                    plugin_result.err().unwrap().to_string(),
                    "Method 'test' not found on plugin."
                );
            }

            Ok(())
        });
    }
}

#[tokio::test]
async fn test_thrown_error_test_method() {
    let printer = modules::console::ConsoleLogger::new();
    let js =
        std::fs::read_to_string("tests/model-plugin/test-method/test-method-throws.js").unwrap();

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
                let plugin_result = plugin_test(&ctx, plugin).await;
                assert!(plugin_result.is_err(), "expected plugin error");
            }

            return Ok(());
        });

        engine.idle().await;
    }
}

#[tokio::test]
async fn test_simple_test_method() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("tests/model-plugin/test-method/simple.js").unwrap();

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
                let plugin_result = plugin_test(&ctx, plugin).await;
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
