// common helpers and constants

use std::error::Error;

pub static CONFIG_FILE_NAME: &str = "ramenrc";

pub type BoxResult<T> = Result<T, Box<dyn Error>>;
