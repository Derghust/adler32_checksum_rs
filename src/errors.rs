use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RError {
    message: String
}

impl RError {
    pub fn new(message: &str) -> Self {
        RError{message: message.to_string()}
    }
}


impl Display for RError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RError {
    fn description(&self) -> &str {
        &self.message
    }
}