use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::parser::{ParsedLogCombined, ParsedLog};
#[cfg(feature = "gazelle_ev")]
pub mod gazelle_evaluate;
#[cfg(feature = "cambia_ev")]
pub mod cambia_evaluate;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EvaluatorType {
    Cambia,
    RED,
    OPS,
}

// Ease of grouping on downstream
#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum DeductionCategory {
    Release,
    Track(Option<u8>),
}

// This will be used to indicate which field in the log is relevant to a deduction
#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum DeductionField {
    Encoding,
    RipperVersion,
    Drive,
    Ripper,
    Offset,
    Cache,
    TestAndCopy,
    Encoder,
    Checksum,
    MediaType,
    ReadMode,
    MaxRetryCount,
    AccurateStream,
    C2,
    SilentSamples,
    NullSamples,
    Gap,
    Tag,
    Gain,
    RangeSplit,
    Samples,
    SilentBlocks,
    Normalization,
    Filename,
    ReadError,
    SkipError,
    JitterGenericError,
    JitterEdgeError,
    JitterAtomError,
    DriftError,
    DroppedError,
    DuplicatedError,
    InconsistentErrorSectors,
    DamagedSector,
    Abort,
}

// This holds the reasoning for the smallest unit of evaluation
#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub struct DeductionData {
    category: DeductionCategory,
    field: DeductionField,
    message: String,
}

// Output from a single evaluator
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EvaluationCombined {
    evaluator: EvaluatorType,
    combined_score: String,
    evaluations: Vec<Evaluation>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Evaluation {
    score: String,
    deductions: Vec<Deduction>,
}

#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub struct Deduction {
    deduction_score: String,
    data: DeductionData,
}

// All evaluators are required to implement this
pub trait Evaluator {
    fn evaluate_combined(&mut self, parsed_logs: &ParsedLogCombined) -> EvaluationCombined;
    fn evaluate(&mut self, parsed_log: &ParsedLog) -> Evaluation;
}

impl DeductionData {
    pub fn new(category: DeductionCategory, field: DeductionField, message: &str) -> Self {
        DeductionData { category, field, message: message.to_string() }
    }
}

impl Deduction {
    pub fn new(deduction_score: String, data: DeductionData) -> Self {
        Deduction { deduction_score, data }
    }

    pub fn new_from_u32(deduction_score: u32, data: DeductionData) -> Self {
        Deduction { deduction_score: deduction_score.to_string(), data }
    }
}

impl Evaluation {
    pub fn new(score: String, deductions: Vec<Deduction>) -> Self {
        Evaluation { score, deductions }
    }

    pub fn gazelle_fail(deductions: Vec<Deduction>) -> Self {
        Evaluation::new(String::from("-1"), deductions)
    }
}

impl EvaluationCombined {
    pub fn new(evaluator: EvaluatorType, combined_score: String, evaluations: Vec<Evaluation>) -> Self {
        EvaluationCombined { evaluator, combined_score, evaluations }
    }
}