
// common helpers and constants

use std::error::Error;

pub static DEFAULT_CONFIG_DIR: &str = "/usr/share/ramen";
pub static CONFIG_FILE_NAME: &str = "ramenrc";

pub static DEFAULT_IMG_URL: &str = ""; // TODO maybe make this a config option

pub type BoxResult<T> = Result<T,Box<Error>>;

