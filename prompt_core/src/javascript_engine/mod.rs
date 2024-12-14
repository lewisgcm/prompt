use rquickjs::loader::{BuiltinLoader, BuiltinResolver, ModuleLoader};
use rquickjs::{async_with, AsyncContext, AsyncRuntime, CatchResultExt, Error, Module, Value};
pub struct JavascriptEngineModule {
    pub name: String,
    pub code: String,
}

pub struct JavascriptEngine {
    context: AsyncContext,
}
pub async fn new(
    modules: Vec<JavascriptEngineModule>,
) -> anyhow::Result<JavascriptEngine, anyhow::Error> {
    let runtime = AsyncRuntime::new()?;
    let context = AsyncContext::full(&runtime).await?;
    let mut resolver = BuiltinResolver::default()
        .with_module("fs")
        .with_module("fs/promises")
        .with_module("url")
        .with_module("path");

    let mut builtin_loader = BuiltinLoader::default();
    for module in modules {
        resolver.add_module(module.name.as_str());
        builtin_loader.add_module(module.name.as_str(), module.code.as_str());
    }

    let loader = (
        ModuleLoader::default()
            .with_module("fs", llrt_modules::fs::FsModule)
            .with_module("fs/promises", llrt_modules::fs::FsPromisesModule)
            .with_module("url", llrt_modules::url::UrlModule)
            .with_module("path", llrt_modules::path::PathModule),
        builtin_loader,
    );

    runtime.set_loader(resolver, loader).await;

    // Load default modules such as http, os, etc
    async_with!(context => |ctx| {
        llrt_modules::http::init(&ctx)?;
        llrt_modules::url::init(&ctx)?;

        Ok::<(), Error>(())
    })
    .await?;

    Ok(JavascriptEngine { context })
}

pub async fn eval_module(
    engine: &JavascriptEngine,
    name: &str,
) -> anyhow::Result<(), anyhow::Error> {
    async_with!(engine.context => |ctx| {
        let promise = Module::import(&ctx, name).catch(&ctx);
        return match promise {
            Ok(promise) => {
                let result = promise.into_future::<Value>().await.catch(&ctx);
                return if let Err(err) = result {
                    Err(anyhow::Error::msg(format!("{:#?}", err)))
                } else {
                    let v = result.unwrap();
                    println!("V: {:#?}", v);
                    Ok(())
                };
            },
            Err(err) => {
                Err(anyhow::Error::msg(format!("{:#?}", err)))
            }
        }
    })
    .await?;

    Ok(())
}
