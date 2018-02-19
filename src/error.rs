use std::result;
use std::error;

pub type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug)]
pub enum Error {}