use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::parser::{ParsedLogCombined, ParsedLog};
#[cfg(feature = "gazelle_ev")]
pub mod gazelle_evaluate;
// #[cfg(feature = "cambia_ev")]
// pub mod cambia_evaluate;

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
pub enum EvaluationUnitScope {
    Release,
    Track(Option<u8>),
}

#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum EvaluationUnitClass {
    Critical,
    Bad,
    Neutral,
    Good,
    Perfect,
}

// This will be used to indicate which field in the log is relevant to a EvaluationUnit
#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum EvaluationUnitField {
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
pub struct EvaluationUnitData {
    pub scope: EvaluationUnitScope,
    pub field: EvaluationUnitField,
    pub message: String,
    pub class: EvaluationUnitClass,
}

// Output from a single evaluator
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EvaluationCombined {
    pub evaluator: EvaluatorType,
    pub combined_score: String,
    pub evaluations: Vec<Evaluation>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Evaluation {
    pub score: String,
    pub evaluation_units: Vec<EvaluationUnit>,
}

#[derive(Serialize, Deserialize, TS, Hash, PartialEq, Eq, Clone)]
#[ts(export)]
pub struct EvaluationUnit {
    pub unit_score: String,
    pub data: EvaluationUnitData,
}

// All evaluators are required to implement this
pub trait Evaluator {
    fn evaluate_combined(&mut self, parsed_logs: &ParsedLogCombined) -> EvaluationCombined;
    fn evaluate(&mut self, parsed_log: &ParsedLog) -> Evaluation;
}

impl EvaluationUnitData {
    pub fn new(scope: EvaluationUnitScope, field: EvaluationUnitField, message: &str, class: EvaluationUnitClass) -> Self {
        EvaluationUnitData { scope, field, message: message.to_string(), class }
    }
}

impl EvaluationUnit {
    pub fn new(unit_score: String, data: EvaluationUnitData) -> Self {
        EvaluationUnit { unit_score, data }
    }

    pub fn new_from_u32(unit_score: u32, data: EvaluationUnitData) -> Self {
        EvaluationUnit { unit_score: unit_score.to_string(), data }
    }
}

impl Evaluation {
    pub fn new(score: String, evaluation_units: Vec<EvaluationUnit>) -> Self {
        Evaluation { score, evaluation_units }
    }

    pub fn gazelle_fail(evaluation_units: Vec<EvaluationUnit>) -> Self {
        Evaluation::new(String::from("-1"), evaluation_units)
    }
}

impl EvaluationCombined {
    pub fn new(evaluator: EvaluatorType, combined_score: String, evaluations: Vec<Evaluation>) -> Self {
        EvaluationCombined { evaluator, combined_score, evaluations }
    }
}