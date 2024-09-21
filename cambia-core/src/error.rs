use std::fmt;

use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct CambiaError {
    pub id: Vec<u8>,
    pub message: String,
}

impl CambiaError {
    pub fn new(id: Vec<u8>, _message: &str) -> Self {
        CambiaError { id, message: _message.to_string() }
    }

    pub fn new_anon(_message: &str) -> Self {
        CambiaError { id: Vec::new(), message: _message.to_string() }
    }
}

impl fmt::Display for CambiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}