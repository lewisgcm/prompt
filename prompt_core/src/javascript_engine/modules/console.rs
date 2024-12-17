use rquickjs::function::{Func, Rest};
use rquickjs::{Ctx, Error, Object, Value};
use std::io::Write;

pub trait Console {
    fn log(&self, ctx: Ctx, args: Rest<Value>);
    fn error(&self, ctx: Ctx, args: Rest<Value>);
    fn warn(&self, ctx: Ctx, args: Rest<Value>);
    fn info(&self, ctx: Ctx, args: Rest<Value>);
    fn debug(&self, ctx: Ctx, args: Rest<Value>);
    fn trace(&self, ctx: Ctx, args: Rest<Value>);
}

pub struct PrintStream {}

impl Console for PrintStream {
    fn log(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }

    fn error(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }

    fn warn(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }

    fn info(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }

    fn debug(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }

    fn trace(&self, ctx: Ctx, args: Rest<Value>) {
        args.iter().for_each(|arg| {
            print!("{:?}", arg);
        });
    }
}

//unsafe impl Sync for dyn Console {}
//unsafe impl Send for dyn Console {}

pub fn init(ctx: &Ctx, logger: &'static dyn Console) -> Result<(), Error> {
    let console = Object::new(ctx.clone())?;
    console.set(
        "log",
        Func::from(|ctx: Ctx, args: Rest<Value>| {
            logger.log(ctx, args);
        }),
    )?;
    ctx.globals().set("console", console)?;

    Ok(())
}
