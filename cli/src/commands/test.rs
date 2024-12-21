use clap::ArgMatches;
use prompt_core::javascript_engine::JavascriptEngineModule;
use prompt_core::{eval_module, plugin};
use std::fs;

pub async fn run_command(sub_matches: &ArgMatches) -> Result<(), anyhow::Error> {
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
        let plugin = plugin::plugin_from_module(&ctx, value)?;
        let _ = plugin::plugin_test(&ctx, plugin).await?;

        Ok::<(), anyhow::Error>(())
    })?;

    println!("Finished running module.");

    Ok(())
}
