use std::{fs::OpenOptions, io::Read};

use simple_text_decode::DecodedText;

use crate::error::CambiaError;
use crate::util::first_line;
use crate::evaluate::{EvaluationCombined, Evaluator, gazelle_evaluate::ops_evaluate::OpsEvaluator, cambia_evaluate::CambiaEvaluator};
use crate::parser::{eac_parser::EacParser, ParserCombined, xld_parser::XldParser, whipper_parser::WhipperParser, ParsedLogCombined};
use crate::response::CambiaResponse;

pub fn parse_file(filepath: &str) {
    let mut raw: Vec<u8> = Vec::new();

    let mut fh = OpenOptions::new().read(true).open(filepath).expect(
        "Could not open file",
    );

    fh.read_to_end(&mut raw).expect(
        "Could not read file"
    );

    if let Ok(parsed) = parse_log_bytes(raw) {
        println!("{}", serde_json::to_string(&parsed).unwrap());
    }
}

pub fn detect_ripper(encoded_log: DecodedText) -> Result<Box<dyn ParserCombined>, CambiaError> {
    match first_line(&encoded_log.text) {
        eac if eac.contains("Exact Audio Copy") || eac.contains("EAC") => Ok(Box::new(EacParser::new(encoded_log))),
        xld if xld.contains("X Lossless Decoder version") => Ok(Box::new(XldParser::new(encoded_log))),
        whipper if whipper.contains("Log created by: whipper") => Ok(Box::new(WhipperParser::new(encoded_log))),
        _ => Err(CambiaError::new("Unsupported file."))
    }
} 

pub fn parse_log_bytes(log_raw: Vec<u8>) -> Result<CambiaResponse, CambiaError> {
    if log_raw.is_empty() {
        return Err(CambiaError::new("Empty request body"));
    }

    let encoded_log = DecodedText::new(log_raw).unwrap_or_default();

    let parsed_logs: ParsedLogCombined = match detect_ripper(encoded_log) {
        Ok(parser) => parser.parse_combined(),
        Err(e) => return Err(e),
    };

    let mut ops_evaluator: OpsEvaluator = OpsEvaluator::new();
    let ops_evaluation: EvaluationCombined = ops_evaluator.evaluate_combined(&parsed_logs);

    let mut cambia_evaluator = CambiaEvaluator::new();
    let cambia_evaluation: EvaluationCombined = cambia_evaluator.evaluate_combined(&parsed_logs);

    Ok(CambiaResponse::new(parsed_logs, vec![ops_evaluation, cambia_evaluation]))
}

pub fn translate_log_bytes(log_raw: Vec<u8>) -> Result<String, CambiaError> {
    if log_raw.is_empty() {
        return Err(CambiaError::new("Empty request body"));
    }

    let encoded_log = DecodedText::new(log_raw).unwrap_or_default();
    
    match detect_ripper(encoded_log) {
        Ok(parser) => Ok(parser.translate_combined()),
        Err(e) => Err(e),
    }
}

pub fn parse_ws_request(mut ws_body: Vec<u8>) -> Result<CambiaResponse, CambiaError> {
    // xxH64 is 8 bytes
    if ws_body.len() < 8 {
        return Err(CambiaError::new("WS message length too small"));
    }
    
    let log_bytes = ws_body.split_off(8);
    match parse_log_bytes(log_bytes) {
        Ok(mut res) => {
            res.id = ws_body;
            Ok(res)
        },
        other => other,
    }
}