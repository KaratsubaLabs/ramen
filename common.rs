
// common helpers and constants

use std::error::Error;

pub static DEFAULT_CONFIG_DIR: &str = "/usr/share/ramen";
pub static CONFIG_FILE_NAME: &str = "ramenrc";

pub type BoxResult<T> = Result<T,Box<Error>>;

