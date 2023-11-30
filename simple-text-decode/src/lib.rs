use std::fmt;

use chardet::detect;
use encoding_rs::Encoding;

#[derive(Debug)]
pub struct DecodedText {
    pub text: String,
    pub orig_encoding: String,
}

pub struct DecodingError;

impl DecodedText {
    pub fn new(raw: Vec<u8>) -> Result<DecodedText, DecodingError> {

        let detect_result = detect(&raw);

        if &detect_result.0.as_str() == &String::from("utf-8") {
            return Ok(DecodedText {
                text: String::from_utf8(raw).unwrap(),
                orig_encoding: detect_result.0,
            })
        }

        let coder = Encoding::for_label(&detect_result.0.as_bytes());

        if coder.is_none() {
            return Err(DecodingError);
        }

        let coder = coder.unwrap();

        let (cow, _, had_errors) = coder.decode(&raw);

        if had_errors {
            return Err(DecodingError);
        }

        let text = cow.to_string();

        Ok(DecodedText {
            text,
            orig_encoding: detect_result.0,
        })
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoding Error")
    }
}

impl Default for DecodedText {
    fn default() -> Self {
        Self { text: String::default(), orig_encoding: String::from("unknown") }
    }
}