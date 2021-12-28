
use std::fmt;
use std::error::Error;

// simple error, holds a text string with the error message

#[derive(Debug)]
pub struct SimpleError {
    pub msg: String
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError {
            msg: msg.to_string()
        }
    }
}

impl Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

