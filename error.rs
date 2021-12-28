
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

// TODO maybe move this to commands.rs
#[derive(Debug)]
pub struct CommandError {
    pub msg: Option<String>,
    pub display_help: bool,
}

impl CommandError {
    pub fn new(msg: Option<&str>, display_help: bool) -> CommandError {
        CommandError{
            msg: msg.map(str::to_string),
            display_help: display_help
        }
    }
    pub fn with_help() -> CommandError {
        CommandError::new(None, true)
    }
    pub fn with_error(msg: &str) -> CommandError {
        CommandError::new(Some(msg), false)
    }
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.msg.is_some() {
            write!(f, "{}", self.msg.as_ref().unwrap())
        } else {
            write!(f, "")
        }
    }
}

