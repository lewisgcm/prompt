use llrt_modules::timers::poll_timers;
use rquickjs::atom::PredefinedAtom;
use rquickjs::function::Func;
use rquickjs::{qjs, Ctx, Error, Module, Object, Value};
use tokio::time::Instant;

fn require(ctx: Ctx, name: String) -> Result<Value, Error> {
    let import_promise = Module::import(&ctx, name)?;
    let rt = unsafe { qjs::JS_GetRuntime(ctx.as_raw().as_ptr()) };
    let mut deadline = Instant::now();
    let mut executing_timers = Vec::new();
    let imported_object = loop {
        if let Some(x) = import_promise.result::<Object>() {
            break x?;
        }

        if deadline < Instant::now() {
            poll_timers(rt, &mut executing_timers, None, Some(&mut deadline))?;
        }

        ctx.execute_pending_job();
    };

    let props = imported_object.props::<String, Value>();

    let default_export: Option<Value> = imported_object.get(PredefinedAtom::Default)?;
    if let Some(default_export) = default_export {
        //if default export is object attach all named exports to
        if let Some(default_object) = default_export.as_object() {
            for prop in props {
                let (key, value) = prop?;
                if !default_object.contains_key(&key)? {
                    default_object.set(key, value)?;
                }
            }
            let default_object = default_object.clone().into_value();
            return Ok(default_object);
        }
    }

    let obj = Object::new(ctx.clone())?;
    for prop in props {
        let (key, value) = prop?;
        obj.set(key, value)?;
    }

    let value = obj.into_value();

    Ok(value)
}

pub fn init(ctx: &Ctx) -> Result<(), Error> {
    ctx.globals().set("require", Func::from(require))?;

    Ok(())
}
