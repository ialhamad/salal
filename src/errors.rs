use std::{error::Error, fmt};
#[derive(Debug)]
pub struct SalalError {
    details: String,
}

impl SalalError {
    pub fn new(msg: &str) -> SalalError {
        SalalError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SalalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SalalError {
    fn description(&self) -> &str {
        &self.details
    }
}
