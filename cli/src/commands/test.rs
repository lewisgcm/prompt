use crate::util::map_js_err;
use anyhow::format_err;
use clap::ArgMatches;
use prompt_core::eval_module;
use prompt_core::javascript_engine::JavascriptEngineModule;
use rquickjs::promise::MaybePromise;
use rquickjs::Value;
use std::fs;

pub async fn run_command(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let logger = prompt_core::javascript_engine::modules::console::ConsoleLogger::new();

    println!("Creating JS engine...");
    let module_path = sub_matches
        .get_one::<String>("PATH")
        .expect("PATH should be set");
    let data = fs::read_to_string(module_path)?;
    let js_engine = prompt_core::javascript_engine::new(
        vec![JavascriptEngineModule {
            name: "test_module".to_string(),
            code: data,
        }],
        &logger,
    )
    .await?;

    println!("Running module...");

    let () = eval_module!(&js_engine, "test_module", |ctx, value| {
        let object = value.as_object();
        if let Some(object) = object {
            let handler: Value = object.get("test").map_err(|e| map_js_err(e, ctx.clone()))?;

            return match handler.as_function() {
                None => Err(format_err!("No function named 'test' defined in module.")),
                Some(handler) => {
                    let handler_promise: MaybePromise =
                        handler.call(()).map_err(|e| map_js_err(e, ctx.clone()))?;

                    let () = handler_promise
                        .into_future()
                        .await
                        .map_err(|e| map_js_err(e, ctx))?;

                    Ok(())
                }
            };
        }

        Ok(())
    })?;

    Ok(())
}
