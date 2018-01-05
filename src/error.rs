use std::result;
use failure;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    // #[fail(display = "{}", _0)]
    // Base64DecodeError(DecodeError),
}
