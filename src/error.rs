use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CambiaError {
    message: String,
}

impl CambiaError {
    pub fn new(_message: &str) -> Self {
        CambiaError { message: _message.to_string() }
    }
}

impl fmt::Display for CambiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}