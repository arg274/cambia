use simple_text_decode::DecodedText;

use crate::{translate::TranslatorCombined, extract::{Ripper, Extractor, Quartet}};

use super::{eac_parser::EacParserSingle, ParsedLog, ParsedLogCombined, ParserCombined, Parser, IntegrityChecker};

use regex::Regex;

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"CUERipper v(.+) Copyright").unwrap();
}

pub struct CueRipperParser {
    encoded_log: DecodedText,
}

pub struct CueRipperParserSingle {
    log: String,
    language: String,
}

impl CueRipperParser {
    pub fn new(encoded_log: DecodedText) -> Self {
        CueRipperParser {
            encoded_log,
        }
    }
}

impl ParserCombined for CueRipperParser {
    fn parse_combined(&self) -> ParsedLogCombined {        
        let parsed_logs: Vec<ParsedLog> = vec![CueRipperParserSingle::new(self.encoded_log.text.trim().to_owned()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string()
        }
    }
}

impl TranslatorCombined for CueRipperParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl CueRipperParserSingle {
    pub fn new(log: String) -> Self {
        Self {
            log,
            language: String::from("English"),
        }
    }
}

// FIXME: Handle CUERipper's own format
impl Parser for CueRipperParserSingle {
    fn parse(&mut self) -> ParsedLog {
        let mut eac_variant = EacParserSingle::new(self.log.trim().to_string()).parse();
        eac_variant.ripper = self.extract_ripper();
        eac_variant.ripper_version = self.extract_ripper_version();
        eac_variant.checksum = self.get_checksum();
        eac_variant.id3_enabled = self.extract_id3_enabled();
        eac_variant
    }
}

impl Extractor for CueRipperParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::CueRipper
    }

    fn extract_ripper_version(&self) -> String {
        let captures = RIPPER_VERSION.captures(&self.log);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().trim_start_matches('V').to_string(),
            None => String::from("Unknown"),
        }
    }

    fn extract_language(&self) -> String {
        self.language.clone()
    }

    fn extract_id3_enabled(&self) -> crate::extract::Quartet {
        Quartet::False
    }
}

impl IntegrityChecker for CueRipperParserSingle {
    fn extract_checksum(&self) -> String {
        String::new()
    }

    fn calculate_checksum(&self) -> String {
        String::new()
    }
}