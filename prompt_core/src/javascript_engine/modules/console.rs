use rquickjs::function::{Func, Rest};
use rquickjs::{Ctx, Error, Object, Value};

pub trait Logger {
    fn log(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
    fn info(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
    fn error(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
    fn warn(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
    fn debug(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
    fn trace(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error>;
}

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {}
    }
}

enum LogLevel {
    Info,
    Warn,
    Debug,
    Trace,
    Error,
    Log,
}

fn log(_log_level: LogLevel, _ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
    args.iter().for_each(|arg| {
        print!("{:?}", arg);
    });
    println!();

    Ok(())
}

impl Logger for ConsoleLogger {
    fn log(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Log, ctx, args)
    }

    fn info(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Info, ctx, args)
    }

    fn error(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Error, ctx, args)
    }

    fn warn(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Warn, ctx, args)
    }

    fn debug(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Debug, ctx, args)
    }

    fn trace(&self, ctx: Ctx, args: Rest<Value>) -> Result<(), Error> {
        log(LogLevel::Trace, ctx, args)
    }
}

pub fn init<'js>(ctx: &Ctx, logger: &dyn Logger) -> Result<(), Error> {
    let c = unsafe { Ctx::from_raw(ctx.as_raw()) };
    let console = Object::new(c.clone())?;
    console.set(
        "log",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.log(c, args);
        }),
    )?;
    console.set(
        "warn",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.warn(c, args);
        }),
    )?;
    console.set(
        "error",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.error(c, args);
        }),
    )?;
    console.set(
        "debug",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.debug(c, args);
        }),
    )?;
    console.set(
        "trace",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.trace(c, args);
        }),
    )?;
    console.set(
        "info",
        Func::from(|c: Ctx, args: Rest<Value>| {
            return logger.trace(c, args);
        }),
    )?;
    c.globals().set("console", console)?;

    Ok(())
}
