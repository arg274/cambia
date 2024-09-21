use simple_text_decode::DecodedText;
use xxhash_rust::xxh3::xxh3_64;

use crate::error::CambiaError;
use crate::util::{first_line};
use crate::evaluate::{EvaluationCombined, Evaluator};
use crate::parser::{ParserCombined, ParsedLogCombined};
use crate::response::CambiaResponse;

pub fn detect_ripper(encoded_log: DecodedText) -> Result<Box<dyn ParserCombined>, CambiaError> {
    match first_line(&encoded_log.text) {
        #[cfg(feature = "eac")]
        eac if eac.contains("Exact Audio Copy") || eac.contains("EAC") => Ok(Box::new(crate::parser::eac_parser::EacParser::new(encoded_log))),
        #[cfg(feature = "xld")]
        xld if xld.contains("X Lossless Decoder version") => Ok(Box::new(crate::parser::xld_parser::XldParser::new(encoded_log))),
        #[cfg(feature = "whipper")]
        whipper if whipper.contains("Log created by: whipper") => Ok(Box::new(crate::parser::whipper_parser::WhipperParser::new(encoded_log))),
        #[cfg(feature = "cueripper")]
        cueripper if cueripper.contains("CUERipper") => Ok(Box::new(crate::parser::cueripper_parser::CueRipperParser::new(encoded_log))),
        cyanrip if cyanrip.contains("cyanrip") => Err(CambiaError::new_anon("cyanrip not supported at the moment.")),
        dbpa if dbpa.contains("dBpoweramp Release") => Err(CambiaError::new_anon("dBpoweramp not supported at the moment.")),
        morituri if morituri.contains("Logfile created by: morituri") => Err(CambiaError::new_anon("morituri not supported at the moment.")),
        ezcd if ezcd.contains("EZ CD Audio Converter") => Err(CambiaError::new_anon("EZ CD Audio Converter not supported at the moment.")),
        rip if rip.contains("Rip ") && rip.contains(" Audio Extraction Log") => Err(CambiaError::new_anon("Rip (OS X) not supported at the moment.")),
        freac if freac.contains("Conversion #") => Err(CambiaError::new_anon("fre:ac not supported at the moment.")),
        _ => Err(CambiaError::new_anon("Unsupported file."))
    }
}

pub fn parse_log_bytes(id: Vec<u8>, log_raw: &Vec<u8>) -> Result<CambiaResponse, CambiaError> {
    if log_raw.is_empty() {
        return Err(CambiaError::new(id, "Empty request body"));
    }

    let res_id = if id.is_empty() { xxh3_64(&log_raw).to_be_bytes().to_vec() } else { id };
    let encoded_log = DecodedText::new(&log_raw).unwrap_or_default();

    tracing::debug!("Log {}: {} encoding detected ", hex::encode(&res_id), encoded_log.orig_encoding);

    let parsed_logs: ParsedLogCombined = match detect_ripper(encoded_log) {
        Ok(parser) => parser.parse_combined(),
        Err(mut e) => {
            e.id = res_id;
            return Err(e)
        },
    };

    let evaluation_combined: Vec<EvaluationCombined> = vec![
		#[cfg(feature = "ops_ev")]
        crate::evaluate::gazelle_evaluate::ops_evaluate::OpsEvaluator::new().evaluate_combined(&parsed_logs),
		// #[cfg(feature = "cambia_ev")]
		// crate::evaluate::cambia_evaluate::CambiaEvaluator::new().evaluate_combined(&parsed_logs),
    ];
    
    Ok(CambiaResponse::new(res_id, parsed_logs, evaluation_combined))
}

pub fn translate_log_bytes(log_raw: Vec<u8>) -> Result<String, CambiaError> {
    if log_raw.is_empty() {
        return Err(CambiaError::new_anon("Empty request body"));
    }

    let encoded_log = DecodedText::new(&log_raw).unwrap_or_default();
    
    match detect_ripper(encoded_log) {
        Ok(parser) => Ok(parser.translate_combined()),
        Err(e) => Err(e),
    }
}


