use rquickjs::loader::{BuiltinLoader, BuiltinResolver, ModuleLoader};
use rquickjs::{async_with, AsyncContext, AsyncRuntime, Error};

pub struct JavascriptEngineModule {
    pub name: String,
    pub code: String,
}

pub struct JavascriptEngine {
    pub context: AsyncContext,
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

#[macro_export]
macro_rules! eval_module {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($script_engine:expr, $name:expr, |$value:ident| { $($t:tt)* }) => {
        rquickjs::async_with!($script_engine.context => |ctx| {
        use rquickjs::{CatchResultExt};

        let promise = rquickjs::Module::import(&ctx, $name).catch(&ctx);
        return match promise {
                Ok(promise) => {
                    let result = promise.into_future::<Value>().await.catch(&ctx);
                    return if let Err(err) = result {
                        Err(anyhow::Error::msg(format!("{:#?}", err)))
                    } else {
                        let $value = result.unwrap();

                        let fut = Box::pin(async move {
                            $($t)*
                        });

                        fut.await
                    };
                },
                Err(err) => {
                    Err(anyhow::Error::msg(format!("{:#?}", err)))
                }
            }
        })
        .await
    };
}
