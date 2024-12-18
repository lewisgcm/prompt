use anyhow::format_err;
use rquickjs::Ctx;

pub fn map_js_err(error: rquickjs::Error, ctx: Ctx) -> anyhow::Error {
    let last_err = ctx.catch();
    println!("{:?}", last_err.type_of());
    if last_err.is_exception() | last_err.is_error() {
        return format_err!("{:#?}", last_err);
    }

    format_err!("{}", error)
}
