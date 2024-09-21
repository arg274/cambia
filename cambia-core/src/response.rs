use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{parser::ParsedLogCombined, evaluate::EvaluationCombined};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CambiaResponse {
    pub id: Vec<u8>,
    pub parsed: ParsedLogCombined,
    pub evaluation_combined: Vec<EvaluationCombined>,
}

impl CambiaResponse {
    pub fn new(id: Vec<u8>, parsed: ParsedLogCombined, evaluation_combined: Vec<EvaluationCombined>) -> Self {
        Self { id, parsed, evaluation_combined }
    }
}