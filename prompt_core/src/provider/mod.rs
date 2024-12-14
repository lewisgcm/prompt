use llrt_modules::buffer;
use rquickjs::{Context, Error, Module, Runtime};

pub fn javascript() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;

    context.with(|ctx| {
        //buffer::init(ctx)?;
        //buffer::init(*ctx)?;

        //let (_module, module_eval) = Module::evaluate_def(ctx.clone(), "buffer")?;

        ctx.eval(
            r#"
          import { Buffer } from "buffer";
          Buffer.alloc(10);
          "#,
        )?;

        Ok::<_, Error>(())
    })?;

    Ok(())
}
