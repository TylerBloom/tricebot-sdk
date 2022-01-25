use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TriceError {
    reason: String,
}

impl TriceError {
    pub fn new(reason: String) -> Self {
        TriceError { reason }
    }
}

impl fmt::Display for TriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "An error while communicating with TriceBot: {}",
            self.reason
        )
    }
}

impl Error for TriceError {}
