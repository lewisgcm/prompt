pub mod modules;

use crate::javascript_engine::modules::console::Console;
use futures::FutureExt;
use rquickjs::loader::{BuiltinLoader, BuiltinResolver, Loader, ModuleLoader};
use rquickjs::{async_with, AsyncContext, AsyncRuntime, CatchResultExt, Error};
use std::future::Future;
use std::io::Write;

pub struct JavascriptEngineModule {
    pub name: String,
    pub code: String,
}

pub struct JavascriptEngine {
    pub context: AsyncContext,
    pub runtime: AsyncRuntime,
}

impl JavascriptEngine {
    pub async fn idle(&self) {
        self.runtime.idle().await;
    }
}

pub async fn new(
    modules: Vec<JavascriptEngineModule>,
    console: &'static (dyn Console + Send + Sync),
) -> anyhow::Result<JavascriptEngine, anyhow::Error> {
    let runtime = AsyncRuntime::new()?;
    runtime.set_max_stack_size(512 * 1024).await;
    runtime.set_gc_threshold(20 * 1024 * 1024).await;
    let context = AsyncContext::full(&runtime).await?;
    let mut resolver = BuiltinResolver::default()
        .with_module("fs")
        .with_module("fs/promises")
        .with_module("url")
        .with_module("path")
        .with_module("crypto")
        .with_module("buffer")
        .with_module("timers")
        .with_module("stream")
        .with_module("process")
        .with_module("child_process")
        .with_module("os")
        .with_module("events");

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
            .with_module("path", llrt_modules::path::PathModule)
            .with_module("crypto", llrt_modules::crypto::CryptoModule)
            .with_module("buffer", llrt_modules::buffer::BufferModule)
            .with_module("timers", llrt_modules::timers::TimersModule)
            .with_module("events", llrt_modules::events::EventsModule)
            .with_module("process", llrt_modules::process::ProcessModule)
            .with_module(
                "child_process",
                llrt_modules::child_process::ChildProcessModule,
            )
            .with_module("os", llrt_modules::os::OsModule),
        builtin_loader,
    );

    runtime.set_loader(resolver, loader).await;

    // Load default modules such as http, os, etc
    async_with!(context => |ctx| {
        llrt_modules::http::init(&ctx)?;
        llrt_modules::url::init(&ctx)?;
        llrt_modules::crypto::init(&ctx)?;
        llrt_modules::buffer::init(&ctx)?;
        llrt_modules::timers::init(&ctx)?;
        llrt_modules::navigator::init(&ctx)?;
        llrt_modules::abort::init(&ctx)?;
        llrt_modules::process::init(&ctx)?;
        llrt_modules::events::init(&ctx)?;
        modules::require::init(&ctx)?;
        modules::console::init(&ctx, console)?;

        Ok::<(), Error>(())
    })
    .await?;

    Ok(JavascriptEngine { runtime, context })
}

#[macro_export]
macro_rules! eval_module {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($script_engine:expr, $name:expr, |$ctx:ident,$value:ident| { $($t:tt)* }) => {
        rquickjs::async_with!($script_engine.context => |$ctx| {
            use rquickjs::{CatchResultExt};

            let promise = rquickjs::Module::import(&$ctx, $name).catch(&$ctx);
            return match promise {
                    Ok(promise) => {
                        let result = promise.into_future::<Value>().await.catch(&$ctx);
                        return if let Err(err) = result {
                            Err(anyhow::Error::msg(format!("{:#?}", err)))
                        } else {
                            let $value = result.unwrap();

                            $($t)*
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
