use anyhow::format_err;
use rquickjs::CaughtError;

pub trait CaughtResultExt<O> {
    fn to_result(self) -> Result<O, anyhow::Error>;
}

fn format_error(error: CaughtError) -> anyhow::Error {
    format_err!("{}", error)
}

impl<'a, O> CaughtResultExt<O> for Result<O, CaughtError<'a>> {
    fn to_result(self) -> Result<O, anyhow::Error> {
        self.map_err(format_error)
    }
}
