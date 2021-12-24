
// common helpers

use std::error::Error;

pub type BoxResult<T> = Result<T,Box<Error>>;

